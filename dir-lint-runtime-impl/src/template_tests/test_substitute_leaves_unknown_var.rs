use std::collections::HashMap;

use crate::template::substitute;

#[test]
fn test_substitute_leaves_unknown_var() {
    // Arrange
    let scope = HashMap::new();

    // Act
    let result = substitute("${unknown}", &scope);

    // Assert
    assert_eq!(result, "${unknown}");
}
