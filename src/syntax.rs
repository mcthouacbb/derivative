pub mod binary_node;
pub mod const_node;
pub mod operator_info;
pub mod unary_node;
pub mod var_node;

use crate::syntax::{
    binary_node::{BinaryNode, BinaryOp},
    const_node::ConstNode,
    operator_info::OperatorInfo,
    unary_node::{UnaryNode, UnaryOp},
    var_node::VarNode,
};

#[derive(Debug, Clone)]
pub enum SyntaxNode {
    Binary(BinaryNode),
    Unary(UnaryNode),
    Var(VarNode),
    Const(ConstNode),
}

impl SyntaxNode {
    pub fn new_binary(left: Box<SyntaxNode>, right: Box<SyntaxNode>, op: BinaryOp) -> Self {
        Self::Binary(BinaryNode::new(left, right, op))
    }

    pub fn new_unary(right: Box<SyntaxNode>, op: UnaryOp) -> Self {
        Self::Unary(UnaryNode::new(right, op))
    }

    pub fn new_var(name: String) -> Self {
        Self::Var(VarNode::new(name))
    }

    pub fn new_const(value: f64) -> Self {
        Self::Const(ConstNode::new(value))
    }

    pub fn get_operator_info(&self) -> OperatorInfo {
        match self {
            Self::Binary(binary_node) => binary_node.get_operator_info(),
            Self::Unary(unary_node) => unary_node.get_operator_info(),
            Self::Var(_) | Self::Const(_) => OperatorInfo::ATOMIC,
        }
    }
}
