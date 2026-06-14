use std::path::Path;

use dir_lint_error::RuntimeError;
use dir_lint_expr::ConfigExpr;
use dir_lint_runtime::{CheckReport, Checker};
use dir_lint_walk::DirTree;

/// Checker implementation using the standard dir-lint engine.
pub struct DirLintChecker;

impl Checker for DirLintChecker {
    fn check(
        &self,
        config: &ConfigExpr,
        tree: &DirTree,
        base: &Path,
    ) -> Result<CheckReport, RuntimeError> {
        crate::check_dir::check_dir(config, tree, base)
    }
}
