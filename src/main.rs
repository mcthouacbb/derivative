use crate::{
    expr::{Expr, operator_info::Associativity},
    lexer::get_tokens,
    parser::parse_expr,
};

mod expr;
mod lexer;
mod parser;

fn serialize_expr(expr: &Expr) -> String {
    let operator_info = expr.get_operator_info();

    match expr {
        Expr::Binary(binary_expr) => {
            let mut left = serialize_expr(binary_expr.left());
            let mut right = serialize_expr(binary_expr.right());
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
        Expr::Unary(unary_expr) => {
            let mut right = serialize_expr(unary_expr.right());
            let right_operator_info = unary_expr.right().get_operator_info();
            if right_operator_info.precedence() < operator_info.precedence() {
                right = "(".to_owned() + &right + ")";
            }
            format!("{}{}", unary_expr.op(), right)
        }
        Expr::Var(var_expr) => var_expr.name().to_string(),
        Expr::Const(const_expr) => const_expr.value().to_string(),
    }
}

fn main() {
    /*let str = "x + (3 - 4) / 7 * (5^z) + y";
    println!("{:?}", get_tokens(str));

    let str = "3 4 + / - 8* 33( 02343 )) yy38ss";
    println!("{:?}", get_tokens(str));

    let str = "3 38 7 342394 s0FEWf ) &";
    println!("{:?}", get_tokens(str));*/

    let str = "x + (3 - 4) / 7 * (5^z) + y";
    let tokens = get_tokens(str).expect("Can't lex");
    let expr_tree = parse_expr(tokens).expect("Can't parse");

    println!("{:?}", expr_tree);
    println!("{}", serialize_expr(&expr_tree));

    let str = "--x + x - (y - y) + y - (x / (x / y) - y) - y ^ (x ^ y) ^ (y ^ x) ^ z";
    let tokens = get_tokens(str).expect("Can't lex");
    let expr_tree = parse_expr(tokens).expect("Can't parse");

    println!("{:?}", expr_tree);
    println!("{}", serialize_expr(&expr_tree));

    let str = "x + (3 - 4) / 7 * *(5^z) + y";
    let tokens = get_tokens(str).expect("Can't lex");
    let result = parse_expr(tokens);

    println!("{:?}", result);
}
