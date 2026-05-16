pub mod parse_err;

use crate::{
    expr::{Expr, binary_expr::BinaryOp, unary_expr::UnaryOp},
    lexer::token::{Token, TokenKind},
    parser::parse_err::ParseErr,
};

struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    curr: usize,
}

impl<'a> Parser<'a> {
    fn new(tokens: Vec<Token<'a>>) -> Self {
        Self { tokens, curr: 0 }
    }

    fn peek_nth(&self, n: i32) -> Option<Token<'a>> {
        self.tokens.get(self.curr.wrapping_add(n as usize)).cloned()
    }

    fn peek(&self) -> Option<Token<'a>> {
        self.peek_nth(0)
    }

    fn peek_last(&self) -> Option<Token<'a>> {
        self.peek_nth(-1)
    }

    fn advance(&mut self) -> Token<'a> {
        let tok = self
            .peek()
            .expect("Attempting to advance more than one past the end");
        self.curr += 1;
        tok
    }

    fn match_tok(&mut self, kind: TokenKind) -> bool {
        if self.peek().is_some_and(|tok| tok.kind() == kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn expect(&mut self, kind: TokenKind) -> Result<(), ParseErr> {
        if self.match_tok(kind) {
            Ok(())
        } else {
            Err(ParseErr::ExpectedToken(kind))
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseErr> {
        self.parse_additive()
    }

    fn parse_additive(&mut self) -> Result<Expr, ParseErr> {
        let mut result = self.parse_multiplicative()?;
        loop {
            let op = if self.match_tok(TokenKind::Plus) {
                BinaryOp::Add
            } else if self.match_tok(TokenKind::Minus) {
                BinaryOp::Sub
            } else {
                break;
            };

            let right = self.parse_multiplicative()?;
            result = Expr::new_binary(Box::new(result), Box::new(right), op);
        }
        Ok(result)
    }

    fn parse_multiplicative(&mut self) -> Result<Expr, ParseErr> {
        let mut result = self.parse_power()?;
        loop {
            let op = if self.match_tok(TokenKind::Star) {
                BinaryOp::Mul
            } else if self.match_tok(TokenKind::Slash) {
                BinaryOp::Div
            } else {
                break;
            };

            let right = self.parse_power()?;
            result = Expr::new_binary(Box::new(result), Box::new(right), op);
        }
        Ok(result)
    }

    fn parse_power(&mut self) -> Result<Expr, ParseErr> {
        let result = self.parse_unary()?;
        if self.match_tok(TokenKind::Caret) {
            let right = self.parse_power()?;
            Ok(Expr::new_binary(
                Box::new(result),
                Box::new(right),
                BinaryOp::Pow,
            ))
        } else {
            Ok(result)
        }
    }

    fn parse_unary(&mut self) -> Result<Expr, ParseErr> {
        if self.match_tok(TokenKind::Minus) {
            let expr = self.parse_unary()?;
            Ok(Expr::new_unary(Box::new(expr), UnaryOp::Neg))
        } else {
            self.parse_primary()
        }
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseErr> {
        if self.match_tok(TokenKind::Literal) {
            let tok = self.peek_last().unwrap();
            let value = tok
                .str()
                .parse::<f64>()
                .expect("Valid literal token can't be parsed");

            Ok(Expr::new_const(value))
        } else if self.match_tok(TokenKind::Identifier) {
            let tok = self.peek_last().unwrap();
            let name = tok.str().to_string();

            Ok(Expr::new_var(name))
        } else if self.match_tok(TokenKind::OpenParen) {
            let expr = self.parse_additive()?;
            self.expect(TokenKind::CloseParen)?;

            Ok(expr)
        } else {
            Err(ParseErr::ExpectedPrimary)
        }
    }
}

pub fn parse_expr<'a>(tokens: Vec<Token<'a>>) -> Result<Expr, ParseErr> {
    let mut parser = Parser::new(tokens);
    parser.parse_expr()
}
