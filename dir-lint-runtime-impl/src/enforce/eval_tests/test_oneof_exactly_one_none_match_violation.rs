use std::path::Path;

use indexmap::IndexMap;

use crate::enforce::TrialMemo;
use crate::enforce::eval::eval_node;
use crate::enforce::fixtures::{empty_scope, make_file_entry};
use dir_lint_error::LintError;
use dir_lint_expr::{ExprEntry, ExprMatcher, ExprPattern, Quant};
use dir_lint_walk::DirTree;

/// one_of (min=1, max=1): neither alternative exists → CardinalityViolation
#[test]
fn test_oneof_exactly_one_none_match_violation() {
    // Arrange
    let tree = DirTree {
        name: "root".to_string(),
        dirs: vec![],
        files: vec![],
    };
    let alt1 = make_file_entry(ExprPattern::Exact("config.toml".to_string()), None);
    let alt2 = make_file_entry(ExprPattern::Exact("config.yaml".to_string()), None);
    let group_entry = ExprEntry {
        id: None,
        source_path: None,
        count: Quant::Default,
        matcher: ExprMatcher::Choice {
            min: 1,
            max: Some(1),
            body: vec![alt1, alt2],
        },
    };
    let entries = vec![group_entry];
    let scope = empty_scope();
    let rules = IndexMap::new();
    let path = Path::new("root");

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

    // Assert
    let violations: Vec<_> = errors
        .iter()
        .filter(|e| matches!(e, LintError::CardinalityViolation { .. }))
        .collect();
    assert_eq!(
        violations.len(),
        1,
        "expected CardinalityViolation when neither alternative exists: {:?}",
        errors
    );
    let LintError::CardinalityViolation {
        realized, min, max, ..
    } = violations[0]
    else {
        panic!("expected CardinalityViolation");
    };
    assert_eq!(*realized, 0);
    assert_eq!(*min, 1);
    assert_eq!(*max, Some(1));
}
