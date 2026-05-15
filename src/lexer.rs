pub mod lex_err;
pub mod token;

use std::{iter::Peekable, str::CharIndices};

use crate::lexer::{
    lex_err::LexErr,
    token::{Token, TokenKind},
};

struct Lexer<'a> {
    str: &'a str,
    start: Peekable<CharIndices<'a>>,
    curr: Peekable<CharIndices<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(str: &'a str) -> Self {
        Self {
            str,
            start: str.char_indices().peekable(),
            curr: str.char_indices().peekable(),
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.curr.peek().map(|&(_, chr)| chr)
    }

    fn advance(&mut self) -> Option<char> {
        self.curr.next().map(|(_, chr)| chr)
    }

    fn skip_whitespace(&mut self) {
        while self.peek().is_some_and(|chr| chr.is_whitespace()) {
            self.advance();
        }
    }

    fn get_tok_str(&mut self) -> &'a str {
        let start_idx = self.start.peek().unwrap().0;
        let curr_idx = self
            .curr
            .peek()
            .map(|&(idx, _)| idx)
            .unwrap_or(self.str.len());
        &self.str[start_idx..curr_idx]
    }

    fn next_token(&mut self) -> Result<Token<'a>, LexErr> {
        self.skip_whitespace();

        self.start = self.curr.clone();

        let peek = if let Some(chr) = self.advance() {
            chr
        } else {
            return Err(LexErr::Eof);
        };

        let kind = if peek.is_ascii_digit() {
            while self.peek().is_some_and(|chr| chr.is_ascii_digit()) {
                self.advance();
            }

            if self.peek().is_some_and(|chr| chr == '.') {
                self.advance();
                while self.peek().is_some_and(|chr| chr.is_ascii_digit()) {
                    self.advance();
                }
            }

            TokenKind::Literal
        } else if peek.is_alphabetic() {
            while self.peek().is_some_and(|chr| chr.is_ascii_alphanumeric()) {
                self.advance();
            }

            TokenKind::Identifier
        } else {
            match peek {
                '+' => TokenKind::Plus,
                '-' => TokenKind::Minus,
                '*' => TokenKind::Star,
                '/' => TokenKind::Slash,
                '^' => TokenKind::Caret,
                '(' => TokenKind::OpenParen,
                ')' => TokenKind::CloseParen,
                _ => return Err(LexErr::InvalidToken),
            }
        };

        let result = Ok(Token::new(kind, self.get_tok_str()));
        self.start = self.curr.clone();
        result
    }
}

pub fn get_tokens<'a>(str: &'a str) -> Result<Vec<Token<'a>>, LexErr> {
    let mut tokens = Vec::new();
    let mut lexer = Lexer::new(str);
    loop {
        match lexer.next_token() {
            Ok(tok) => tokens.push(tok),
            Err(err) => {
                if err == LexErr::Eof {
                    break;
                } else {
                    return Err(err);
                }
            }
        }
    }

    Ok(tokens)
}
