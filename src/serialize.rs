use crate::expr::{
    Expr, binary_expr::BinaryExpr, operator_info::Associativity, unary_expr::UnaryExpr,
};

fn serialize_binary_expr(binary_expr: &BinaryExpr) -> String {
    let operator_info = binary_expr.get_operator_info();

    let mut left = serialize_syntax_tree(binary_expr.left());
    let mut right = serialize_syntax_tree(binary_expr.right());
    let left_operator_info = binary_expr.left().get_operator_info();
    let right_operator_info = binary_expr.right().get_operator_info();

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

    format!("{} {} {}", left, binary_expr.op(), right)
}

fn serialize_unary_expr(unary_expr: &UnaryExpr) -> String {
    let operator_info = unary_expr.get_operator_info();

    let mut right = serialize_syntax_tree(unary_expr.right());
    let right_operator_info = unary_expr.right().get_operator_info();
    if right_operator_info.precedence() < operator_info.precedence() {
        right = "(".to_owned() + &right + ")";
    }
    format!("{}{}", unary_expr.op(), right)
}

pub fn serialize_syntax_tree(syntax_node: &Expr) -> String {
    match syntax_node {
        Expr::Binary(binary_expr) => serialize_binary_expr(binary_expr),
        Expr::Unary(unary_expr) => serialize_unary_expr(unary_expr),
        Expr::Var(var_node) => var_node.name().to_string(),
        Expr::Const(const_node) => const_node.value().to_string(),
    }
}
