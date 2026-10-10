//! P04 native notification dependency probe.
//!
//! No application watcher service is enabled or exported. The pinned Windows
//! backend silently loses overflow/terminal-error information required by F13.1.
//! See HANDOFF.md and the retained failing native overflow regression.
