use std::path::Path;

use crate::{ConfigExpr, ConfigSource};

/// `ConfigSource` implementation that loads `ConfigExpr` from a YAML file.
pub struct YamlConfigSource;

impl ConfigSource for YamlConfigSource {
    fn load(&self, path: &Path) -> anyhow::Result<ConfigExpr> {
        let yaml = dir_lint_yaml::load_yaml_config(path)?;
        // ConfigErrors implements Error, so it can be wrapped in anyhow directly.
        crate::compile(yaml).map_err(anyhow::Error::new)
    }
}
