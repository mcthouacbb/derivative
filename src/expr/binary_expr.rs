use core::fmt;

use crate::expr::Expr;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Add => '+',
                Self::Sub => '-',
                Self::Mul => '*',
                Self::Div => '/',
            }
        )
    }
}

#[derive(Clone)]
pub struct BinaryExpr {
    left: Box<Expr>,
    right: Box<Expr>,
    op: BinaryOp,
}

impl BinaryExpr {
    fn new(left: Box<Expr>, right: Box<Expr>, op: BinaryOp) -> Self {
        Self { left, right, op }
    }

    fn left(&self) -> &Expr {
        self.left.as_ref()
    }

    fn right(&self) -> &Expr {
        self.right.as_ref()
    }

    fn op(&self) -> BinaryOp {
        self.op
    }
}
