//! OUR helper binary. No GUI/settings/font or parent decoder startup.
#![cfg(windows)]
mod decoder;
#[path = "../protocol.rs"]
mod protocol;
#[path = "../source.rs"]
mod source;
use decoder::{DecodeLimits, ErrorKind, Format};
use protocol::*;
use std::{
    ffi::OsString,
    fs::File,
    io::{Read, Write},
    os::windows::{ffi::OsStringExt, io::FromRawHandle},
    path::PathBuf,
};
fn main() {
    // Suppress critical-error/crash dialogs in OUR process, including real codec
    // failures. The parent reports the bounded failure instead of starting UI.
    use windows_sys::Win32::System::Diagnostics::Debug::{
        SEM_FAILCRITICALERRORS, SEM_NOGPFAULTERRORBOX, SetErrorMode,
    };
    unsafe {
        SetErrorMode(SEM_FAILCRITICALERRORS | SEM_NOGPFAULTERRORBOX);
    }
    // Do not expose an interactive arbitrary-path decoder. Only a parent-created
    // inherited private IPC handle is accepted. No source path in argv/env.
    let mut args = std::env::args_os();
    args.next();
    if args.next() != Some(OsString::from("--ipc")) {
        std::process::exit(64);
    }
    let handle = args
        .next()
        .and_then(|s| s.to_str().and_then(|s| s.parse::<usize>().ok()))
        .unwrap_or(0);
    if handle == 0 || args.next().is_some() {
        std::process::exit(64);
    }
    let mut contained = 0;
    if unsafe {
        windows_sys::Win32::System::JobObjects::IsProcessInJob(
            windows_sys::Win32::System::Threading::GetCurrentProcess(),
            std::ptr::null_mut(),
            &mut contained,
        )
    } == 0
        || contained == 0
    {
        std::process::exit(64);
    }
    // SAFETY: this is the single parent-allowlisted inherited handle. Ownership
    // transfers to File; invalid handles fail read without access to a path.
    let mut ipc = unsafe { File::from_raw_handle(handle as _) };
    if serve(&mut ipc).is_err() {
        std::process::exit(65);
    }
}
fn serve(ipc: &mut File) -> std::io::Result<()> {
    let mut request = [0; REQUEST_SIZE];
    ipc.read_exact(&mut request)?;
    let units = get32(&request, 24) as usize;
    let target = get64(&request, 8);
    let generation = get64(&request, 16);
    if &request[..4] != b"MIQ1"
        || get32(&request, 4) != VERSION
        || target == 0
        || generation == 0
        || units > PATH_UNITS
    {
        return Err(std::io::ErrorKind::InvalidData.into());
    }
    let mode = get32(&request, 28);
    let aux = get64(&request, 32);
    let mut bytes = vec![0; units * 2];
    ipc.read_exact(&mut bytes)?;
    let path: Vec<u16> = bytes
        .chunks_exact(2)
        .map(|b| u16::from_le_bytes([b[0], b[1]]))
        .collect();
    drop(bytes);
    if mode != 0 {
        return probe(ipc, target, generation, mode, aux);
    }
    if aux != 0 || source::admit(&path).is_err() {
        return error(
            ipc,
            target,
            generation,
            3,
            b"invalid native filesystem path",
        );
    }
    let file = match source::open(&path) {
        Ok(f) => f,
        Err(_) => {
            return error(
                ipc,
                target,
                generation,
                3,
                b"read-only regular file unavailable",
            );
        }
    };
    let path = PathBuf::from(OsString::from_wide(&path));
    let limits = DecodeLimits {
        max_input_bytes: INPUT_CAP,
        max_width: DIMENSION_CAP,
        max_height: DIMENSION_CAP,
        max_output_bytes: OUTPUT_CAP as u64,
        max_buffer_bytes: HELPER_CAP as u64,
        max_decoder_bytes: HELPER_CAP as u64,
    };
    match decoder::decode(file, &path, limits) {
        Ok(preview) => {
            if rgba_size(preview.width, preview.height).ok() != Some(preview.rgba.len()) {
                return Err(std::io::ErrorKind::InvalidData.into());
            }
            let h = Header {
                target,
                generation,
                status: 0,
                format: match preview.metadata.format {
                    Format::Png => 1,
                    Format::Jpeg => 2,
                    Format::WebP => 3,
                },
                orientation: preview.metadata.orientation as u32,
                encoded_width: preview.metadata.encoded_width,
                encoded_height: preview.metadata.encoded_height,
                width: preview.width,
                height: preview.height,
                error_len: 0,
                file_bytes: preview.metadata.file_bytes,
                payload: preview.rgba.len() as u64,
            };
            ipc.write_all(&h.bytes())?;
            for chunk in preview.rgba.chunks(IO_CHUNK) {
                ipc.write_all(chunk)?;
            }
        }
        Err(e) => {
            let (status, message): (_, &[u8]) = match e.kind {
                ErrorKind::Unsupported => (1, b"unsupported image"),
                ErrorKind::Malformed(_) => (2, b"malformed image"),
                ErrorKind::Unavailable(_) => (3, b"file unavailable"),
                ErrorKind::Oversize(_) => (4, b"image exceeds admission limits"),
                ErrorKind::AllocationFailed => (5, b"allocation failed"),
            };
            error(ipc, target, generation, status, message)?;
        }
    }
    Ok(())
}
fn error(
    ipc: &mut File,
    target: u64,
    generation: u64,
    status: u32,
    message: &[u8],
) -> std::io::Result<()> {
    assert!(message.len() <= ERROR_CAP);
    ipc.write_all(&Header::error(target, generation, status, message.len()).bytes())?;
    ipc.write_all(message)
}
// Bounded diagnostics of OUR binary, driven only by component-owned tests. These
// do not open sources or launch another executable, and allocate at most 144 MiB.
fn probe(ipc: &mut File, target: u64, generation: u64, mode: u32, aux: u64) -> std::io::Result<()> {
    use windows_sys::Win32::{
        Foundation::*,
        System::{Diagnostics::Debug::*, JobObjects::*, Memory::*, Threading::*},
    };
    let mut payload = [0u8; 4];
    let h = Header {
        target,
        generation,
        status: 0,
        format: 1,
        orientation: 1,
        encoded_width: 1,
        encoded_height: 1,
        width: 1,
        height: 1,
        error_len: 0,
        file_bytes: 1,
        payload: 4,
    };
    match mode {
        1 => unsafe {
            let mut contained = 0;
            if IsProcessInJob(GetCurrentProcess(), std::ptr::null_mut(), &mut contained) == 0
                || contained == 0
            {
                return Err(std::io::ErrorKind::Other.into());
            }
            let small = VirtualAlloc(
                std::ptr::null(),
                64 * 1024 * 1024,
                MEM_RESERVE | MEM_COMMIT,
                PAGE_READWRITE,
            );
            if small.is_null() {
                return Err(std::io::Error::last_os_error());
            }
            // Touch generated anonymous pages, then free before the capped attempt.
            std::ptr::write_volatile(small as *mut u8, 1);
            VirtualFree(small, 0, MEM_RELEASE);
            let large = VirtualAlloc(
                std::ptr::null(),
                144 * 1024 * 1024,
                MEM_RESERVE | MEM_COMMIT,
                PAGE_READWRITE,
            );
            let code = GetLastError();
            if !large.is_null() {
                VirtualFree(large, 0, MEM_RELEASE);
                return Err(std::io::ErrorKind::Other.into());
            }
            payload = code.to_le_bytes();
        },
        2 => unsafe {
            SetErrorMode(SEM_FAILCRITICALERRORS | SEM_NOGPFAULTERRORBOX);
            RaiseException(0xc0000005, 1, 0, std::ptr::null());
            return Err(std::io::ErrorKind::Other.into());
        },
        3 => unsafe {
            Sleep(INFINITE);
            return Ok(());
        },
        4 => unsafe {
            // Numeric handle slots can be reused in the child. Testing whether
            // the SAME parent event becomes signaled proves kernel-object identity.
            payload = u32::from(SetEvent(aux as _) != 0).to_le_bytes();
        },
        5 => {
            let mut bad = h.bytes();
            put64(&mut bad, 64, u64::MAX);
            return ipc.write_all(&bad);
        }
        6 => {
            ipc.write_all(&h.bytes())?;
            return ipc.write_all(&[0, 1]);
        }
        7 => {
            ipc.write_all(&h.bytes())?;
            ipc.write_all(&payload)?;
            return ipc.write_all(&[9]);
        }
        8 => {
            let mut bad = h.bytes();
            put64(&mut bad, 16, generation + 1);
            return ipc.write_all(&bad);
        }
        9 => {
            ipc.write_all(&h.bytes())?;
            ipc.write_all(&payload)?;
            // Owned fault probe: EOF is not process exit. This helper never
            // returns or drops File after closing it; the Job kills the sleeper.
            use std::os::windows::io::AsRawHandle;
            unsafe {
                CloseHandle(ipc.as_raw_handle());
                Sleep(INFINITE);
            }
            return Ok(());
        }
        10 => {
            unsafe {
                SetErrorMode(SEM_FAILCRITICALERRORS | SEM_NOGPFAULTERRORBOX);
            }
            // Real Rust allocator failure inside the hard-capped child. One
            // 144MiB attempt only, never a growing/multi-GiB OOM workload.
            let layout = std::alloc::Layout::from_size_align(144 * 1024 * 1024, 1).unwrap();
            let allocation = unsafe { std::alloc::alloc(layout) };
            if allocation.is_null() {
                std::alloc::handle_alloc_error(layout);
            }
            unsafe {
                std::alloc::dealloc(allocation, layout);
            }
            return Err(std::io::ErrorKind::Other.into());
        }
        _ => return Err(std::io::ErrorKind::InvalidData.into()),
    }
    ipc.write_all(&h.bytes())?;
    ipc.write_all(&payload)
}
