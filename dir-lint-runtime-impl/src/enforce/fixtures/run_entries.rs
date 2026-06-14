use std::path::Path;

use indexmap::IndexMap;

use crate::enforce::TrialMemo;
use crate::enforce::eval::eval_node;
use dir_lint_error::LintError;
use dir_lint_expr::ExprEntry;
use dir_lint_walk::DirTree;

/// Helper that runs eval_node with multiple entries and an empty scope.
pub fn run_entries(entries: &[ExprEntry], tree: &DirTree) -> Vec<LintError> {
    let scope = crate::env::Scope::new();
    let rules = IndexMap::new();
    let mut errors = Vec::new();
    let mut produced = crate::record_map::RecordMap::new();
    eval_node(
        tree,
        entries,
        &scope,
        &rules,
        Path::new("root"),
        "test_rule",
        &mut errors,
        &mut produced,
        &mut TrialMemo::new(),
    );
    errors
}
