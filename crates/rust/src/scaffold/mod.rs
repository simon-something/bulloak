//! Scaffold module for generating Rust test code.

pub mod comment;
pub mod emitter;

pub use emitter::Emitter;

use crate::{config::Config, hir::Translator};
use anyhow::Result;
use bulloak_syntax::Ast;

/// Scaffold Rust test code from an AST.
///
/// # Errors
///
/// Returns an error if scaffolding fails.
pub fn scaffold(ast: &Ast, cfg: &Config) -> Result<String> {
    // Translate AST to HIR
    let translator = Translator::new(cfg.format_descriptions, cfg.skip_helpers);
    let hir = translator.translate(ast)?;

    // Emit Rust code from HIR
    let emitter = Emitter::new(cfg.format_descriptions);
    let code = emitter.emit(&hir);

    Ok(code)
}
