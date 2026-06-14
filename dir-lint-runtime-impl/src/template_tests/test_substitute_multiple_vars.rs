use std::collections::HashMap;

use crate::template::substitute;

#[test]
fn test_substitute_multiple_vars() {
    // Arrange
    let mut scope = HashMap::new();
    scope.insert("layer".to_string(), "usecase".to_string());
    scope.insert("domain".to_string(), "foo".to_string());

    // Act
    let result = substitute("${layer}-${domain}", &scope);

    // Assert
    assert_eq!(result, "usecase-foo");
}
