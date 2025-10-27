//! Constants used in the Rust backend.

/// Default indentation for generated code.
pub(crate) const DEFAULT_INDENTATION: usize = 4;

/// Keywords that indicate a test should panic.
pub(crate) const PANIC_KEYWORDS: &[&str] = &[
    "panic",
    "panics",
    "revert",
    "reverts",
    "error",
    "errors",
    "fail",
    "fails",
];

/// Name of the test context struct.
pub(crate) const CONTEXT_STRUCT_NAME: &str = "TestContext";

/// Name of the test module.
pub(crate) const TEST_MODULE_NAME: &str = "tests";
