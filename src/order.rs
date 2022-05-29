use crate::Timestamp;

#[derive(Clone, Copy, Debug)]
pub struct Order {
    pub price: f64,
    pub volume: f64,
    pub timestamp: Timestamp,
}

impl Order {
    pub fn new(price: f64, volume: f64) -> Order {
        Order {
            price,
            volume,
            timestamp: Timestamp::default(),
        }
    }
    pub fn cost(&self) -> f64 {
        self.price * self.volume
    }
}
