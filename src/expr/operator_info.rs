#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    AddSub,
    MulDiv,
    Pow,
    Neg,
    Atomic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Associativity {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub struct OperatorInfo {
    precedence: Precedence,
    associativity: Option<(Associativity, bool)>,
}

impl OperatorInfo {
    pub const ADD: Self = Self {
        precedence: Precedence::AddSub,
        associativity: Some((Associativity::Left, true)),
    };
    pub const SUB: Self = Self {
        precedence: Precedence::AddSub,
        associativity: Some((Associativity::Left, false)),
    };

    pub const MUL: Self = Self {
        precedence: Precedence::MulDiv,
        associativity: Some((Associativity::Left, true)),
    };
    pub const DIV: Self = Self {
        precedence: Precedence::MulDiv,
        associativity: Some((Associativity::Left, false)),
    };

    pub const POW: Self = Self {
        precedence: Precedence::Pow,
        associativity: Some((Associativity::Right, false)),
    };

    pub const NEG: Self = Self {
        precedence: Precedence::Neg,
        associativity: None,
    };

    pub const ATOMIC: Self = Self {
        precedence: Precedence::Atomic,
        associativity: None,
    };

    pub fn precedence(&self) -> Precedence {
        self.precedence
    }

    pub fn associativity(&self) -> Option<(Associativity, bool)> {
        self.associativity
    }
}
