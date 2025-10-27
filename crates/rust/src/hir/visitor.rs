//! Visitor pattern for traversing the HIR.

use super::hir::{
    Comment, ContextStruct, HelperFunction, Hir, Root, TestFunction, TestModule,
};

/// A visitor trait for traversing the HIR.
pub trait Visitor: Sized {
    /// The result type of visiting a node.
    type Output;

    /// Visit a HIR node.
    fn visit(&mut self, hir: &Hir) -> Self::Output {
        match hir {
            Hir::Root(root) => self.visit_root(root),
            Hir::Context(context) => self.visit_context(context),
            Hir::Helper(helper) => self.visit_helper(helper),
            Hir::TestModule(module) => self.visit_test_module(module),
            Hir::TestFunction(func) => self.visit_test_function(func),
            Hir::Comment(comment) => self.visit_comment(comment),
        }
    }

    /// Visit a root node.
    fn visit_root(&mut self, root: &Root) -> Self::Output;

    /// Visit a context struct.
    fn visit_context(&mut self, context: &ContextStruct) -> Self::Output;

    /// Visit a helper function.
    fn visit_helper(&mut self, helper: &HelperFunction) -> Self::Output;

    /// Visit a test module.
    fn visit_test_module(&mut self, module: &TestModule) -> Self::Output;

    /// Visit a test function.
    fn visit_test_function(&mut self, func: &TestFunction) -> Self::Output;

    /// Visit a comment.
    fn visit_comment(&mut self, comment: &Comment) -> Self::Output;
}
