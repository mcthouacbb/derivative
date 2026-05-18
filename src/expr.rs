pub mod binary_expr;
pub mod const_expr;
pub mod operator_info;
pub mod unary_expr;
pub mod var_expr;

use crate::expr::{
    binary_expr::{BinaryExpr, BinaryOp},
    const_expr::ConstExpr,
    operator_info::OperatorInfo,
    unary_expr::{UnaryExpr, UnaryOp},
    var_expr::VarExpr,
};

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Var(VarExpr),
    Const(ConstExpr),
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

    pub fn new_const(value: f64) -> Self {
        Self::Const(ConstExpr::new(value))
    }

    pub fn get_operator_info(&self) -> OperatorInfo {
        match self {
            Self::Binary(binary_expr) => binary_expr.get_operator_info(),
            Self::Unary(unary_expr) => unary_expr.get_operator_info(),
            Self::Var(_) | Self::Const(_) => OperatorInfo::ATOMIC,
        }
    }
}
