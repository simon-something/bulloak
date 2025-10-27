//! Scaffold module for generating Rust test code.

pub mod comment;
pub mod generator;

pub use generator::Generator;

use crate::config::Config;
use anyhow::Result;
use bulloak_syntax::Ast;

/// Scaffold Rust test code from an AST.
///
/// # Errors
///
/// Returns an error if scaffolding fails.
pub fn scaffold(ast: &Ast, cfg: &Config) -> Result<String> {
    let generator = Generator::new(cfg);
    generator.generate(ast)
}
