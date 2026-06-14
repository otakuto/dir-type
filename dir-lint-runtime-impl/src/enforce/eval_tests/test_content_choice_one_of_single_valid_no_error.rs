use std::path::Path;

use crate::enforce::TrialMemo;
use crate::enforce::eval::eval_node;
use crate::enforce::fixtures::{content_choice_rules, empty_scope, make_splice_group};
use dir_lint_walk::DirTree;

/// one_of: only one valid (valid=1) → no errors.
#[test]
fn test_content_choice_one_of_single_valid_no_error() {
    // Arrange: only group.toml exists (only resource_group_dir is valid)
    let rules = content_choice_rules();
    let tree = DirTree {
        name: "foo".to_string(),
        dirs: vec![],
        files: vec!["group.toml".to_string()],
    };
    let entries = vec![make_splice_group(
        1,
        Some(1), // one_of
        &["resource_dir", "resource_group_dir"],
    )];
    let scope = empty_scope();
    let path = Path::new("envs/foo");

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

    // Assert: exactly 1 valid, so no errors
    assert!(
        errors.is_empty(),
        "expected no errors when exactly 1 is valid: {:?}",
        errors
    );
}
