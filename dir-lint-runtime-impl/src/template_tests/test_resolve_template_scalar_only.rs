use crate::env::Scope;
use crate::node_id::NodeKind;
use crate::template::resolve_template;
use crate::value::Value;

/// A scalar-only template returns a single resolved string.
#[test]
fn test_resolve_template_scalar_only() {
    // Arrange
    let mut scope = Scope::new();
    scope.bind_lex(
        NodeKind::Regex,
        "layer",
        Value::Scalar("usecase".to_string()),
    );

    // Act
    let result = resolve_template("myapp-${layer}-foo", &scope);

    // Assert
    assert_eq!(result, "myapp-usecase-foo");
}
