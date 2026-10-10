use crate::{Mode, Proof};
use mado_file_clipboard::{FilePacket, Intent};
use std::{
    mem::size_of,
    ptr::{null, null_mut},
};
use windows_sys::Win32::{
    Foundation::*,
    Security::{Authorization::*, *},
    System::{LibraryLoader::GetModuleHandleW, StationsAndDesktops::*, Threading::*},
    UI::WindowsAndMessaging::*,
};

const STATION_RIGHTS: u32 = (WINSTA_READATTRIBUTES
    | WINSTA_ACCESSCLIPBOARD
    | WINSTA_CREATEDESKTOP
    | WINSTA_ACCESSGLOBALATOMS) as u32;
const DESKTOP_RIGHTS: u32 = DESKTOP_CREATEWINDOW | DESKTOP_READOBJECTS | DESKTOP_WRITEOBJECTS;
type Result<T> = std::result::Result<T, &'static str>;
fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain([0]).collect()
}
fn record(api: &str, raw: usize, error: u32) {
    println!("stage={api} raw=0x{raw:x} GetLastError={error}");
}
fn check(api: &'static str, f: impl FnOnce() -> i32) -> Result<()> {
    unsafe {
        SetLastError(0);
    }
    let raw = f();
    let error = unsafe { GetLastError() };
    record(api, raw as usize, error);
    if raw == 0 { Err(api) } else { Ok(()) }
}
fn name(handle: HANDLE) -> Result<String> {
    let mut units = [0u16; 256];
    let mut needed = 0;
    check("GetUserObjectInformationW/UOI_NAME", || unsafe {
        GetUserObjectInformationW(
            handle,
            UOI_NAME,
            units.as_mut_ptr().cast(),
            size_of_val(&units) as u32,
            &mut needed,
        )
    })?;
    if needed < 2 || needed as usize > size_of_val(&units) || needed % 2 != 0 {
        return Err("UOI_NAME length");
    }
    let len = needed as usize / 2;
    if units[len - 1] != 0 || units[..len - 1].contains(&0) {
        return Err("UOI_NAME terminator");
    }
    String::from_utf16(&units[..len - 1]).map_err(|_| "UOI_NAME encoding")
}
fn flags(handle: HANDLE) -> Result<u32> {
    let mut value = USEROBJECTFLAGS::default();
    let mut needed = 0;
    check("GetUserObjectInformationW/UOI_FLAGS", || unsafe {
        GetUserObjectInformationW(
            handle,
            UOI_FLAGS,
            (&mut value as *mut USEROBJECTFLAGS).cast(),
            size_of::<USEROBJECTFLAGS>() as u32,
            &mut needed,
        )
    })?;
    if needed != size_of::<USEROBJECTFLAGS>() as u32 || value.fInherit != 0 {
        return Err("UOI_FLAGS size/inheritance");
    }
    Ok(value.dwFlags)
}

struct Descriptor(PSECURITY_DESCRIPTOR);
impl Drop for Descriptor {
    fn drop(&mut self) {
        unsafe {
            LocalFree(self.0);
        }
    }
}
impl Descriptor {
    fn own(mask: u32) -> Result<Self> {
        // Own primary token only, QUERY only. No privilege adjustment/impersonation.
        let mut token = null_mut();
        check("OpenProcessToken/TOKEN_QUERY", || unsafe {
            OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token)
        })?;
        // usize storage supplies TOKEN_USER alignment; identity query stays bounded.
        let mut storage = [0usize; 512];
        let mut needed = 0;
        let query = check("GetTokenInformation/TokenUser", || unsafe {
            GetTokenInformation(
                token,
                TokenUser,
                storage.as_mut_ptr().cast(),
                size_of_val(&storage) as u32,
                &mut needed,
            )
        });
        let close = check("CloseHandle/own-token", || unsafe { CloseHandle(token) });
        query?;
        close?;
        if needed < size_of::<TOKEN_USER>() as u32 || needed as usize > size_of_val(&storage) {
            return Err("TokenUser length");
        }
        let user = unsafe { &*storage.as_ptr().cast::<TOKEN_USER>() };
        let mut sid_text = null_mut();
        check("ConvertSidToStringSidW/own-user", || unsafe {
            ConvertSidToStringSidW(user.User.Sid, &mut sid_text)
        })?;
        let sid_result = (|| {
            let mut len = 0;
            while len < 184 && unsafe { *sid_text.add(len) } != 0 {
                len += 1;
            }
            if len == 184 {
                return Err("SID length");
            }
            String::from_utf16(unsafe { std::slice::from_raw_parts(sid_text, len) })
                .map_err(|_| "SID encoding")
        })();
        unsafe {
            LocalFree(sid_text.cast());
        }
        let sid = sid_result?;
        println!("own_identity_sid={sid} token_access=0x{TOKEN_QUERY:x}");
        // Protected, non-inheritable DACL, exact object-specific rights, only user+SYSTEM.
        // Applied to NEW objects only. No default descriptor, GA, existing ACL edit or SACL.
        let sddl = wide(&format!("D:P(A;;0x{mask:x};;;{sid})(A;;0x{mask:x};;;SY)"));
        let mut sd = null_mut();
        check(
            "ConvertStringSecurityDescriptorToSecurityDescriptorW",
            || unsafe {
                ConvertStringSecurityDescriptorToSecurityDescriptorW(
                    sddl.as_ptr(),
                    1,
                    &mut sd,
                    null_mut(),
                )
            },
        )?;
        Ok(Self(sd))
    }
    fn attributes(&self) -> SECURITY_ATTRIBUTES {
        SECURITY_ATTRIBUTES {
            nLength: size_of::<SECURITY_ATTRIBUTES>() as u32,
            lpSecurityDescriptor: self.0,
            bInheritHandle: 0,
        }
    }
}

struct Owned {
    original_station: HWINSTA, // borrowed, restoration only; never queried/closed
    original_desktop: HDESK,
    station: HWINSTA,
    desktop: HDESK,
    window: HWND,
    class: Vec<u16>,
    class_atom: u16,
    instance: HINSTANCE,
    station_name: String,
    desktop_name: String,
    created_only: bool,
    private_dacl: bool,
    station_assigned: bool,
    desktop_assigned: bool,
}
impl Owned {
    fn new() -> Self {
        unsafe {
            Self {
                original_station: GetProcessWindowStation(),
                original_desktop: GetThreadDesktop(GetCurrentThreadId()),
                station: null_mut(),
                desktop: null_mut(),
                window: null_mut(),
                class: wide("P22PrivateClipboardOwner"),
                class_atom: 0,
                instance: GetModuleHandleW(null()),
                station_name: String::new(),
                desktop_name: String::new(),
                created_only: false,
                private_dacl: false,
                station_assigned: false,
                desktop_assigned: false,
            }
        }
    }
    fn setup(&mut self) -> Result<()> {
        if self.original_station.is_null()
            || self.original_desktop.is_null()
            || self.instance.is_null()
        {
            return Err("borrowed context/module");
        }
        let sd = Descriptor::own(STATION_RIGHTS)?;
        println!(
            "station_name_argument=NULL station_flags=0x{CWF_CREATE_ONLY:x} station_access=0x{STATION_RIGHTS:x} protected_dacl=current_user+SYSTEM"
        );
        unsafe {
            SetLastError(0);
        }
        self.station = unsafe {
            CreateWindowStationW(null(), CWF_CREATE_ONLY, STATION_RIGHTS, &sd.attributes())
        };
        let error = unsafe { GetLastError() };
        record("CreateWindowStationW", self.station as usize, error);
        if self.station.is_null() || error == ERROR_ALREADY_EXISTS {
            return Err("CreateWindowStationW/new-only");
        }
        self.created_only = true;
        self.private_dacl = true;
        self.station_name = name(self.station)?;
        let station_flags = flags(self.station)?;
        println!(
            "owned_station={} station_uoi_flags=0x{station_flags:x}",
            self.station_name
        );
        if self.station_name.is_empty()
            || self.station_name.eq_ignore_ascii_case("WinSta0")
            || station_flags & WSF_VISIBLE as u32 != 0
        {
            return Err("station isolation");
        }
        check("SetProcessWindowStation/private", || unsafe {
            SetProcessWindowStation(self.station)
        })?;
        self.station_assigned = true;
        if unsafe { GetProcessWindowStation() } != self.station
            || name(unsafe { GetProcessWindowStation() })? != self.station_name
            || flags(unsafe { GetProcessWindowStation() })? != station_flags
        {
            return Err("assigned station identity");
        }

        let sd = Descriptor::own(DESKTOP_RIGHTS)?;
        self.desktop_name = format!("p22-{}", unsafe { GetCurrentProcessId() });
        let desktop_name = wide(&self.desktop_name);
        println!(
            "desktop_name={} desktop_flags=0 desktop_access=0x{DESKTOP_RIGHTS:x}",
            self.desktop_name
        );
        unsafe {
            SetLastError(0);
        }
        // Brand-new empty station; no desktop-open/reuse path. No switch/input/hook rights.
        self.desktop = unsafe {
            CreateDesktopW(
                desktop_name.as_ptr(),
                null(),
                null(),
                0,
                DESKTOP_RIGHTS,
                &sd.attributes(),
            )
        };
        let error = unsafe { GetLastError() };
        record("CreateDesktopW", self.desktop as usize, error);
        if self.desktop.is_null() || error == ERROR_ALREADY_EXISTS {
            return Err("CreateDesktopW/new-only");
        }
        check("SetThreadDesktop/private", || unsafe {
            SetThreadDesktop(self.desktop)
        })?;
        self.desktop_assigned = true;
        if unsafe { GetThreadDesktop(GetCurrentThreadId()) } != self.desktop
            || name(self.desktop)? != self.desktop_name
        {
            return Err("assigned desktop identity");
        }
        println!(
            "owned_desktop={} desktop_uoi_flags=0x{:x}",
            name(self.desktop)?,
            flags(self.desktop)?
        );
        let wc = WNDCLASSW {
            lpfnWndProc: Some(window_proc),
            hInstance: self.instance,
            lpszClassName: self.class.as_ptr(),
            ..Default::default()
        };
        unsafe {
            SetLastError(0);
        }
        self.class_atom = unsafe { RegisterClassW(&wc) };
        let error = unsafe { GetLastError() };
        record("RegisterClassW", self.class_atom as usize, error);
        if self.class_atom == 0 {
            return Err("RegisterClassW");
        }
        unsafe {
            SetLastError(0);
        }
        // No WS_VISIBLE, parent, menu, hooks, message loop or application startup.
        self.window = unsafe {
            CreateWindowExW(
                0,
                self.class.as_ptr(),
                wide("P22 owned hidden").as_ptr(),
                WS_POPUP,
                0,
                0,
                1,
                1,
                null_mut(),
                null_mut(),
                self.instance,
                null(),
            )
        };
        let error = unsafe { GetLastError() };
        record("CreateWindowExW/hidden", self.window as usize, error);
        if self.window.is_null() {
            return Err("CreateWindowExW");
        }
        self.guard()
    }
    fn guard(&self) -> Result<()> {
        let current_station = unsafe { GetProcessWindowStation() };
        let tid = unsafe { GetCurrentThreadId() };
        let current_desktop = unsafe { GetThreadDesktop(tid) };
        let mut window_pid = 0;
        let window_tid = unsafe { GetWindowThreadProcessId(self.window, &mut window_pid) };
        let proof = Proof {
            created_only: self.created_only,
            private_dacl: self.private_dacl,
            station: name(self.station)?,
            current_station: name(current_station)?,
            station_handles_match: current_station == self.station,
            flags: flags(self.station)?,
            current_flags: flags(current_station)?,
            desktop: name(self.desktop)?,
            current_desktop: name(current_desktop)?,
            desktop_handles_match: current_desktop == self.desktop,
            window_live: unsafe { IsWindow(self.window) } != 0,
            window_hidden: unsafe { IsWindowVisible(self.window) } == 0,
            window_pid,
            pid: unsafe { GetCurrentProcessId() },
            window_tid,
            tid,
            window_desktop_matches: window_tid != 0
                && unsafe { GetThreadDesktop(window_tid) } == self.desktop,
        };
        println!(
            "stage=isolation_guard station={} current_station={} station_flags=0x{:x} current_flags=0x{:x} desktop={} window_pid={} window_tid={} allowed={}",
            proof.station,
            proof.current_station,
            proof.flags,
            proof.current_flags,
            proof.desktop,
            proof.window_pid,
            proof.window_tid,
            proof.allowed()
        );
        if proof.station != self.station_name
            || proof.desktop != self.desktop_name
            || !proof.allowed()
        {
            Err("isolation guard")
        } else {
            Ok(())
        }
    }
    fn cleanup(&mut self) -> Result<()> {
        // Stop at any failure. Never close a context still assigned or destroy an
        // HWND after restoring to the borrowed interactive context. Process exit
        // releases remaining private resources, with failure explicitly reported.
        if !self.window.is_null() {
            check("DestroyWindow/owned", || unsafe {
                DestroyWindow(self.window)
            })?;
            self.window = null_mut();
        }
        if self.class_atom != 0 {
            check("UnregisterClassW/owned", || unsafe {
                UnregisterClassW(self.class.as_ptr(), self.instance)
            })?;
            self.class_atom = 0;
        }
        // Creation APIs can associate contexts: inspect handles, not only our Set flags.
        if self.desktop_assigned
            || unsafe { GetThreadDesktop(GetCurrentThreadId()) } == self.desktop
        {
            check("SetThreadDesktop/restore-borrowed", || unsafe {
                SetThreadDesktop(self.original_desktop)
            })?;
            self.desktop_assigned = false;
        }
        if self.station_assigned || unsafe { GetProcessWindowStation() } == self.station {
            check("SetProcessWindowStation/restore-borrowed", || unsafe {
                SetProcessWindowStation(self.original_station)
            })?;
            self.station_assigned = false;
        }
        if !self.desktop.is_null() {
            if unsafe { GetThreadDesktop(GetCurrentThreadId()) } == self.desktop {
                return Err("cleanup desktop still assigned");
            }
            check("CloseDesktop/owned", || unsafe {
                CloseDesktop(self.desktop)
            })?;
            self.desktop = null_mut();
        }
        if !self.station.is_null() {
            if unsafe { GetProcessWindowStation() } == self.station {
                return Err("cleanup station still assigned");
            }
            check("CloseWindowStation/owned", || unsafe {
                CloseWindowStation(self.station)
            })?;
            self.station = null_mut();
        }
        println!("stage=cleanup outcome=complete");
        Ok(())
    }
}
unsafe extern "system" fn window_proc(hwnd: HWND, msg: u32, wp: WPARAM, lp: LPARAM) -> LRESULT {
    unsafe { DefWindowProcW(hwnd, msg, wp, lp) }
}

pub fn run(mode: Mode) -> Result<()> {
    println!(
        "mode={mode:?} own_pid={} station_access=0x{STATION_RIGHTS:x} desktop_access=0x{DESKTOP_RIGHTS:x}",
        unsafe { GetCurrentProcessId() }
    );
    let mut owned = Owned::new();
    let mut writes = 0;
    let mut reads = 0;
    let result = (|| {
        owned.setup()?;
        if mode == Mode::Roundtrip {
            // Only an explicitly reviewed future invocation may enter this branch.
            let fixtures = Fixtures::create()?;
            for intent in [Intent::Copy, Intent::Move] {
                let packet =
                    FilePacket::snapshot(&fixtures.paths, intent).map_err(|_| "fixture packet")?;
                owned.guard()?;
                writes += 1;
                mado_file_clipboard::write_clipboard(owned.window, &packet).map_err(|e| {
                    println!("stage=backend_write error={e}");
                    "backend write"
                })?;
                owned.guard()?;
                reads += 1;
                let returned = mado_file_clipboard::read_clipboard().map_err(|e| {
                    println!("stage=backend_read error={e}");
                    "backend read"
                })?;
                if packet != returned {
                    return Err("packet roundtrip mismatch");
                }
                println!("stage=roundtrip intent={intent:?} outcome=matched");
            }
            fixtures.cleanup()?;
        }
        Ok(())
    })();
    if let Err(stage) = result {
        println!("outcome=blocked failed_stage={stage}");
    }
    let cleanup = owned.cleanup();
    if let Err(stage) = cleanup {
        println!(
            "outcome=blocked cleanup_failed_stage={stage} remaining_private_handles=process_exit"
        );
    }
    println!("backend_write_calls={writes} backend_read_calls={reads}");
    if writes == 0 && reads == 0 {
        println!("clipboard_calls=0");
    } else {
        println!(
            "clipboard_native_call_count=not_instrumented backend_calls={}",
            writes + reads
        );
    }
    result?;
    cleanup?;
    println!(
        "outcome=setup-verified data_phase={}",
        if mode == Mode::SetupOnly {
            "not-run"
        } else {
            "private-packet-matched"
        }
    );
    Ok(())
}

// Stage 2 only. No filesystem mutation on setup-only or in default tests.
struct Fixtures {
    root_handle: std::fs::File,
    files: Vec<std::fs::File>,
    paths: Vec<std::path::PathBuf>,
}
fn non_reparse(path: &std::path::Path, directory: bool) -> Result<()> {
    use std::os::windows::fs::MetadataExt;
    let meta = std::fs::symlink_metadata(path).map_err(|_| "fixture metadata")?;
    if meta.file_attributes() & 0x400 != 0 || meta.is_dir() != directory {
        return Err("fixture reparse/type");
    }
    Ok(())
}
impl Fixtures {
    fn create() -> Result<Self> {
        use std::io::Write;
        let base = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/p22");
        // Inspect each existing ancestor, never create beneath an alias/reparse point.
        let base = std::path::absolute(base).map_err(|_| "fixture base")?;
        for ancestor in base.ancestors() {
            non_reparse(ancestor, true)?;
        }
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|_| "fixture clock")?
            .as_nanos();
        let root = base.join(format!("fixture-{}-{nonce}", unsafe {
            GetCurrentProcessId()
        }));
        std::fs::create_dir(&root).map_err(|_| "fixture create-new directory")?;
        use std::os::windows::fs::{MetadataExt, OpenOptionsExt};
        use windows_sys::Win32::Storage::FileSystem::{
            DELETE, FILE_FLAG_BACKUP_SEMANTICS, FILE_FLAG_OPEN_REPARSE_POINT, FILE_READ_ATTRIBUTES,
        };
        let root_handle = std::fs::OpenOptions::new()
            .access_mode(DELETE | FILE_READ_ATTRIBUTES)
            .share_mode(0)
            .custom_flags(FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_OPEN_REPARSE_POINT)
            .open(&root)
            .map_err(|_| "fixture own-directory handle; retained leftovers")?;
        if root_handle
            .metadata()
            .map_err(|_| "fixture handle metadata")?
            .file_attributes()
            & 0x400
            != 0
        {
            return Err("fixture own-directory reparse");
        }
        let paths = vec![root.join("é-𐐀.txt")];
        let mut files = Vec::new();
        for path in [root.join("P22.owned"), paths[0].clone()] {
            let mut file = std::fs::OpenOptions::new()
                .access_mode(GENERIC_WRITE | DELETE)
                .share_mode(0)
                .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT)
                .create_new(true)
                .open(path)
                .map_err(|_| "fixture create-new file; retained owned leftovers")?;
            file.write_all(b"P22 generated private packet fixture\n")
                .map_err(|_| "fixture file write; retained owned leftovers")?;
            files.push(file);
        }
        println!(
            "stage=fixture created_files=2 generated_directory={}",
            root.file_name().unwrap().to_string_lossy()
        );
        Ok(Self {
            root_handle,
            files,
            paths,
        })
    }
    fn cleanup(self) -> Result<()> {
        use std::os::windows::io::AsRawHandle;
        use windows_sys::Win32::Storage::FileSystem::{
            FILE_DISPOSITION_INFO, FileDispositionInfo, SetFileInformationByHandle,
        };
        // Only original retained create-new handles are deleted, never reopened
        // paths, recursive removal or marker-based claims on existing files.
        // A preflight ancestor check is not an adversarial filesystem sandbox.
        let remove = |file: &std::fs::File| {
            let info = FILE_DISPOSITION_INFO { DeleteFile: true };
            check("SetFileInformationByHandle/own-fixture-delete", || unsafe {
                SetFileInformationByHandle(
                    file.as_raw_handle(),
                    FileDispositionInfo,
                    (&info as *const FILE_DISPOSITION_INFO).cast(),
                    size_of::<FILE_DISPOSITION_INFO>() as u32,
                )
            })
        };
        for file in &self.files {
            remove(file)?;
        }
        drop(self.files);
        remove(&self.root_handle)?;
        drop(self.root_handle);
        println!("stage=fixture cleanup=original-handles-only");
        Ok(())
    }
}
