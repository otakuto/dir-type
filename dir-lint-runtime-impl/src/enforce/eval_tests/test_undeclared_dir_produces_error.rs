use std::path::{Path, PathBuf};

use indexmap::IndexMap;

use crate::enforce::TrialMemo;
use crate::enforce::eval::eval_node;
use crate::enforce::fixtures::{empty_scope, make_dir_entry};
use dir_lint_error::LintError;
use dir_lint_expr::{ExprPattern, ExprSubtree};
use dir_lint_walk::DirTree;

#[test]
fn test_undeclared_dir_produces_error() {
    // Arrange
    let tree = DirTree {
        name: "root".to_string(),
        dirs: vec![
            DirTree {
                name: "src".to_string(),
                dirs: vec![],
                files: vec![],
            },
            DirTree {
                name: "unknown_dir".to_string(),
                dirs: vec![],
                files: vec![],
            },
        ],
        files: vec![],
    };
    let entries = vec![make_dir_entry(
        ExprPattern::Exact("src".to_string()),
        Some((0, Some(1))),
        ExprSubtree::Leaf,
    )];
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
    assert_eq!(errors.len(), 1);
    let LintError::Undeclared { path: err_path, .. } = &errors[0] else {
        panic!("expected Undeclared, got {:?}", errors[0]);
    };
    assert_eq!(err_path, &PathBuf::from("root/unknown_dir"));
}
