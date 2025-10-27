//! Structural matching rule that checks if Rust code matches the spec.

use crate::{
    check::violation::{Violation, ViolationKind},
    config::Config,
    hir::Translator,
    rust::ParsedRustFile,
};
use anyhow::Result;
use bulloak_syntax::Ast;
use std::collections::HashSet;

/// Check that the Rust file structurally matches the spec.
///
/// # Errors
///
/// Returns an error if checking fails.
pub fn check_structural_match(
    ast: &Ast,
    rust_source: &str,
    file_path: &str,
    cfg: &Config,
) -> Result<Vec<Violation>> {
    let mut violations = Vec::new();

    // Parse the Rust file
    let parsed = match ParsedRustFile::parse(rust_source) {
        Ok(p) => p,
        Err(e) => {
            violations.push(Violation::new(
                ViolationKind::RustFileInvalid(e.to_string()),
                file_path.to_string(),
            ));
            return Ok(violations);
        }
    };

    // Check test module exists
    if parsed.find_test_module().is_none() {
        violations.push(Violation::new(
            ViolationKind::TestModuleMissing,
            file_path.to_string(),
        ));
        return Ok(violations);
    }

    // Generate expected HIR from AST
    let translator = Translator::new(cfg.format_descriptions, cfg.skip_helpers);
    let hir = translator.translate(ast)?;

    // Extract expected test functions from HIR
    let mut expected_tests = Vec::new();
    let mut expected_helpers = HashSet::new();

    if let crate::hir::Hir::Root(root) = &hir {
        for child in &root.children {
            if let crate::hir::Hir::Helper(helper) = child {
                expected_helpers.insert(helper.name.clone());
            } else if let crate::hir::Hir::TestModule(module) = child {
                for test_child in &module.children {
                    if let crate::hir::Hir::TestFunction(func) = test_child {
                        expected_tests.push(func.clone());
                    }
                }
            }
        }
    }

    // Check helpers (if not skipped)
    if !cfg.skip_helpers {
        let found_helpers: HashSet<String> = parsed
            .find_helper_functions()
            .iter()
            .map(|f| f.sig.ident.to_string())
            .collect();

        for expected_helper in &expected_helpers {
            if !found_helpers.contains(expected_helper) {
                violations.push(Violation::new(
                    ViolationKind::HelperFunctionMissing(expected_helper.clone()),
                    file_path.to_string(),
                ));
            }
        }
    }

    // Check test functions
    let found_tests = parsed.find_test_functions();
    let found_test_names: HashSet<String> =
        found_tests.iter().map(|f| f.sig.ident.to_string()).collect();

    for expected_test in &expected_tests {
        if !found_test_names.contains(&expected_test.name) {
            violations.push(Violation::new(
                ViolationKind::TestFunctionMissing(expected_test.name.clone()),
                file_path.to_string(),
            ));
        } else {
            // Check attributes
            let found_fn = found_tests
                .iter()
                .find(|f| f.sig.ident == expected_test.name)
                .unwrap();

            let has_should_panic = ParsedRustFile::has_should_panic(found_fn);
            let expects_should_panic = expected_test
                .attributes
                .iter()
                .any(|a| matches!(a, crate::hir::Attribute::ShouldPanic));

            if expects_should_panic && !has_should_panic {
                violations.push(Violation::new(
                    ViolationKind::TestAttributeIncorrect {
                        function: expected_test.name.clone(),
                        expected: "#[should_panic]".to_string(),
                        found: "none".to_string(),
                    },
                    file_path.to_string(),
                ));
            }
        }
    }

    // Check order
    let expected_order: Vec<&str> = expected_tests.iter().map(|t| t.name.as_str()).collect();
    let found_order: Vec<&str> = found_tests
        .iter()
        .map(|f| f.sig.ident.to_string())
        .map(|s| Box::leak(s.into_boxed_str()) as &str)
        .collect();

    // Check if the order matches (found may have extra tests, but expected ones should be in order)
    let mut expected_idx = 0;
    for found_name in &found_order {
        if expected_idx < expected_order.len() && *found_name == expected_order[expected_idx] {
            expected_idx += 1;
        }
    }

    if expected_idx != expected_order.len() {
        violations.push(Violation::new(
            ViolationKind::TestOrderIncorrect,
            file_path.to_string(),
        ));
    }

    Ok(violations)
}
