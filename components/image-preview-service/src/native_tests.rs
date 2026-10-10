//! Access to native private fields only for bounded owned process probes.
use super::*;
use crate::tests::SERIAL;
use mado_pdf_preview_service::CpuBudget;
#[test]
fn setup_failure_keeps_primary_suspended_contained_and_reaps_before_refund() {
    let _serial = SERIAL.lock().unwrap();
    let budget = CpuBudget::new();
    let _control = budget.reserve_capacity(crate::CONTROL_CAP).unwrap();
    let mut exe = std::env::current_exe().unwrap();
    exe.pop();
    exe.pop();
    exe.push("mado-image-helper.exe");
    let mut retained = None;
    let result = Child::launch(
        &exe,
        &budget,
        || true,
        |c, thread| {
            assert!(c.before_resume_verified);
            // Actual kernel suspend count is 1 before the first ResumeThread. The
            // extra Suspend/Resume pair observes it without executing helper code.
            assert_eq!(unsafe { SuspendThread(thread) }, 1);
            assert_eq!(unsafe { ResumeThread(thread) }, 2);
            let mut duplicated = ptr::null_mut();
            assert_ne!(
                unsafe {
                    DuplicateHandle(
                        GetCurrentProcess(),
                        c.process.0,
                        GetCurrentProcess(),
                        &mut duplicated,
                        0,
                        0,
                        DUPLICATE_SAME_ACCESS,
                    )
                },
                0
            );
            retained = Some(Handle(duplicated));
            assert_eq!(
                budget.used(),
                crate::CONTROL_CAP + HELPER_CAP + crate::OUTPUT_CAP
            );
            Err(Failure::SetupFailure(995))
        },
    );
    assert!(matches!(result, Err(Failure::SetupFailure(995))));
    let retained = retained.unwrap();
    assert_eq!(unsafe { WaitForSingleObject(retained.0, 0) }, WAIT_OBJECT_0);
    let mut code = 0;
    assert_ne!(unsafe { GetExitCodeProcess(retained.0, &mut code) }, 0);
    assert_eq!(code, 995);
    assert_eq!(budget.used(), crate::CONTROL_CAP);
    eprintln!(
        "Injected setup failure: actual primary suspend count 1, own Job membership verified before run, terminated process signaled/995 before helper refund."
    );
}
#[test]
fn nonallowlisted_inheritable_private_event_does_not_reach_child() {
    let _serial = SERIAL.lock().unwrap();
    let budget = CpuBudget::new();
    let _control = budget.reserve_capacity(crate::CONTROL_CAP).unwrap();
    let sa = SECURITY_ATTRIBUTES {
        nLength: size_of::<SECURITY_ATTRIBUTES>() as u32,
        bInheritHandle: 1,
        lpSecurityDescriptor: ptr::null_mut(),
    };
    let sentinel = unsafe { Handle::checked(CreateEventW(&sa, 1, 0, ptr::null())) }.unwrap();
    let mut exe = std::env::current_exe().unwrap();
    exe.pop();
    exe.pop();
    exe.push("mado-image-helper.exe");
    let mut c = Child::launch(&exe, &budget, || true, |_, _| Ok(())).unwrap();
    let wake = Handle::event().unwrap();
    let request = crate::Request {
        target: crate::Target(1),
        generation: crate::Generation(2),
        path: Vec::new(),
        probe_mode: 0,
    };
    let mut wire = crate::protocol::request(1, 2, &[], 4, sentinel.0 as u64);
    c.exact(&mut wire, true, &wake, || true).unwrap();
    let frame = crate::receive(&mut c, &request, &wake, || true).unwrap();
    // Child-local handle slot reuse is not inheritance. If this exact parent
    // event were inherited, the child's SetEvent would signal this kernel object.
    assert!([0u32, 1].contains(&u32::from_le_bytes(frame.pixels().try_into().unwrap())));
    assert_eq!(unsafe { WaitForSingleObject(sentinel.0, 0) }, WAIT_TIMEOUT);
    drop(c);
    drop(frame);
    assert_eq!(budget.used(), crate::CONTROL_CAP);
    eprintln!(
        "Explicit inherit allowlist: child SetEvent probe cannot signal the parent inheritable private event; only private IPC handle allowed including std handles."
    );
}
