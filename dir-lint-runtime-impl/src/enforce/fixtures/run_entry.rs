use dir_lint_error::LintError;
use dir_lint_expr::ExprEntry;
use dir_lint_walk::DirTree;

use super::run_entries::run_entries;

/// Helper that runs eval_node with a single entry and an empty scope.
pub fn run_entry(entry: ExprEntry, tree: &DirTree) -> Vec<LintError> {
    run_entries(&[entry], tree)
}
