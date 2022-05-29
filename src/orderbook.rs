use crate::matching::match_sides;
use crate::{Direction, Order, Side, Trade};

use std::collections::HashMap;
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

    order_side: HashMap<K, Direction>,
}

impl<K: Hash + Eq + Clone> Orderbook<K> {
    pub fn new() -> Self {
        Self {
            asks: Side::new(Direction::Ask),
            bids: Side::new(Direction::Bid),
            order_side: Default::default(),
        }
    }

    pub fn get_order(&self, key: &K) -> Option<(Direction, Order)> {
        self.order_side.get(key).map(|side| {
            let side = *side;
            (side, self.get_side(side).get_order(key).unwrap())
        })
    }

    pub fn insert(&mut self, key: K, order: (Direction, Order)) -> usize {
        let (side, order) = order;
        self.order_side.insert(key.clone(), side);
        self.get_mut_side(side).insert(key, order)
    }

    pub fn insert_bid(&mut self, key: K, order: Order) -> usize {
        self.insert(key, (Direction::Bid, order))
    }

    pub fn insert_ask(&mut self, key: K, order: Order) -> usize {
        self.insert(key, (Direction::Ask, order))
    }

    pub fn remove(&mut self, key: &K) {
        let removed = self.order_side.remove(key);
        if let Some(side) = removed {
            self.get_mut_side(side).remove(key)
        }
    }

    pub fn resolve_matches(&mut self) -> Vec<Trade<K>> {
        match_sides(self)
    }

    fn get_side(&self, direction: Direction) -> &Side<K> {
        match direction {
            Direction::Ask => &self.asks,
            Direction::Bid => &self.bids,
        }
    }

    fn get_mut_side(&mut self, direction: Direction) -> &mut Side<K> {
        match direction {
            Direction::Ask => &mut self.asks,
            Direction::Bid => &mut self.bids,
        }
    }
}

impl<K: Hash + Eq + Clone> Default for Orderbook<K> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
}
