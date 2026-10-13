//! Independent versioned session codec and single-writer durable storage.
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::ffi::OsString;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::os::windows::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum SavedLocation {
    #[default]
    Filesystem,
    RecycleBin,
}

pub const VERSION: u32 = 1;
pub const MAX_BYTES: usize = 4 * 1024 * 1024;
const MAX_TABS: usize = 128;
static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum SavedSort {
    #[default]
    Name,
    Size,
    Modified,
    Type,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum SavedView {
    #[default]
    Details,
    Icons,
}
fn default_preview_zoom() -> f32 {
    1.0
}
fn default_enabled() -> bool {
    true
}

fn default_icon_size() -> f32 {
    96.0
}

fn default_directories_first() -> bool {
    true
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct TabState {
    pub id: u64,
    #[serde(default)]
    pub kind: SavedLocation,
    pub path: Vec<u16>,
    pub filter: String,
    pub show_hidden: bool,
    pub show_system: bool,
    #[serde(default)]
    pub sort: SavedSort,
    #[serde(default)]
    pub descending: bool,
    #[serde(default = "default_directories_first")]
    pub directories_first: bool,
    #[serde(default)]
    pub expanded: bool,
    #[serde(default)]
    pub view: SavedView,
    #[serde(default = "default_icon_size")]
    pub icon_size: f32,
    #[serde(default = "default_enabled")]
    pub inspector_visible: bool,
    #[serde(default = "default_preview_zoom")]
    pub preview_zoom: f32,
    #[serde(default = "default_enabled")]
    pub word_wrap: bool,
    #[serde(default)]
    pub line_numbers: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct NamedLayout {
    pub name: String,
    pub tabs: Vec<TabState>,
    pub active: usize,
    pub layout: Option<Layout>,
}

/// Window geometry in logical points; runtime monitor discovery is not serialized.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct WindowGeometry {
    pub position: [f32; 2],
    pub size: [f32; 2],
    pub maximized: bool,
}
impl WindowGeometry {
    pub fn validate(&self) -> Result<(), String> {
        if self
            .position
            .iter()
            .any(|value| !value.is_finite() || value.abs() > 1_000_000.0)
            || self
                .size
                .iter()
                .any(|value| !value.is_finite() || !(100.0..=32768.0).contains(value))
        {
            return Err("Invalid window position or size".into());
        }
        Ok(())
    }
    pub fn clamped(&self, work_origin: [f32; 2], work_size: [f32; 2]) -> Result<Self, String> {
        self.validate()?;
        if work_origin
            .iter()
            .any(|value| !value.is_finite() || value.abs() > 1_000_000.0)
            || work_size
                .iter()
                .any(|value| !value.is_finite() || !(100.0..=32768.0).contains(value))
        {
            return Err("Invalid monitor work area".into());
        }
        let mut result = *self;
        for (axis, minimum) in [640.0_f32, 480.0].into_iter().enumerate() {
            result.size[axis] =
                self.size[axis].clamp(minimum.min(work_size[axis]), work_size[axis]);
            result.position[axis] = self.position[axis].clamp(
                work_origin[axis],
                work_origin[axis] + work_size[axis] - result.size[axis],
            );
        }
        Ok(result)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Session {
    pub version: u32,
    pub tabs: Vec<TabState>,
    pub active: usize,
    #[serde(default)]
    pub layout: Option<Layout>,
    #[serde(default)]
    pub settings: crate::preferences::Settings,
    #[serde(default)]
    pub presets: Vec<NamedLayout>,
    #[serde(default)]
    pub geometry: Option<WindowGeometry>,
}

/// Owned split tree. Tab identities occur exactly once across all leaves.
/// `vertical` divides the available width into left/right children.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum Layout {
    Leaf {
        tabs: Vec<u64>,
        active: u64,
    },
    Split {
        vertical: bool,
        ratio: f32,
        first: Box<Layout>,
        second: Box<Layout>,
    },
}

impl Layout {
    fn validate(&self, depth: usize, seen: &mut HashSet<u64>) -> Result<(), String> {
        if depth > 12 {
            return Err("Session layout exceeds depth 12".into());
        }
        match self {
            Self::Leaf { tabs, active } => {
                if tabs.is_empty() || tabs.len() > MAX_TABS || !tabs.contains(active) {
                    return Err("Invalid layout leaf tabs or active identity".into());
                }
                for id in tabs {
                    if !seen.insert(*id) {
                        return Err("Layout contains duplicate tab identity".into());
                    }
                }
            }
            Self::Split {
                ratio,
                first,
                second,
                ..
            } => {
                if !ratio.is_finite() || !(0.05..=0.95).contains(ratio) {
                    return Err("Layout split ratio must be finite and within 0.05..=0.95".into());
                }
                first.validate(depth + 1, seen)?;
                second.validate(depth + 1, seen)?;
            }
        }
        Ok(())
    }
}

pub fn encode_path(path: &Path) -> Vec<u16> {
    path.as_os_str().encode_wide().collect()
}

pub fn decode_path(units: &[u16]) -> Result<PathBuf, String> {
    if units.is_empty() || units.len() > 32767 || units.contains(&0) {
        return Err("Session path is empty, oversized or contains NUL".into());
    }
    let path = PathBuf::from(OsString::from_wide(units));
    if !path.is_absolute() {
        return Err("Session paths must be absolute".into());
    }
    Ok(path)
}

impl Session {
    pub fn validate(&self) -> Result<(), String> {
        if self.version != VERSION {
            return Err(format!(
                "Unsupported session version {}; preserved unchanged",
                self.version
            ));
        }
        if self.tabs.is_empty() || self.tabs.len() > MAX_TABS || self.active >= self.tabs.len() {
            return Err("Invalid session tab count or active index".into());
        }
        self.settings.validate()?;
        if let Some(geometry) = self.geometry {
            geometry.validate()?;
        }
        if self.presets.len() > 32 {
            return Err("At most 32 named layouts are supported".into());
        }
        let mut names = HashSet::new();
        for preset in &self.presets {
            if preset.name.trim().is_empty()
                || preset.name.len() > 128
                || !names.insert(&preset.name)
            {
                return Err(
                    "Layout names must be unique, nonempty and at most 128 UTF-8 bytes".into(),
                );
            }
            Session {
                version: VERSION,
                tabs: preset.tabs.clone(),
                active: preset.active,
                layout: preset.layout.clone(),
                settings: crate::preferences::Settings::default(),
                presets: Vec::new(),
                geometry: None,
            }
            .validate()?;
        }
        let mut ids = HashSet::new();
        for tab in &self.tabs {
            if !ids.insert(tab.id) || tab.filter.len() > 4096 {
                return Err("Duplicate tab identity or oversized filter".into());
            }
            match tab.kind {
                SavedLocation::Filesystem => {
                    decode_path(&tab.path)?;
                }
                SavedLocation::RecycleBin if !tab.path.is_empty() => {
                    return Err("Recycle Bin tabs cannot contain a filesystem path".into());
                }
                SavedLocation::RecycleBin => {}
            }
            if !tab.preview_zoom.is_finite() || !(0.01..=4.0).contains(&tab.preview_zoom) {
                return Err("Preview zoom must be finite and within 0.1..=4.0".into());
            }
            if !tab.icon_size.is_finite() || !(48.0..=192.0).contains(&tab.icon_size) {
                return Err("Icon size must be finite and within 48..=192".into());
            }
        }
        if let Some(layout) = &self.layout {
            let mut seen = HashSet::new();
            layout.validate(0, &mut seen)?;
            if seen != ids {
                return Err("Layout identities must match session tabs exactly".into());
            }
        }
        Ok(())
    }
}

pub fn decode(bytes: &[u8]) -> Result<Session, String> {
    if bytes.len() > MAX_BYTES {
        return Err("Session exceeds 4 MiB limit".into());
    }
    let session: Session = serde_json::from_slice(bytes)
        .map_err(|e| format!("Invalid session; preserved unchanged: {e}"))?;
    session.validate()?;
    Ok(session)
}

pub fn encode(session: &Session) -> Result<Vec<u8>, String> {
    session.validate()?;
    let bytes = serde_json::to_vec(session).map_err(|e| e.to_string())?;
    if bytes.len() > MAX_BYTES {
        return Err("Session exceeds 4 MiB limit".into());
    }
    Ok(bytes)
}

pub struct Store {
    directory: PathBuf,
    // The persistent lease file is never unlinked; the OS releases this exclusive
    // handle on process exit, so stale files cannot strand writer ownership.
    _lease: File,
}

impl Store {
    pub fn open(directory: PathBuf) -> Result<Self, String> {
        if !directory.is_absolute() {
            return Err("State directory must be absolute".into());
        }
        fs::create_dir_all(&directory).map_err(|e| format!("Create state directory: {e}"))?;
        let directory = fs::canonicalize(directory).map_err(|e| e.to_string())?;
        let lease = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .share_mode(0)
            .open(directory.join("writer.lock"))
            .map_err(|e| format!("State writer unavailable (another instance may own it): {e}"))?;
        Ok(Self {
            directory,
            _lease: lease,
        })
    }

    pub fn load(&self) -> Result<Option<Session>, String> {
        let file = match File::open(self.directory.join("session.json")) {
            Ok(file) => file,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(e) => return Err(format!("Read state: {e}")),
        };
        let mut bytes = Vec::new();
        file.take((MAX_BYTES + 1) as u64)
            .read_to_end(&mut bytes)
            .map_err(|e| e.to_string())?;
        decode(&bytes).map(Some)
    }

    pub fn save(&self, session: &Session) -> Result<(), String> {
        let bytes = encode(session)?;
        // Recheck disk on every save. In particular, never overwrite corrupt or
        // future state merely because startup fell back to a default session.
        let existing = self.load()?.is_some();
        let seq = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let temporary = self
            .directory
            .join(format!("session.{}.{}.tmp", std::process::id(), seq));
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)
            .map_err(|e| format!("Create state temporary: {e}"))?;
        let result = (|| {
            file.write_all(&bytes)
                .map_err(|e| format!("Write state temporary: {e}"))?;
            file.sync_all()
                .map_err(|e| format!("Synchronize state temporary: {e}"))?;
            drop(file);
            replacement::replace(&temporary, &self.directory.join("session.json"), existing)
        })();
        // Only remove the uniquely created, application-owned temporary file.
        // Never delete the prior state to make a failed replacement succeed.
        if result.is_err() {
            let _ = fs::remove_file(&temporary);
        }
        result
    }
}

mod replacement {
    use super::encode_path;
    use std::path::Path;
    use windows::Win32::Storage::FileSystem::{
        MOVEFILE_WRITE_THROUGH, MoveFileExW, REPLACE_FILE_FLAGS, ReplaceFileW,
    };
    use windows::core::PCWSTR;

    pub(super) fn replace(temp: &Path, destination: &Path, existing: bool) -> Result<(), String> {
        let source = encode_path(temp).into_iter().chain([0]).collect::<Vec<_>>();
        let target = encode_path(destination)
            .into_iter()
            .chain([0])
            .collect::<Vec<_>>();
        // SAFETY: both buffers remain alive and NUL-terminated for synchronous
        // Windows calls. No pointer or borrowed handle escapes this adapter.
        let result = unsafe {
            if existing {
                ReplaceFileW(
                    PCWSTR(target.as_ptr()),
                    PCWSTR(source.as_ptr()),
                    PCWSTR::null(),
                    REPLACE_FILE_FLAGS(0),
                    None,
                    None,
                )
            } else {
                // No replace-existing flag: a raced destination is preserved.
                MoveFileExW(
                    PCWSTR(source.as_ptr()),
                    PCWSTR(target.as_ptr()),
                    MOVEFILE_WRITE_THROUGH,
                )
            }
        };
        result.map_err(|e| format!("Atomic state replacement failed; prior state retained: {e}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Session {
        Session {
            layout: None,
            settings: crate::preferences::Settings::default(),
            presets: Vec::new(),
            geometry: None,
            version: VERSION,
            tabs: vec![TabState {
                id: 1,
                kind: SavedLocation::Filesystem,
                path: encode_path(Path::new("C:\\fixture")),
                filter: String::new(),
                show_hidden: false,
                show_system: false,
                sort: SavedSort::Name,
                descending: false,
                directories_first: true,
                expanded: false,
                view: SavedView::Details,
                icon_size: 96.0,
                inspector_visible: true,
                preview_zoom: 1.0,
                word_wrap: true,
                line_numbers: false,
            }],
            active: 0,
        }
    }

    struct Fixture(PathBuf);
    impl Fixture {
        fn new() -> Self {
            let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("target")
                .join("session-fixtures")
                .join(format!(
                    "{}-{}",
                    std::process::id(),
                    TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed)
                ));
            fs::create_dir_all(&path).unwrap();
            Self(path)
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn session_window_geometry_clamps_and_rejects_invalid_input() {
        let geometry = WindowGeometry {
            position: [-5000.0, 5000.0],
            size: [2500.0, 1600.0],
            maximized: true,
        };
        let clamped = geometry.clamped([-1920.0, 0.0], [1920.0, 1040.0]).unwrap();
        assert_eq!(clamped.position, [-1920.0, 0.0]);
        assert_eq!(clamped.size, [1920.0, 1040.0]);
        assert!(clamped.maximized);
        let small = WindowGeometry {
            position: [0.0, 0.0],
            size: [100.0, 100.0],
            maximized: false,
        };
        assert_eq!(
            small.clamped([0.0, 0.0], [320.0, 240.0]).unwrap().size,
            [320.0, 240.0]
        );
        assert!(small.clamped([0.0, 0.0], [f32::NAN, 500.0]).is_err());
        let mut saved = sample();
        saved.geometry = Some(geometry);
        assert_eq!(decode(&encode(&saved).unwrap()).unwrap(), saved);
        saved.geometry.as_mut().unwrap().position[0] = f32::INFINITY;
        assert!(saved.validate().is_err());
    }

    #[test]
    fn recycle_bin_location_codec() {
        let mut saved = sample();
        saved.tabs[0].kind = SavedLocation::RecycleBin;
        saved.tabs[0].path.clear();
        assert_eq!(decode(&encode(&saved).unwrap()).unwrap(), saved);
        saved.tabs[0].path = encode_path(Path::new("C:\\not-a-bin-path"));
        assert!(saved.validate().is_err());
        saved.tabs[0].kind = SavedLocation::Filesystem;
        saved.tabs[0].path.clear();
        assert!(saved.validate().is_err());
        let mut legacy = serde_json::to_value(sample()).unwrap();
        legacy["tabs"][0].as_object_mut().unwrap().remove("kind");
        assert_eq!(
            decode(&serde_json::to_vec(&legacy).unwrap()).unwrap().tabs[0].kind,
            SavedLocation::Filesystem
        );
    }

    #[test]
    fn layout_presets_codec_and_validation() {
        let mut session = sample();
        session.tabs[0].sort = SavedSort::Modified;
        session.tabs[0].descending = true;
        session.tabs[0].expanded = true;
        session.presets.push(NamedLayout {
            name: "Research".into(),
            tabs: session.tabs.clone(),
            active: 0,
            layout: None,
        });
        assert_eq!(decode(&encode(&session).unwrap()).unwrap(), session);
        session.presets.push(session.presets[0].clone());
        assert!(session.validate().is_err());
        session.presets.pop();
        session.presets[0].name.clear();
        assert!(session.validate().is_err());
        session.presets[0].name = "Valid".into();
        session.presets[0].active = 1;
        assert!(session.validate().is_err());
        let mut value = serde_json::to_value(sample()).unwrap();
        let object = value.as_object_mut().unwrap();
        object.remove("presets");
        let tab = object.get_mut("tabs").unwrap()[0].as_object_mut().unwrap();
        for field in [
            "sort",
            "descending",
            "directories_first",
            "expanded",
            "view",
            "icon_size",
            "inspector_visible",
            "preview_zoom",
            "word_wrap",
            "line_numbers",
        ] {
            tab.remove(field);
        }
        let legacy = decode(&serde_json::to_vec(&value).unwrap()).unwrap();
        assert!(legacy.presets.is_empty());
        assert_eq!(legacy.tabs[0].sort, SavedSort::Name);
        assert!(legacy.tabs[0].directories_first);
        assert!(legacy.tabs[0].inspector_visible);
        assert_eq!(legacy.tabs[0].preview_zoom, 1.0);
        assert!(legacy.tabs[0].word_wrap);
        assert!(!legacy.tabs[0].line_numbers);
        for zoom in [f32::NAN, f32::INFINITY, 0.001, 4.1] {
            let mut invalid = sample();
            invalid.tabs[0].preview_zoom = zoom;
            assert!(invalid.validate().is_err());
        }
    }

    #[test]
    fn session_codec_layout_validation() {
        let mut session = sample();
        let mut other = session.tabs[0].clone();
        other.id = 2;
        session.tabs.push(other);
        session.layout = Some(Layout::Split {
            vertical: true,
            ratio: 0.5,
            first: Box::new(Layout::Leaf {
                tabs: vec![1],
                active: 1,
            }),
            second: Box::new(Layout::Leaf {
                tabs: vec![2],
                active: 2,
            }),
        });
        assert_eq!(decode(&encode(&session).unwrap()).unwrap(), session);
        let good = session.layout.clone();
        for ratio in [f32::NAN, f32::INFINITY, 0.01, 0.99] {
            if let Some(Layout::Split { ratio: current, .. }) = &mut session.layout {
                *current = ratio;
            }
            assert!(session.validate().is_err());
        }
        session.layout = good;
        if let Some(Layout::Split { second, .. }) = &mut session.layout {
            **second = Layout::Leaf {
                tabs: vec![1],
                active: 1,
            };
        }
        assert!(session.validate().is_err());
        session.layout = Some(Layout::Leaf {
            tabs: vec![1, 2],
            active: 3,
        });
        assert!(session.validate().is_err());
        session.layout = Some(Layout::Leaf {
            tabs: vec![1, 3],
            active: 1,
        });
        assert!(session.validate().is_err());
        let mut deep = Layout::Leaf {
            tabs: vec![1],
            active: 1,
        };
        for _ in 0..33 {
            deep = Layout::Split {
                vertical: true,
                ratio: 0.5,
                first: Box::new(deep),
                second: Box::new(Layout::Leaf {
                    tabs: vec![2],
                    active: 2,
                }),
            };
        }
        session.layout = Some(deep);
        assert!(session.validate().unwrap_err().contains("depth"));
        // Missing layout remains compatible with the original simple-tab DTO.
        let bytes = serde_json::to_value(sample()).unwrap();
        let mut object = bytes.as_object().unwrap().clone();
        object.remove("layout");
        assert_eq!(
            decode(&serde_json::to_vec(&object).unwrap())
                .unwrap()
                .layout,
            None
        );
    }

    #[test]
    fn session_codec_lossless_and_validation() {
        let mut session = sample();
        session.tabs[0].path.push(0xd800);
        assert_eq!(decode(&encode(&session).unwrap()).unwrap(), session);
        assert_eq!(
            encode_path(&decode_path(&session.tabs[0].path).unwrap()),
            session.tabs[0].path
        );
        session.tabs.push(session.tabs[0].clone());
        assert!(encode(&session).is_err());
        session = sample();
        session.active = 1;
        assert!(encode(&session).is_err());
        assert!(decode(&vec![b' '; MAX_BYTES + 1]).is_err());
        assert!(decode_path(&[0]).is_err());
    }

    #[test]
    fn session_lifecycle_exclusive_lease_and_reopen() {
        let fixture = Fixture::new();
        let store = Store::open(fixture.0.clone()).unwrap();
        assert!(Store::open(fixture.0.clone()).is_err());
        assert_eq!(store.load().unwrap(), None);
        store.save(&sample()).unwrap();
        assert_eq!(store.load().unwrap(), Some(sample()));
        drop(store);
        assert!(Store::open(fixture.0.clone()).is_ok());
    }

    #[test]
    fn session_replacement_ignores_abandoned_temporary() {
        let fixture = Fixture::new();
        let store = Store::open(fixture.0.clone()).unwrap();
        store.save(&sample()).unwrap();
        let committed = fs::read(fixture.0.join("session.json")).unwrap();
        // Model an interrupted pre-commit write. This is not a killed-process or
        // power-loss test: reopen must ignore uncommitted temporary contents.
        let abandoned = fixture.0.join("session.abandoned-owned-fixture.tmp");
        fs::write(&abandoned, b"{partial uncommitted").unwrap();
        drop(store);
        let reopened = Store::open(fixture.0.clone()).unwrap();
        assert_eq!(reopened.load().unwrap(), Some(sample()));
        assert_eq!(fs::read(fixture.0.join("session.json")).unwrap(), committed);
        assert_eq!(fs::read(&abandoned).unwrap(), b"{partial uncommitted");
        let mut newer = sample();
        newer.tabs[0].filter = "committed-new".into();
        reopened.save(&newer).unwrap();
        assert_eq!(reopened.load().unwrap(), Some(newer));
        assert!(
            abandoned.exists(),
            "never clean a temp not created by this save"
        );
    }

    #[test]
    fn session_replacement_preserves_corrupt_future_and_failed_replace() {
        let fixture = Fixture::new();
        let store = Store::open(fixture.0.clone()).unwrap();
        let path = fixture.0.join("session.json");
        fs::write(&path, b"corrupt").unwrap();
        assert!(store.save(&sample()).is_err());
        assert_eq!(fs::read(&path).unwrap(), b"corrupt");
        let mut future = sample();
        future.version = 999;
        let future_bytes = serde_json::to_vec(&future).unwrap();
        fs::write(&path, &future_bytes).unwrap();
        assert!(store.save(&sample()).is_err());
        assert_eq!(fs::read(&path).unwrap(), future_bytes);
        fs::write(&path, encode(&sample()).unwrap()).unwrap();
        let old = fs::read(&path).unwrap();
        // Read sharing permits validation but denies delete sharing/replacement.
        let locked = OpenOptions::new()
            .read(true)
            .share_mode(1)
            .open(&path)
            .unwrap();
        let mut next = sample();
        next.tabs[0].filter = "new".into();
        assert!(store.save(&next).is_err());
        assert_eq!(fs::read(&path).unwrap(), old);
        drop(locked);
        store.save(&next).unwrap();
        assert_eq!(store.load().unwrap(), Some(next));
    }
}
