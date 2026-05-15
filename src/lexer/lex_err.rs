#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LexErr {
    Eof,
    InvalidToken,
}
