#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Eof,
    Identifier,
    Literal,
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    OpenParen,
    CloseParen,
}

#[derive(Debug, Clone, Copy)]
pub struct Token<'a> {
    kind: TokenKind,
    str: Option<&'a str>,
}

impl<'a> Token<'a> {
    pub fn eof() -> Self {
        Self {
            kind: TokenKind::Eof,
            str: None,
        }
    }

    pub fn new(kind: TokenKind, str: &'a str) -> Self {
        Self {
            kind,
            str: Some(str),
        }
    }

    pub fn kind(&self) -> TokenKind {
        self.kind
    }

    pub fn str(&self) -> &str {
        assert!(self.kind != TokenKind::Eof);
        self.str.unwrap()
    }
}
