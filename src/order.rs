#[derive(Clone, Copy, Debug)]
pub struct Order {
    pub price: f64,
    pub volume: f64,
}

impl Order {
    pub fn cost(&self) -> f64 {
        self.price * self.volume
    }
}
