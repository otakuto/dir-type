use std::path::Path;

use crate::ConfigExpr;

/// Port for obtaining a `ConfigExpr` from a configuration source (concrete implementations
/// such as YAML are provided by config-impl).
pub trait ConfigSource {
    /// Loads configuration from the specified path and returns a validated `ConfigExpr`.
    ///
    /// Returns `anyhow::Result` to handle both I/O/parse errors and validation errors.
    /// Validation errors are wrapped in `anyhow` as `ConfigErrors`.
    fn load(&self, path: &Path) -> anyhow::Result<ConfigExpr>;
}
