//! Validation rules for Noir tests.

pub mod rules;
pub mod violation;

use anyhow::Result;
use std::path::Path;

use crate::Config;
pub use violation::Violation;

/// Check that a Noir test file matches its tree specification.
///
/// # Errors
///
/// Returns an error if checking fails.
pub fn check(tree_path: &Path, cfg: &Config) -> Result<Vec<Violation>> {
    rules::structural_match::check(tree_path, cfg)
}
