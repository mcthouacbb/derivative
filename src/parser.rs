pub mod parse_err;

use crate::{
    lexer::token::{Token, TokenKind},
    parser::parse_err::ParseErr,
    syntax::{SyntaxNode, binary_node::BinaryOp, unary_node::UnaryOp},
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

    fn parse(&mut self) -> Result<SyntaxNode, ParseErr> {
        self.parse_additive()
    }

    fn parse_additive(&mut self) -> Result<SyntaxNode, ParseErr> {
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
            result = SyntaxNode::new_binary(Box::new(result), Box::new(right), op);
        }
        Ok(result)
    }

    fn parse_multiplicative(&mut self) -> Result<SyntaxNode, ParseErr> {
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
            result = SyntaxNode::new_binary(Box::new(result), Box::new(right), op);
        }
        Ok(result)
    }

    fn parse_power(&mut self) -> Result<SyntaxNode, ParseErr> {
        let result = self.parse_unary()?;
        if self.match_tok(TokenKind::Caret) {
            let right = self.parse_power()?;
            Ok(SyntaxNode::new_binary(
                Box::new(result),
                Box::new(right),
                BinaryOp::Pow,
            ))
        } else {
            Ok(result)
        }
    }

    fn parse_unary(&mut self) -> Result<SyntaxNode, ParseErr> {
        if self.match_tok(TokenKind::Minus) {
            let syntax_node = self.parse_unary()?;
            Ok(SyntaxNode::new_unary(Box::new(syntax_node), UnaryOp::Neg))
        } else {
            self.parse_primary()
        }
    }

    fn parse_primary(&mut self) -> Result<SyntaxNode, ParseErr> {
        if self.match_tok(TokenKind::Literal) {
            let tok = self.peek_last().unwrap();
            let value = tok
                .str()
                .parse::<f64>()
                .expect("Valid literal token can't be parsed");

            Ok(SyntaxNode::new_const(value))
        } else if self.match_tok(TokenKind::Identifier) {
            let tok = self.peek_last().unwrap();
            let name = tok.str().to_string();

            Ok(SyntaxNode::new_var(name))
        } else if self.match_tok(TokenKind::OpenParen) {
            let syntax_node = self.parse_additive()?;
            self.expect(TokenKind::CloseParen)?;

            Ok(syntax_node)
        } else {
            Err(ParseErr::ExpectedPrimary)
        }
    }
}

pub fn parse_syntax_tree<'a>(tokens: Vec<Token<'a>>) -> Result<SyntaxNode, ParseErr> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}
