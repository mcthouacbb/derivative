#[derive(Debug, Clone)]
pub struct ConstNode {
    value: f64,
}

impl ConstNode {
    pub fn new(value: f64) -> Self {
        Self { value }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}
