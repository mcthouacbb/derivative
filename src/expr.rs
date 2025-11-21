mod binary_expr;
mod unary_expr;

use binary_expr::{BinaryExpr, BinaryOp};
use unary_expr::{UnaryExpr, UnaryOp};

#[derive(Clone)]
enum Expr {
    Binary(BinaryExpr),
    Unary(UnaryExpr),
}

impl Expr {
    fn binary(left: Box<Expr>, right: Box<Expr>, op: BinaryOp) -> Self {
        Self::Binary(BinaryExpr::new(left, right, op))
    }

    fn unary(right: Box<Expr>, op: UnaryOp) -> Self {
        Self::Unary(UnaryExpr::new(right, op))
    }
}
