use tree_sitter::Parser;

fn print_tree(node: tree_sitter::Node, source: &str, depth: usize) {
    let indent = "  ".repeat(depth);
    let text = node.utf8_text(source.as_bytes()).unwrap_or("");
    let text_preview = if text.len() > 50 {
        format!("{}...", &text[..50])
    } else {
        text.to_string()
    };

    println!("{}{}  [{}]", indent, node.kind(), text_preview.replace('\n', "\\n"));

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        print_tree(child, source, depth + 1);
    }
}

#[test]
fn debug_noir_ast() {
    let source = r#"
#[test]
fn test_something() {
    assert(true);
}
"#;

    let mut parser = Parser::new();
    parser.set_language(tree_sitter_noir::language()).unwrap();
    let tree = parser.parse(source, None).unwrap();

    println!("\n=== AST STRUCTURE ===");
    print_tree(tree.root_node(), source, 0);
}

#[test]
fn debug_noir_should_fail() {
    let source = r#"
#[test(should_fail)]
fn test_panics() {
    assert(false);
}
"#;

    let mut parser = Parser::new();
    parser.set_language(tree_sitter_noir::language()).unwrap();
    let tree = parser.parse(source, None).unwrap();

    println!("\n=== AST STRUCTURE (should_fail) ===");
    print_tree(tree.root_node(), source, 0);
}

#[test]
fn debug_noir_unconstrained() {
    let source = r#"
#[test]
unconstrained fn test_something() {
    assert(true);
}
"#;

    let mut parser = Parser::new();
    parser.set_language(tree_sitter_noir::language()).unwrap();
    let tree = parser.parse(source, None).unwrap();

    println!("\n=== AST STRUCTURE (unconstrained) ===");
    print_tree(tree.root_node(), source, 0);
}
