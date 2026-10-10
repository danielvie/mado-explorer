//! Safe generated-fixture check. Never accepts a caller path.
use mado_image_preview_service::{CpuBudget, Failure, Service};
use std::{
    fs,
    path::PathBuf,
    thread,
    time::{Duration, Instant},
};
fn main() {
    let root: PathBuf = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join(format!("p21-owned-selfcheck-{}", std::process::id()));
    fs::create_dir(&root).unwrap();
    let path = root.join("malformed.png");
    fs::write(&path, b"not an image").unwrap();
    let budget = CpuBudget::new();
    let mut service = Service::start(budget.clone()).unwrap();
    let target = service.open_target().unwrap();
    loop {
        match service.request(target, &path) {
            Ok(_) => break,
            Err(Failure::Busy) => thread::yield_now(),
            Err(e) => panic!("{e}"),
        }
    }
    let start = Instant::now();
    loop {
        match service.take_current(target) {
            Ok(Some(response)) => {
                assert_eq!(response.result.unwrap_err(), Failure::MalformedImage);
                break;
            }
            Ok(None) | Err(Failure::Busy) => {}
            Err(e) => panic!("{e}"),
        }
        assert!(start.elapsed() < Duration::from_secs(20));
        thread::sleep(Duration::from_millis(2));
    }
    service.stop();
    while service.poll_join().is_none() {
        thread::sleep(Duration::from_millis(2));
    }
    drop(service);
    assert_eq!(budget.used(), 0);
    assert_eq!(fs::read(&path).unwrap(), b"not an image");
    fs::remove_file(path).unwrap();
    fs::remove_dir(root).unwrap();
    println!("Owned malformed fixture, helper exit/reap and shared-budget release passed.");
}
