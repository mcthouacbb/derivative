mod binary_expr;
mod unary_expr;

use binary_expr::{BinaryExpr, BinaryOp};
use unary_expr::{UnaryExpr, UnaryOp};

#[derive(Clone)]
enum Expr {
    Binary(BinaryExpr),
    Unary(UnaryExpr),
}
