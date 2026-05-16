pub mod binary_expr;
pub mod literal_expr;
pub mod operator_info;
pub mod unary_expr;
pub mod var_expr;

use binary_expr::{BinaryExpr, BinaryOp};
use literal_expr::LiteralExpr;
use unary_expr::{UnaryExpr, UnaryOp};

use crate::expr::{operator_info::OperatorInfo, var_expr::VarExpr};

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Var(VarExpr),
    Literal(LiteralExpr),
}

impl Expr {
    pub fn new_binary(left: Box<Expr>, right: Box<Expr>, op: BinaryOp) -> Self {
        Self::Binary(BinaryExpr::new(left, right, op))
    }

    pub fn new_unary(right: Box<Expr>, op: UnaryOp) -> Self {
        Self::Unary(UnaryExpr::new(right, op))
    }

    pub fn new_var(name: String) -> Self {
        Self::Var(VarExpr::new(name))
    }

    pub fn new_literal(value: f64) -> Self {
        Self::Literal(LiteralExpr::new(value))
    }

    pub fn get_operator_info(&self) -> OperatorInfo {
        match self {
            Self::Binary(binary_expr) => binary_expr.get_operator_info(),
            Self::Unary(unary_expr) => unary_expr.get_operator_info(),
            Self::Var(_) | Self::Literal(_) => OperatorInfo::ATOMIC,
        }
    }
}
