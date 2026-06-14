use std::path::{Path, PathBuf};

use indexmap::IndexMap;

use crate::enforce::TrialMemo;
use crate::enforce::eval::eval_node;
use crate::enforce::fixtures::{empty_scope, make_file_entry};
use dir_lint_error::LintError;
use dir_lint_expr::ExprPattern;
use dir_lint_walk::DirTree;
use dir_lint_yaml::RegexPattern;

#[test]
fn test_regex_file_match_denies_invalid_name() {
    // Arrange
    let tree = DirTree {
        name: "src".to_string(),
        dirs: vec![],
        files: vec!["lib.rs".to_string(), "INVALID.rs".to_string()],
    };
    let entries = vec![make_file_entry(
        ExprPattern::Regex(RegexPattern(r"^[a-z_]+\.rs$".to_string())),
        None,
    )];
    let scope = empty_scope();
    let rules = IndexMap::new();
    let path = Path::new("src");

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
    assert_eq!(err_path, &PathBuf::from("src/INVALID.rs"));
}
