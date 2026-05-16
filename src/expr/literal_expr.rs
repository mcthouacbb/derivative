#[derive(Debug, Clone)]
pub struct LiteralExpr {
    value: f64,
}

impl LiteralExpr {
    pub fn new(value: f64) -> Self {
        Self { value }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}
