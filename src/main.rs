use crate::lexer::get_tokens;

mod expr;
mod lexer;

fn main() {
    let str = "x + (3 - 4) / 7 * (5^z) + y";
    println!("{:?}", get_tokens(str));

    let str = "3 4 + / - 8* 33( 02343 )) yy38ss";
    println!("{:?}", get_tokens(str));

    let str = "3 38 7 342394 s0FEWf ) &";
    println!("{:?}", get_tokens(str));
}
