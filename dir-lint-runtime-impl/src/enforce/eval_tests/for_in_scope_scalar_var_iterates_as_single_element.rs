use std::path::Path;

use indexmap::IndexMap;

use crate::enforce::TrialMemo;
use crate::enforce::eval::eval_node;
use crate::enforce::fixtures::{
    empty_scope, make_file_entry, make_for_entry_expr, tree_with_files,
};
use dir_lint_expr::{ExprPattern, ExprRule};

/// `in: ${var}` iterates over a scalar variable in scope (scalar is treated as 1 element).
#[test]
fn for_in_scope_scalar_var_iterates_as_single_element() {
    // Arrange: scope has layer=usecase → for {id: x, value: ${layer}} { file: '${value.x}.rs' }
    let file_entry = make_file_entry(ExprPattern::Exact("${value.x}.rs".to_string()), None);
    let for_entry = make_for_entry_expr("x", "${layer}", vec![file_entry]);
    let tree = tree_with_files("root", vec!["usecase.rs"]);
    let entries = vec![for_entry];
    let rules: IndexMap<_, ExprRule> = IndexMap::new();
    let path = Path::new("root");

    let mut scope = empty_scope();
    // Place the scalar binding on the lex side (Value). The bare `${layer}` resolves via transparent get.
    scope.bind_lex(
        crate::node_id::NodeKind::Value,
        "layer",
        crate::value::Value::Scalar("usecase".to_string()),
    );

    // Act
    let mut errors = Vec::new();
    eval_node(
        &tree,
        &entries,
        &scope,
        &rules,
        path,
        "test_rule",
        &mut errors,
        &mut crate::record_map::RecordMap::new(),
        &mut TrialMemo::new(),
    );

    // Assert: usecase.rs exists, so no errors
    assert!(errors.is_empty(), "unexpected errors: {:?}", errors);
}
