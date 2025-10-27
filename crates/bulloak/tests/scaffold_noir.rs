//! Integration tests for Noir scaffolding.

use std::{fs, path::PathBuf, process::Command};

fn bulloak_binary() -> PathBuf {
    assert_cmd::cargo::cargo_bin("bulloak")
}

fn tests_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/scaffold_noir")
}

fn cmd(binary: &PathBuf, command: &str, tree_path: &PathBuf, extra_args: &[&str]) -> std::process::Output {
    let mut cmd = Command::new(binary);
    cmd.arg(command);
    cmd.args(extra_args);
    cmd.arg(tree_path);
    cmd.output().expect("Failed to execute bulloak")
}

#[test]
fn test_scaffold_noir_basic() {
    let binary_path = bulloak_binary();
    let tests_path = tests_path();
    let tree_path = tests_path.join("basic.tree");

    let output = cmd(&binary_path, "scaffold", &tree_path, &["--backend", "noir"]);
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = fs::read_to_string(tests_path.join("basic_test.nr")).unwrap();

    assert_eq!(expected.trim(), actual.trim(), "Basic scaffold output should match expected");
}

#[test]
fn test_scaffold_noir_with_panic() {
    let binary_path = bulloak_binary();
    let tests_path = tests_path();
    let tree_path = tests_path.join("with_panic.tree");

    let output = cmd(&binary_path, "scaffold", &tree_path, &["--backend", "noir"]);
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = fs::read_to_string(tests_path.join("with_panic_test.nr")).unwrap();

    assert_eq!(expected.trim(), actual.trim(), "Should generate #[test(should_fail)] for panic cases");
}

#[test]
fn test_scaffold_noir_no_helpers() {
    let binary_path = bulloak_binary();
    let tests_path = tests_path();
    let tree_path = tests_path.join("no_helpers.tree");

    let output = cmd(&binary_path, "scaffold", &tree_path, &["--backend", "noir"]);
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = fs::read_to_string(tests_path.join("no_helpers_test.nr")).unwrap();

    assert_eq!(expected.trim(), actual.trim(), "Simple test should work without helpers");
}

#[test]
fn test_scaffold_noir_nested() {
    let binary_path = bulloak_binary();
    let tests_path = tests_path();
    let tree_path = tests_path.join("nested.tree");

    let output = cmd(&binary_path, "scaffold", &tree_path, &["--backend", "noir"]);
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = fs::read_to_string(tests_path.join("nested_test.nr")).unwrap();

    assert_eq!(expected.trim(), actual.trim(), "Nested conditions should be handled correctly");
}
