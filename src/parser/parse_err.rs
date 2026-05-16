use crate::lexer::token::TokenKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseErr {
    ExpectedPrimary,
    ExpectedToken(TokenKind),
}
