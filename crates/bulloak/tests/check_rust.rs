#![allow(missing_docs)]
use std::{env, fs};

use common::{cmd, get_binary_path};

mod common;

#[cfg(not(target_os = "windows"))]
#[test]
fn check_rust_passes_when_correct() {
    let cwd = env::current_dir().unwrap();
    let binary_path = get_binary_path();
    let tests_path = cwd.join("tests").join("scaffold_rust");
    let tree_name = "basic.tree";

    let tree_path = tests_path.join(tree_name);
    let output = cmd(&binary_path, "check", &tree_path, &["--rust"]);

    // Should pass with no violations
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("All checks completed successfully"));
}

#[cfg(not(target_os = "windows"))]
#[test]
fn check_rust_fails_when_missing_file() {
    let cwd = env::current_dir().unwrap();
    let binary_path = get_binary_path();
    let tests_path = cwd.join("tests").join("scaffold_rust");

    // Create a temporary tree file without corresponding test file
    let temp_tree = tests_path.join("temp_missing.tree");
    fs::write(
        &temp_tree,
        "test_func\n└── It should work.",
    )
    .unwrap();

    let output = cmd(&binary_path, "check", &temp_tree, &["--rust"]);

    // Should fail
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("Rust test file is missing"));

    // Clean up
    fs::remove_file(temp_tree).ok();
}

#[cfg(not(target_os = "windows"))]
#[test]
fn check_rust_fails_when_missing_test_function() {
    let cwd = env::current_dir().unwrap();
    let binary_path = get_binary_path();
    let tests_path = cwd.join("tests").join("scaffold_rust");

    // Create a tree file
    let temp_tree = tests_path.join("temp_incomplete.tree");
    fs::write(
        &temp_tree,
        "test_func\n├── It should work.\n└── It should also work differently.",
    )
    .unwrap();

    // Create an incomplete test file (missing one test)
    let temp_test = tests_path.join("temp_incomplete_test.rs");
    fs::write(
        &temp_test,
        r#"
#[derive(Default)]
struct TestContext {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_work() {
        // It should work.
    }
    // Missing: test_should_also_work_differently
}
"#,
    )
    .unwrap();

    let output = cmd(&binary_path, "check", &temp_tree, &["--rust"]);

    // Should fail
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("Test function") && stderr.contains("is missing"));

    // Clean up
    fs::remove_file(temp_tree).ok();
    fs::remove_file(temp_test).ok();
}
