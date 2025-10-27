//! Noir backend for bulloak.
//!
//! This crate provides Noir test generation and validation for bulloak,
//! converting `.tree` specifications into Noir test files with `#[test]` attributes.

#![warn(missing_docs)]
#![warn(unreachable_pub)]

pub mod check;
pub mod config;
pub mod noir;
pub mod scaffold;

mod constants;
mod utils;

pub use config::Config;

use anyhow::Result;
use bulloak_syntax::Ast;

/// Generate Noir test code from an AST.
///
/// # Errors
///
/// Returns an error if code generation fails.
pub fn scaffold(ast: &Ast, cfg: &Config) -> Result<String> {
    scaffold::generate(ast, cfg)
}
