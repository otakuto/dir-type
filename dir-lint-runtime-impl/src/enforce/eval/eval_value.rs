use crate::env::Scope;

pub(super) fn eval_value(
    var: &dir_lint_yaml::VarName,
    value: &dir_lint_yaml::ValueExpr,
    work_scope: &mut Scope,
) {
    let bound = super::super::expand::eval_value_expr(value, work_scope);
    work_scope.bind_lex(crate::node_id::NodeKind::Value, var.0.clone(), bound);
}
