//! Generated memory only. Never opens or reads/writes the interactive clipboard.
#[cfg(windows)]
fn main() {
    use mado_file_clipboard::{FilePacket, Intent, NativePathBuf};
    use std::os::windows::ffi::OsStrExt;
    use std::ptr::{copy_nonoverlapping, null_mut};
    use windows_sys::Win32::Foundation::{GlobalFree, HGLOBAL};
    use windows_sys::Win32::System::Memory::{
        GMEM_MOVEABLE, GMEM_ZEROINIT, GlobalAlloc, GlobalLock, GlobalUnlock,
    };
    use windows_sys::Win32::UI::Shell::DragQueryFileW;

    struct Owned(HGLOBAL);
    impl Drop for Owned {
        fn drop(&mut self) {
            assert!(unsafe { GlobalFree(self.0) }.is_null());
        }
    }
    let paths = [
        NativePathBuf::from("C:\\generated\\données-漢字.txt"),
        NativePathBuf::from("\\\\fixture-server\\fixture-share\\entry"),
    ];
    let packet = FilePacket::snapshot(&paths, Intent::Copy).unwrap();
    let bytes = packet.encode_hdrop().unwrap();
    let memory = Owned(unsafe { GlobalAlloc(GMEM_MOVEABLE | GMEM_ZEROINIT, bytes.len()) });
    assert!(!memory.0.is_null());
    let pointer = unsafe { GlobalLock(memory.0) };
    assert!(!pointer.is_null());
    unsafe {
        copy_nonoverlapping(bytes.as_ptr(), pointer.cast::<u8>(), bytes.len());
        windows_sys::Win32::Foundation::SetLastError(0);
        let still_locked = GlobalUnlock(memory.0);
        assert!(still_locked != 0 || windows_sys::Win32::Foundation::GetLastError() == 0);
    }
    assert_eq!(
        unsafe { DragQueryFileW(memory.0, u32::MAX, null_mut(), 0) },
        paths.len() as u32
    );
    for (index, path) in paths.iter().enumerate() {
        let expected: Vec<u16> = path.as_os_str().encode_wide().collect();
        let mut actual = vec![0; expected.len() + 1];
        let length = unsafe {
            DragQueryFileW(
                memory.0,
                index as u32,
                actual.as_mut_ptr(),
                actual.len() as u32,
            )
        };
        assert_eq!(length as usize, expected.len());
        assert_eq!(&actual[..expected.len()], expected);
        assert_eq!(actual[expected.len()], 0);
    }
    println!("Owned HDROP parsed by Windows Shell; 2 generated paths. No global clipboard access.");
}

#[cfg(not(windows))]
fn main() {
    panic!("Windows-only owned HDROP probe");
}
