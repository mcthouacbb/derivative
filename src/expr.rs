use crate::expr::binary_expr::BinaryExpr;

mod binary_expr;

enum Expr {
    Binary(BinaryExpr),
}
