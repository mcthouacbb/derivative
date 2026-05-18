use crate::syntax::{
    SyntaxNode, binary_node::BinaryNode, operator_info::Associativity, unary_node::UnaryNode,
};

fn serialize_binary_node(binary_node: &BinaryNode) -> String {
    let operator_info = binary_node.get_operator_info();

    let mut left = serialize_syntax_tree(binary_node.left());
    let mut right = serialize_syntax_tree(binary_node.right());
    let left_operator_info = binary_node.left().get_operator_info();
    let right_operator_info = binary_node.right().get_operator_info();

    if left_operator_info.precedence() < operator_info.precedence()
        || (left_operator_info.precedence() == operator_info.precedence()
            && operator_info
                .associativity()
                .is_some_and(|(associativity, associative)| {
                    associativity == Associativity::Right && !associative
                }))
    {
        left = "(".to_owned() + &left + ")";
    }

    if right_operator_info.precedence() < operator_info.precedence()
        || (right_operator_info.precedence() == operator_info.precedence()
            && operator_info
                .associativity()
                .is_some_and(|(associativity, associative)| {
                    associativity == Associativity::Left && !associative
                }))
    {
        right = "(".to_owned() + &right + ")";
    }

    format!("{} {} {}", left, binary_node.op(), right)
}

fn serialize_unary_node(unary_node: &UnaryNode) -> String {
    let operator_info = unary_node.get_operator_info();

    let mut right = serialize_syntax_tree(unary_node.right());
    let right_operator_info = unary_node.right().get_operator_info();
    if right_operator_info.precedence() < operator_info.precedence() {
        right = "(".to_owned() + &right + ")";
    }
    format!("{}{}", unary_node.op(), right)
}

pub fn serialize_syntax_tree(syntax_node: &SyntaxNode) -> String {
    match syntax_node {
        SyntaxNode::Binary(binary_node) => serialize_binary_node(binary_node),
        SyntaxNode::Unary(unary_node) => serialize_unary_node(unary_node),
        SyntaxNode::Var(var_node) => var_node.name().to_string(),
        SyntaxNode::Const(const_node) => const_node.value().to_string(),
    }
}
