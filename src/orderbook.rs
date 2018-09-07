use {Direction, Order, Side};

use std::hash::Hash;


// One side of an orderbook
// Need:
// - Fast iteration up to value
pub struct Orderbook<K>
where
    K: Hash + Eq + Clone,
{
    pub asks: Side<K>,
    pub bids: Side<K>,
}


impl<K: Hash + Eq + Clone> Orderbook<K> {
    pub fn new() -> Self {
        Self {
            asks: Side::new(Direction::Ask),
            bids: Side::new(Direction::Bid),
        }
    }

    pub fn get_order(&self, key: &K) -> Option<(Direction, Order)> {
        if let Some(res) = self.asks.get_order(key) {
            Some((Direction::Ask, res))
        } else if let Some(res) = self.bids.get_order(key) {
            Some((Direction::Bid, res))
        }
        else {
            None
        }
    }

    pub fn insert(&mut self, key: K, order: (Direction, Order)) -> usize {
        match order.0 {
            Direction::Ask => self.insert_ask(key, order.1),
            Direction::Bid => self.insert_bid(key, order.1),
        }
    }

    pub fn insert_bid(&mut self, key: K, order: Order) -> usize {
        self.bids.insert(key, order)
    }

    pub fn insert_ask(&mut self, key: K, order: Order) -> usize {
        self.asks.insert(key, order)
    }

    pub fn remove(&mut self, key: &K) {
        self.asks.remove(key);
        self.bids.remove(key);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
}
