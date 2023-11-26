use ruff_python_ast as ast;
use ruff_python_codegen::Generator;
use ruff_text_size::TextRange;

/// Format a code snippet to call `name.method()`.
pub(super) fn generate_method_call(name: &str, method: &str, generator: Generator) -> String {
    // Construct `name`.
    let var = ast::ExprName {
        id: name.into(),
        ctx: ast::ExprContext::Load,
        range: TextRange::default(),
    };
    // Construct `name.method`.
    let attr = ast::ExprAttribute {
        value: Box::new(var.into()),
        attr: ast::Identifier::new(method.to_string(), TextRange::default()),
        ctx: ast::ExprContext::Load,
        range: TextRange::default(),
    };
    // Make it into a call `name.method()`
    let call = ast::ExprCall {
        func: Box::new(attr.into()),
        arguments: ast::Arguments {
            args: vec![],
            keywords: vec![],
            range: TextRange::default(),
        },
        range: TextRange::default(),
    };
    // And finally, turn it into a statement.
    let stmt = ast::StmtExpr {
        value: Box::new(call.into()),
        range: TextRange::default(),
    };
    generator.stmt(&stmt.into())
}

/// Format a code snippet comparing `name` to `None` (e.g., `name is None`).
pub(super) fn generate_none_identity_comparison(
    name: &str,
    negate: bool,
    generator: Generator,
) -> String {
    // Construct `name`.
    let var = ast::ExprName {
        id: name.into(),
        ctx: ast::ExprContext::Load,
        range: TextRange::default(),
    };
    // Construct `name is None` or `name is not None`.
    let op = if negate {
        ast::CmpOp::IsNot
    } else {
        ast::CmpOp::Is
    };
    let compare = ast::ExprCompare {
        left: Box::new(var.into()),
        ops: vec![op],
        comparators: vec![ast::Expr::NoneLiteral(ast::ExprNoneLiteral::default())],
        range: TextRange::default(),
    };
    generator.expr(&compare.into())
}
