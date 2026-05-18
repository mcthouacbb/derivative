use crate::{lexer::get_tokens, parser::parse_syntax_tree, serialize::serialize_syntax_tree};

mod expr;
mod lexer;
mod parser;
mod serialize;

fn main() {
    /*let str = "x + (3 - 4) / 7 * (5^z) + y";
    println!("{:?}", get_tokens(str));

    let str = "3 4 + / - 8* 33( 02343 )) yy38ss";
    println!("{:?}", get_tokens(str));

    let str = "3 38 7 342394 s0FEWf ) &";
    println!("{:?}", get_tokens(str));*/

    let str = "x + (3 - 4) / 7 * (5^z) + y";
    let tokens = get_tokens(str).expect("Can't lex");
    let syntax_tree = parse_syntax_tree(tokens).expect("Can't parse");

    println!("{:?}", syntax_tree);
    println!("{}", serialize_syntax_tree(&syntax_tree));

    let str = "--(x - 33) + x - (y - y) + y - (x / (x / y) - y) - y ^ (x ^ y) ^ (y ^ x) ^ z";
    let tokens = get_tokens(str).expect("Can't lex");
    let syntax_tree = parse_syntax_tree(tokens).expect("Can't parse");

    println!("{:?}", syntax_tree);
    println!("{}", serialize_syntax_tree(&syntax_tree));

    let str = "x + (3 - 4) / 7 * *(5^z) + y";
    let tokens = get_tokens(str).expect("Can't lex");
    let result = parse_syntax_tree(tokens);

    println!("{:?}", result);
}
