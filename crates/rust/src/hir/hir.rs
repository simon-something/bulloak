//! Defines a high-level intermediate representation (HIR) for Rust tests.

use bulloak_syntax::Span;

/// A high-level intermediate representation (HIR) that describes
/// the semantic structure of a Rust test file as emitted by `bulloak`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Hir {
    /// An abstract root node that does not correspond
    /// to any concrete Rust construct.
    ///
    /// This represents the file boundary.
    Root(Root),
    /// A context struct for passing test state.
    Context(ContextStruct),
    /// A helper function (corresponds to conditions).
    Helper(HelperFunction),
    /// A test module (#[cfg(test)] mod tests).
    TestModule(TestModule),
    /// A test function.
    TestFunction(TestFunction),
    /// A comment.
    Comment(Comment),
}

impl Hir {
    /// Whether this HIR node is a root.
    #[must_use]
    pub fn is_root(&self) -> bool {
        matches!(self, Self::Root(_))
    }

    /// Whether this HIR node is a test module.
    #[must_use]
    pub fn is_test_module(&self) -> bool {
        matches!(self, Self::TestModule(_))
    }

    /// Whether this HIR node is a test function.
    #[must_use]
    pub fn is_test_function(&self) -> bool {
        matches!(self, Self::TestFunction(_))
    }

    /// Whether this HIR node is a helper function.
    #[must_use]
    pub fn is_helper(&self) -> bool {
        matches!(self, Self::Helper(_))
    }
}

impl Default for Hir {
    fn default() -> Self {
        Self::Root(Root::default())
    }
}

type Identifier = String;

/// The root HIR node.
///
/// There can only be one root node in any HIR.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Root {
    /// The children HIR nodes of this node.
    pub children: Vec<Hir>,
}

/// A context struct for passing test state between helpers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextStruct {
    /// The struct name (typically "TestContext").
    pub name: Identifier,
    /// Optional documentation comment.
    pub doc: Option<String>,
}

impl Default for ContextStruct {
    fn default() -> Self {
        Self {
            name: "TestContext".to_string(),
            doc: Some("Context for test conditions".to_string()),
        }
    }
}

/// A helper function that sets up test conditions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HelperFunction {
    /// The function name.
    pub name: Identifier,
    /// Optional documentation comment.
    pub doc: Option<String>,
    /// The span of the original tree node.
    pub span: Option<Span>,
}

/// A test module containing test functions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestModule {
    /// The module name (typically "tests").
    pub name: Identifier,
    /// The test functions in this module.
    pub children: Vec<Hir>,
}

impl Default for TestModule {
    fn default() -> Self {
        Self {
            name: "tests".to_string(),
            children: Vec::new(),
        }
    }
}

/// An attribute for a test function.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Attribute {
    /// #[test]
    Test,
    /// #[should_panic]
    ShouldPanic,
}

/// A test function.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestFunction {
    /// The function name.
    pub name: Identifier,
    /// Attributes (e.g., #[test], #[should_panic]).
    pub attributes: Vec<Attribute>,
    /// Names of helper functions to call.
    pub helpers: Vec<Identifier>,
    /// Comments in the function body.
    pub children: Vec<Hir>,
    /// The span of the original tree node.
    pub span: Option<Span>,
}

/// A comment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Comment {
    /// The comment text.
    pub text: String,
    /// Whether this comment should be formatted (capitalized/punctuated).
    pub format: bool,
}
