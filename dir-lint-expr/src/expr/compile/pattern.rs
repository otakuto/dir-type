use crate::ExprPattern;
use dir_lint_yaml::YamlPattern;

pub fn to_expr_pattern(pattern: &YamlPattern) -> ExprPattern {
    match pattern {
        YamlPattern::Exact(s) => ExprPattern::Exact(s.clone()),
        YamlPattern::Spec(spec) => {
            // Assumption (post-validation): regex must be present.
            ExprPattern::Regex(
                spec.regex
                    .clone()
                    .expect("pattern spec has no regex (should have been validated)"),
            )
        }
    }
}
