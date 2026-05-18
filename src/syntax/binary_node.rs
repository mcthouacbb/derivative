use core::fmt;

use crate::syntax::{SyntaxNode, operator_info::OperatorInfo};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
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
                Self::Pow => '^',
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct BinaryNode {
    left: Box<SyntaxNode>,
    right: Box<SyntaxNode>,
    op: BinaryOp,
}

impl BinaryNode {
    pub fn new(left: Box<SyntaxNode>, right: Box<SyntaxNode>, op: BinaryOp) -> Self {
        Self { left, right, op }
    }

    pub fn left(&self) -> &SyntaxNode {
        self.left.as_ref()
    }

    pub fn right(&self) -> &SyntaxNode {
        self.right.as_ref()
    }

    pub fn op(&self) -> BinaryOp {
        self.op
    }

    pub fn get_operator_info(&self) -> OperatorInfo {
        match self.op {
            BinaryOp::Add => OperatorInfo::ADD,
            BinaryOp::Sub => OperatorInfo::SUB,
            BinaryOp::Mul => OperatorInfo::MUL,
            BinaryOp::Div => OperatorInfo::DIV,
            BinaryOp::Pow => OperatorInfo::POW,
        }
    }
}
