use core::fmt;

use crate::syntax::{SyntaxNode, operator_info::OperatorInfo};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone)]
pub struct UnaryNode {
    right: Box<SyntaxNode>,
    op: UnaryOp,
}

impl UnaryNode {
    pub fn new(right: Box<SyntaxNode>, op: UnaryOp) -> Self {
        Self { right, op }
    }

    pub fn right(&self) -> &SyntaxNode {
        self.right.as_ref()
    }

    pub fn op(&self) -> UnaryOp {
        self.op
    }

    pub fn get_operator_info(&self) -> OperatorInfo {
        match self.op {
            UnaryOp::Neg => OperatorInfo::NEG,
        }
    }
}
