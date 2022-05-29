use crate::{Direction, Timestamp};

#[derive(Debug)]
pub struct Trade<K> {
    pub buy_key: K,
    pub sell_key: K,

    pub price: f64,
    pub volume: f64,
    pub timestamp: Timestamp,
    pub aggressor_side: Option<Direction>,
}

impl<K> Trade<K> {
    pub fn cost(&self) -> f64 {
        // TODO: Add "delivery period"? Maybe just as metadata with a constant factor?
        self.price * self.volume
    }
}
