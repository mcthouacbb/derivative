use core::fmt;

use crate::expr::Expr;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Neg => '-',
            }
        )
    }
}

#[derive(Clone)]
pub struct UnaryExpr {
    right: Box<Expr>,
    op: UnaryOp,
}

impl UnaryExpr {
    fn new(right: Box<Expr>, op: UnaryOp) -> Self {
        Self { right, op }
    }

    fn right(&self) -> &Expr {
        self.right.as_ref()
    }

    fn op(&self) -> UnaryOp {
        self.op
    }
}
