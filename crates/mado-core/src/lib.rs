//! Shared native browsing logic. Frontends supply wake callbacks and own presentation/input.
pub mod directory;
pub mod finder;
pub mod ids;
pub mod native;
pub mod navigation;
pub mod selection;
pub mod sorting;

pub use navigation::{Activation, active_activation, capture_activation};

#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub mod test_support;
