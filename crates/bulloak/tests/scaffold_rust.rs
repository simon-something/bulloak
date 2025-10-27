#![allow(missing_docs)]
use std::{env, fs};

use common::{cmd, get_binary_path};
use pretty_assertions::assert_eq;

mod common;

#[cfg(not(target_os = "windows"))]
#[test]
fn scaffolds_rust_trees() {
    let cwd = env::current_dir().unwrap();
    let binary_path = get_binary_path();
    let tests_path = cwd.join("tests").join("scaffold_rust");
    let trees = [
        "basic.tree",
        "with_panic.tree",
        "no_helpers.tree",
        "nested.tree",
        "deeply_nested.tree",
        "multiple_actions.tree",
    ];

    for tree_name in trees {
        let tree_path = tests_path.join(tree_name);
        let output = cmd(&binary_path, "scaffold", &tree_path, &["--backend", "rust"]);
        let actual = String::from_utf8(output.stdout).unwrap();

        let mut output_file = tree_path.clone();
        output_file.set_extension("");
        let mut output_file_str = output_file.into_os_string();
        output_file_str.push("_test.rs");
        let output_file: std::path::PathBuf = output_file_str.into();

        let expected = fs::read_to_string(&output_file).unwrap_or_else(|_| {
            panic!(
                "Failed to read expected output file: {}",
                output_file.display()
            )
        });

        // We trim here because we don't care about ending newlines.
        assert_eq!(
            expected.trim(),
            actual.trim(),
            "Mismatch for {}",
            tree_name
        );
    }
}

#[cfg(not(target_os = "windows"))]
#[test]
fn scaffolds_rust_trees_skip_helpers() {
    let cwd = env::current_dir().unwrap();
    let binary_path = get_binary_path();
    let tests_path = cwd.join("tests").join("scaffold_rust");
    let tree_name = "basic.tree";

    let tree_path = tests_path.join(tree_name);
    let output = cmd(&binary_path, "scaffold", &tree_path, &["--backend", "rust", "-m"]);
    let actual = String::from_utf8(output.stdout).unwrap();

    // Should not contain helper functions
    assert!(!actual.contains("fn first_arg_is_smaller_than_second_arg"));
    assert!(!actual.contains("fn first_arg_is_bigger_than_second_arg"));

    // Should still contain test module and context
    assert!(actual.contains("#[cfg(test)]"));
    assert!(actual.contains("mod tests"));
    assert!(actual.contains("struct TestContext"));
}

#[cfg(not(target_os = "windows"))]
#[test]
fn scaffolds_rust_trees_format_descriptions() {
    let cwd = env::current_dir().unwrap();
    let binary_path = get_binary_path();
    let tests_path = cwd.join("tests").join("scaffold_rust");
    let tree_name = "basic.tree";

    let tree_path = tests_path.join(tree_name);
    let output = cmd(
        &binary_path,
        "scaffold",
        &tree_path,
        &["--backend", "rust", "--format-descriptions"],
    );
    let actual = String::from_utf8(output.stdout).unwrap();

    // Comments should be capitalized and have periods
    assert!(actual.contains("// It should match the result of hash(a, b)."));
    assert!(actual.contains("// It should match the result of hash(b, a)."));
}
