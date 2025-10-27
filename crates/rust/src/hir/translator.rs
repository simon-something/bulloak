//! Translates a `bulloak-syntax` AST into a Rust HIR.

use bulloak_syntax::{Action, Ast, Condition};

use super::hir::{
    Attribute, Comment, ContextStruct, HelperFunction, Hir, Root, TestFunction, TestModule,
};
use crate::constants::{PANIC_KEYWORDS, TEST_MODULE_NAME};

/// Translates a `bulloak-syntax` AST into a Rust HIR.
pub struct Translator {
    /// Whether to format/capitalize descriptions.
    format_descriptions: bool,
    /// Whether to skip helper functions.
    skip_helpers: bool,
}

impl Translator {
    /// Create a new translator.
    #[must_use]
    pub fn new(format_descriptions: bool, skip_helpers: bool) -> Self {
        Self {
            format_descriptions,
            skip_helpers,
        }
    }

    /// Translate an AST into a HIR.
    ///
    /// # Errors
    ///
    /// Returns an error if translation fails.
    pub fn translate(&self, ast: &Ast) -> anyhow::Result<Hir> {
        let mut root = Root::default();

        // Add context struct
        root.children.push(Hir::Context(ContextStruct::default()));

        // Get the root node's children
        let ast_root = match ast {
            Ast::Root(r) => r,
            _ => anyhow::bail!("Expected Root node"),
        };

        // Collect all unique conditions as helper functions
        if !self.skip_helpers {
            let helpers = self.collect_helpers(&ast_root.children);
            for helper in helpers {
                root.children.push(Hir::Helper(helper));
            }
        }

        // Create test module
        let test_module = self.translate_test_module(&ast_root.children)?;
        root.children.push(Hir::TestModule(test_module));

        Ok(Hir::Root(root))
    }

    /// Collect all unique conditions as helper functions.
    fn collect_helpers(&self, children: &[Ast]) -> Vec<HelperFunction> {
        let mut helpers = Vec::new();
        let mut seen = std::collections::HashSet::new();

        self.collect_helpers_recursive(children, &mut helpers, &mut seen);

        helpers
    }

    /// Recursively collect helpers from the AST tree.
    fn collect_helpers_recursive(
        &self,
        children: &[Ast],
        helpers: &mut Vec<HelperFunction>,
        seen: &mut std::collections::HashSet<String>,
    ) {
        for child in children {
            if let Ast::Condition(condition) = child {
                let name = self.condition_to_helper_name(condition);
                if !seen.contains(&name) {
                    seen.insert(name.clone());
                    helpers.push(HelperFunction {
                        name: name.clone(),
                        doc: Some(condition.title.clone()),
                        span: Some(condition.span.clone()),
                    });
                }
                // Recursively collect from nested conditions
                self.collect_helpers_recursive(&condition.children, helpers, seen);
            }
        }
    }

    /// Translate the AST into a test module.
    fn translate_test_module(&self, children: &[Ast]) -> anyhow::Result<TestModule> {
        let mut module = TestModule {
            name: TEST_MODULE_NAME.to_string(),
            children: Vec::new(),
        };

        // Process all children to generate test functions
        self.process_children(children, &[], &mut module.children)?;

        Ok(module)
    }

    /// Process AST children recursively to generate test functions.
    fn process_children(
        &self,
        children: &[Ast],
        parent_helpers: &[String],
        output: &mut Vec<Hir>,
    ) -> anyhow::Result<()> {
        for child in children {
            match child {
                Ast::Condition(condition) => {
                    let helper_name = self.condition_to_helper_name(condition);
                    let mut new_helpers = parent_helpers.to_vec();
                    new_helpers.push(helper_name);
                    self.process_children(&condition.children, &new_helpers, output)?;
                }
                Ast::Action(action) => {
                    let test_fn = self.translate_action(action, parent_helpers)?;
                    output.push(Hir::TestFunction(test_fn));
                }
                _ => {}
            }
        }
        Ok(())
    }

    /// Translate an action into a test function.
    fn translate_action(
        &self,
        action: &Action,
        helpers: &[String],
    ) -> anyhow::Result<TestFunction> {
        let name = self.action_to_test_name(action, helpers);
        let should_panic = self.should_panic(&action.title);

        let mut attributes = vec![Attribute::Test];
        if should_panic {
            attributes.push(Attribute::ShouldPanic);
        }

        let mut children = Vec::new();

        // Add action title as comment
        children.push(Hir::Comment(Comment {
            text: action.title.clone(),
            format: self.format_descriptions,
        }));

        // Add descriptions as comments
        for desc_ast in &action.children {
            if let Ast::ActionDescription(desc) = desc_ast {
                children.push(Hir::Comment(Comment {
                    text: desc.text.clone(),
                    format: self.format_descriptions,
                }));
            }
        }

        Ok(TestFunction {
            name,
            attributes,
            helpers: helpers.to_vec(),
            children,
            span: Some(action.span.clone()),
        })
    }

    /// Convert a condition title to a helper function name.
    fn condition_to_helper_name(&self, condition: &Condition) -> String {
        self.to_snake_case(&condition.title)
    }

    /// Convert an action to a test function name.
    fn action_to_test_name(&self, action: &Action, helpers: &[String]) -> String {
        let action_part = self.to_snake_case(&action.title);

        if helpers.is_empty() {
            format!("test_{}", action_part)
        } else {
            // Include the last helper in the name for context
            let last_helper = &helpers[helpers.len() - 1];
            format!("test_{}_{}", last_helper, action_part)
        }
    }

    /// Convert a string to snake_case.
    fn to_snake_case(&self, s: &str) -> String {
        // Remove "when", "given", "it" prefixes (case-insensitive)
        let s = s.trim();
        let s = s
            .strip_prefix("when ")
            .or_else(|| s.strip_prefix("When "))
            .or_else(|| s.strip_prefix("WHEN "))
            .or_else(|| s.strip_prefix("given "))
            .or_else(|| s.strip_prefix("Given "))
            .or_else(|| s.strip_prefix("GIVEN "))
            .or_else(|| s.strip_prefix("it "))
            .or_else(|| s.strip_prefix("It "))
            .or_else(|| s.strip_prefix("IT "))
            .unwrap_or(s);

        // Convert to snake_case
        let mut result = String::new();
        let mut prev_is_alphanumeric = false;

        for c in s.chars() {
            if c.is_alphanumeric() {
                if c.is_uppercase() && prev_is_alphanumeric && !result.is_empty() {
                    result.push('_');
                }
                result.push(c.to_ascii_lowercase());
                prev_is_alphanumeric = true;
            } else if c.is_whitespace() || c == '-' {
                if prev_is_alphanumeric {
                    result.push('_');
                    prev_is_alphanumeric = false;
                }
            } else {
                // Skip other characters
                prev_is_alphanumeric = false;
            }
        }

        // Remove trailing underscores
        result.trim_end_matches('_').to_string()
    }

    /// Check if an action title indicates the test should panic.
    fn should_panic(&self, title: &str) -> bool {
        let title_lower = title.to_lowercase();
        PANIC_KEYWORDS
            .iter()
            .any(|keyword| title_lower.contains(keyword))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_snake_case() {
        let translator = Translator::new(false, false);

        assert_eq!(
            translator.to_snake_case("when first arg is smaller"),
            "first_arg_is_smaller"
        );
        assert_eq!(
            translator.to_snake_case("It should return the sum"),
            "should_return_the_sum"
        );
        assert_eq!(
            translator.to_snake_case("Given a valid input"),
            "a_valid_input"
        );
    }

    #[test]
    fn test_should_panic() {
        let translator = Translator::new(false, false);

        assert!(translator.should_panic("It should panic"));
        assert!(translator.should_panic("It should revert"));
        assert!(translator.should_panic("It should fail with error"));
        assert!(!translator.should_panic("It should return a value"));
    }
}
