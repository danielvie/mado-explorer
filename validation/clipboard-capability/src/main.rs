#![cfg(windows)]

mod native;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Mode {
    SetupOnly,
    Roundtrip,
}
fn mode(args: &[String]) -> Result<Mode, &'static str> {
    match args {
        [arg] if arg == "--setup-only" => Ok(Mode::SetupOnly),
        [arg] if arg == "--roundtrip" => Ok(Mode::Roundtrip),
        _ => Err("exactly --setup-only or --roundtrip required"),
    }
}

// No cached proof authorizes a backend entry. Native observations are refreshed
// immediately before every read/write, on this single console thread.
#[derive(Clone, Debug)]
struct Proof {
    created_only: bool,
    private_dacl: bool,
    station: String,
    current_station: String,
    station_handles_match: bool,
    flags: u32,
    current_flags: u32,
    desktop: String,
    current_desktop: String,
    desktop_handles_match: bool,
    window_live: bool,
    window_hidden: bool,
    window_pid: u32,
    pid: u32,
    window_tid: u32,
    tid: u32,
    window_desktop_matches: bool,
}
impl Proof {
    fn allowed(&self) -> bool {
        self.created_only
            && self.private_dacl
            && !self.station.is_empty()
            && !self.station.eq_ignore_ascii_case("WinSta0")
            && self.station == self.current_station
            && self.station_handles_match
            && self.flags & 1 == 0
            && self.current_flags == self.flags
            && !self.desktop.is_empty()
            && self.desktop == self.current_desktop
            && self.desktop_handles_match
            && self.window_live
            && self.window_hidden
            && self.pid != 0
            && self.window_pid == self.pid
            && self.tid != 0
            && self.window_tid == self.tid
            && self.window_desktop_matches
    }
}

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let result = mode(&args).and_then(|mode| native::run(mode).map_err(|_| "probe failed"));
    if let Err(error) = result {
        println!("outcome=blocked error={error}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mado_file_clipboard::{FilePacket, Intent, decode_packet};
    use std::{ffi::OsString, os::windows::ffi::OsStringExt, path::PathBuf};

    fn valid() -> Proof {
        Proof {
            created_only: true,
            private_dacl: true,
            station: "Service-generated".into(),
            current_station: "Service-generated".into(),
            station_handles_match: true,
            flags: 0,
            current_flags: 0,
            desktop: "p22-owned".into(),
            current_desktop: "p22-owned".into(),
            desktop_handles_match: true,
            window_live: true,
            window_hidden: true,
            window_pid: 42,
            pid: 42,
            window_tid: 43,
            tid: 43,
            window_desktop_matches: true,
        }
    }
    #[test]
    fn every_missing_isolation_observation_refuses() {
        assert!(valid().allowed());
        let changes: &[fn(&mut Proof)] = &[
            |p| p.created_only = false,
            |p| p.private_dacl = false,
            |p| p.station.clear(),
            |p| p.station = "wInStA0".into(),
            |p| p.current_station = "other".into(),
            |p| p.station_handles_match = false,
            |p| p.flags = 1,
            |p| p.current_flags = 1,
            |p| p.desktop.clear(),
            |p| p.current_desktop = "other".into(),
            |p| p.desktop_handles_match = false,
            |p| p.window_live = false,
            |p| p.window_hidden = false,
            |p| p.window_pid = 99,
            |p| p.pid = 0,
            |p| p.window_tid = 99,
            |p| p.tid = 0,
            |p| p.window_desktop_matches = false,
        ];
        for change in changes {
            let mut p = valid();
            change(&mut p);
            assert!(!p.allowed());
        }
    }
    #[test]
    fn cli_never_implicitly_enters_data_phase() {
        assert!(mode(&[]).is_err());
        assert_eq!(mode(&["--setup-only".into()]), Ok(Mode::SetupOnly));
        assert_eq!(mode(&["--roundtrip".into()]), Ok(Mode::Roundtrip));
        assert!(mode(&["--setup-only".into(), "--roundtrip".into()]).is_err());
    }
    #[test]
    fn copy_move_codec_preserves_unicode_and_unpaired_units_without_io() {
        let mut units: Vec<u16> = "C:\\p22-codec-only\\é-𐐀-".encode_utf16().collect();
        units.extend([0xd800, 46, 116, 120, 116]);
        let paths = vec![PathBuf::from(OsString::from_wide(&units))];
        for intent in [Intent::Copy, Intent::Move] {
            let packet = FilePacket::snapshot(&paths, intent).unwrap();
            assert_eq!(
                decode_packet(
                    &packet.encode_hdrop().unwrap(),
                    Some(&packet.encode_preferred_effect())
                )
                .unwrap(),
                packet
            );
        }
    }
}
