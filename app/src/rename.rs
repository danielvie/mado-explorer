//! Mutation-free batch preview. Numbering follows caller order, starts at 1 and
//! pads to 3 digits by default. Prefix/stem/suffix/number/date precede the last
//! extension (unless disabled). Dates are explicitly UTC Unix seconds, not locale
//! formatted dates. Cycles, swaps, existing destinations and no-op names fail.
use crate::platform::operations::{Operation, validate_name};
use std::{
    collections::HashSet,
    ffi::{OsStr, OsString},
    fs,
    os::windows::{
        fs::{MetadataExt, OpenOptionsExt},
        io::AsRawHandle,
    },
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Clone, Debug)]
pub struct RenameOptions {
    pub prefix: String,
    pub suffix: String,
    pub numbering: bool,
    pub start: u64,
    pub padding: usize,
    pub preserve_extension: bool,
    pub modified_utc_seconds: bool,
}
impl Default for RenameOptions {
    fn default() -> Self {
        Self {
            prefix: String::new(),
            suffix: String::new(),
            numbering: true,
            start: 1,
            padding: 3,
            preserve_extension: true,
            modified_utc_seconds: false,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Signature {
    len: u64,
    modified: Option<SystemTime>,
    created: Option<SystemTime>,
    attributes: u32,
    canonical: PathBuf,
    volume_serial: u32,
    file_index: u64,
}
impl Signature {
    pub fn read(path: &Path) -> Result<Self, String> {
        // Open the entry itself (including directories/reparse points) for metadata,
        // allowing other users of the file while recording its stable volume/file ID.
        let handle = fs::OpenOptions::new()
            .access_mode(0)
            .share_mode(7)
            .custom_flags(0x02000000 | 0x00200000)
            .open(path)
            .map_err(|e| e.to_string())?;
        let mut identity =
            windows::Win32::Storage::FileSystem::BY_HANDLE_FILE_INFORMATION::default();
        unsafe {
            windows::Win32::Storage::FileSystem::GetFileInformationByHandle(
                windows::Win32::Foundation::HANDLE(handle.as_raw_handle()),
                &mut identity,
            )
        }
        .map_err(|e| e.to_string())?;
        let meta = handle.metadata().map_err(|e| e.to_string())?;
        Ok(Self {
            volume_serial: identity.dwVolumeSerialNumber,
            file_index: ((identity.nFileIndexHigh as u64) << 32) | identity.nFileIndexLow as u64,
            len: meta.len(),
            modified: meta.modified().ok(),
            created: meta.created().ok(),
            attributes: meta.file_attributes(),
            canonical: path.canonicalize().map_err(|e| e.to_string())?,
        })
    }
}
#[derive(Clone, Debug)]
pub struct RenameItem {
    pub source: PathBuf,
    pub target: PathBuf,
    pub errors: Vec<String>,
    signature: Option<Signature>,
}
#[derive(Clone, Debug)]
pub struct RenamePlan {
    pub items: Vec<RenameItem>,
    pub options: RenameOptions,
    pub errors: Vec<String>,
}
impl RenamePlan {
    /// Captures metadata but never changes any selected entry. Input order is the
    /// numbering order; the caller must rebuild after selection/options change.
    pub fn build(paths: &[PathBuf], options: RenameOptions) -> Self {
        let mut plan = Self {
            items: Vec::new(),
            options,
            errors: Vec::new(),
        };
        if paths.is_empty() || paths.len() > 4096 {
            plan.errors.push("Select 1–4096 entries".into());
            return plan;
        }
        if plan.options.padding > 20 {
            plan.errors
                .push("Number padding must be between 0 and 20".into());
            return plan;
        }
        for (index, path) in paths.iter().enumerate() {
            let mut item = RenameItem {
                source: path.clone(),
                target: path.clone(),
                errors: Vec::new(),
                signature: None,
            };
            if !path.is_absolute() {
                item.errors.push("Source must be an absolute path".into());
            }
            match Signature::read(path) {
                Ok(signature) => item.signature = Some(signature),
                Err(error) => item.errors.push(format!("Source unavailable: {error}")),
            }
            let is_folder = item
                .signature
                .as_ref()
                .is_some_and(|s| s.attributes & 0x10 != 0);
            match proposed_name(
                path,
                &plan.options,
                index,
                is_folder,
                item.signature.as_ref().and_then(|s| s.modified),
            ) {
                Ok(name) => {
                    if let Err(error) = validate_name(&name) {
                        item.errors.push(error);
                    }
                    if let Some(parent) = path.parent() {
                        item.target = parent.join(name);
                    } else {
                        item.errors.push("Cannot rename filesystem root".into());
                    }
                }
                Err(error) => item.errors.push(error),
            }
            if item.source == item.target {
                item.errors.push("Proposed name is unchanged".into());
            }
            match item.target.try_exists() {
                Ok(true) => item
                    .errors
                    .push("Target already exists; swaps/cycles are unsupported".into()),
                Err(e) => item.errors.push(format!("Cannot inspect target: {e}")),
                Ok(false) => {}
            }
            plan.items.push(item);
        }
        let mut sources = HashSet::new();
        let mut targets = HashSet::new();
        for item in &mut plan.items {
            if !sources.insert(case_key(&item.source)) {
                item.errors.push("Duplicate source".into());
            }
            if !targets.insert(case_key(&item.target)) {
                item.errors
                    .push("Duplicate proposed target (case insensitive)".into());
            }
        }
        plan
    }
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
            && !self.items.is_empty()
            && self.items.iter().all(|item| item.errors.is_empty())
    }
    /// Rechecks captured metadata and destination absence immediately before submission.
    /// This is not an atomic filesystem transaction: executor validates each item again.
    pub fn revalidate(&self) -> Result<(), String> {
        if !self.is_valid() {
            return Err("Rename preview contains conflicts or invalid names".into());
        }
        let mut sources = HashSet::new();
        let mut targets = HashSet::new();
        for item in &self.items {
            if item.source.parent() != item.target.parent()
                || !sources.insert(case_key(&item.source))
                || !targets.insert(case_key(&item.target))
            {
                return Err("Changed plan contains a duplicate or cross-directory target".into());
            }
            if Some(Signature::read(&item.source)?) != item.signature {
                return Err(format!(
                    "Source changed since preview: {}",
                    item.source.display()
                ));
            }
            if item.target.try_exists().map_err(|e| e.to_string())? {
                return Err(format!(
                    "Target appeared since preview: {}",
                    item.target.display()
                ));
            }
            validate_name(item.target.file_name().ok_or("Missing target name")?)?;
        }
        Ok(())
    }
    /// Returns ordered independent operations, with no rollback or all-or-nothing promise.
    pub fn into_operations(self) -> Result<Vec<Operation>, String> {
        self.revalidate()?;
        self.items
            .into_iter()
            .map(|item| {
                Ok(Operation::Rename {
                    expected: item.signature,
                    source: item.source,
                    name: item
                        .target
                        .file_name()
                        .ok_or("Missing target name")?
                        .to_os_string(),
                })
            })
            .collect()
    }
}
fn case_key(path: &Path) -> String {
    path.to_string_lossy().to_uppercase()
}
fn proposed_name(
    path: &Path,
    options: &RenameOptions,
    index: usize,
    is_folder: bool,
    modified: Option<SystemTime>,
) -> Result<OsString, String> {
    let original = path.file_name().ok_or("Missing source filename")?;
    let extension = if options.preserve_extension && !is_folder {
        path.extension()
    } else {
        None
    };
    let stem = if extension.is_some() {
        path.file_stem().unwrap_or(original)
    } else {
        original
    };
    let mut name = OsString::from(&options.prefix);
    name.push(stem);
    name.push(&options.suffix);
    if options.numbering {
        let number = options
            .start
            .checked_add(index as u64)
            .ok_or("Sequence number overflow")?;
        name.push(format!("_{number:0width$}", width = options.padding));
    }
    if options.modified_utc_seconds {
        let seconds = modified
            .ok_or("Modification time unavailable")?
            .duration_since(UNIX_EPOCH)
            .map_err(|_| "Modification time precedes Unix epoch")?
            .as_secs();
        name.push(format!("_{seconds}"));
    }
    if let Some(extension) = extension {
        name.push(OsStr::new("."));
        name.push(extension);
    }
    Ok(name)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rename_plan_default_numbering_extension() {
        let options = RenameOptions::default();
        assert_eq!(
            proposed_name(
                Path::new("C:\\fixture\\report.tar.gz"),
                &options,
                0,
                false,
                None
            )
            .unwrap(),
            "report.tar_001.gz"
        );
        assert_eq!(
            proposed_name(
                Path::new("C:\\fixture\\folder.ext"),
                &options,
                1,
                true,
                None
            )
            .unwrap(),
            "folder.ext_002"
        );
    }
    #[test]
    fn rename_plan_explicit_utc_seconds() {
        let options = RenameOptions {
            numbering: false,
            modified_utc_seconds: true,
            prefix: "pre-".into(),
            suffix: "-post".into(),
            ..Default::default()
        };
        assert_eq!(
            proposed_name(Path::new("a.txt"), &options, 0, false, Some(UNIX_EPOCH)).unwrap(),
            "pre-a-post_0.txt"
        );
    }
    #[test]
    fn rename_plan_overflow() {
        let options = RenameOptions {
            start: u64::MAX,
            ..Default::default()
        };
        assert!(proposed_name(Path::new("a"), &options, 1, false, None).is_err());
    }
    #[test]
    fn rename_plan_owned_fixture_preview_and_revalidation() {
        let root = std::env::temp_dir().join(format!(
            "wayfarer-rename-preview-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&root).unwrap();
        let source = root.join("source.txt");
        fs::write(&source, b"owned source").unwrap();
        let plan = RenamePlan::build(std::slice::from_ref(&source), RenameOptions::default());
        assert!(plan.is_valid());
        assert!(plan.revalidate().is_ok());
        assert_eq!(fs::read(&source).unwrap(), b"owned source");
        assert!(!plan.items[0].target.exists());
        let duplicate = RenamePlan::build(
            &[source.clone(), source.clone()],
            RenameOptions {
                numbering: false,
                prefix: "new-".into(),
                ..Default::default()
            },
        );
        assert!(!duplicate.is_valid());
        let queued = plan.clone().into_operations().unwrap();
        fs::write(&source, b"changed owned source").unwrap();
        assert!(crate::platform::operations::validate(&queued[0]).is_err());
        assert!(plan.revalidate().is_err());
        fs::remove_file(source).unwrap();
        fs::remove_dir(root).unwrap();
    }
}
