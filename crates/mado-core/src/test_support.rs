//! Generated rows for core and frontend regression checks only.
use crate::directory::{Entry, EntryKind};
use std::{path::PathBuf, sync::Arc};

pub fn rows(names: &[&str]) -> Arc<Vec<Arc<Entry>>> {
    Arc::new(
        names
            .iter()
            .map(|name| {
                Arc::new(Entry {
                    path: PathBuf::from(name),
                    name: "same display text".into(),
                    kind: EntryKind::Folder,
                    modified: "—".into(),
                    modified_time: None,
                    size: None,
                    metadata_error: None,
                    hidden: false,
                    system: false,
                })
            })
            .collect(),
    )
}
