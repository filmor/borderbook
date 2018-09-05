use {Direction, Order};

use std::hash::Hash;
use std::cmp::Ordering::Equal;
use std::collections::HashMap;

// One side of an orderbook
// Need:
// - Fast iteration up to value
pub struct Orderbook<K>
where
    K: Hash + Eq + Clone,
{
    direction: Direction,

    map: HashMap<K, usize>,
    inverse_map: HashMap<usize, K>,

    orders: Vec<Option<Order>>,
    sorting: Vec<usize>,

    free_list: Vec<usize>,
}


impl<K: Hash + Eq + Clone> Orderbook<K> {
    pub fn new(dir: Direction) -> Self {
        Self {
            direction: dir,
            map: Default::default(),
            inverse_map: Default::default(),
            orders: Default::default(),
            sorting: Default::default(),

            free_list: Default::default(),
        }
    }

    pub fn get_order(&self, key: &K) -> Option<Order> {
        if let Some(&res) = self.map.get(key) {
            self.orders[res]
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.sorting.len()
    }

    pub fn get_order_by_index(&self, index: usize) -> Order {
        self.orders[self.sorting[index]].unwrap()
    }

    fn find_index_for_order(&self, order: Order) -> usize {
        let factor = match self.direction {
            Direction::Ask => 1.0,
            Direction::Bid => -1.0,
        };

        let price = order.price * factor;

        match self.sorting.binary_search_by(|index| {
            let other_order = self.orders[*index].unwrap();
            let other_price = other_order.price * factor;
            price.partial_cmp(&other_price).unwrap_or(Equal)
        }) {
            Ok(i) => i + 1,
            Err(i) => i,
        }
    }

    pub fn insert(&mut self, key: K, order: Order) -> Result<(), ()> {
        let sorting_index = self.find_index_for_order(order);

        // Insert before or after depending on direction
        let index = if let Some(i) = self.free_list.pop() {
            self.orders[i] = Some(order);
            i
        } else {
            self.orders.push(Some(order));
            self.orders.len() - 1
        };

        self.sorting.insert(sorting_index, index);
        self.map.insert(key.clone(), index);
        self.inverse_map.insert(index, key);

        return Ok(());
    }

    pub fn remove(&mut self, key: &K) {
        if let Some(&index) = self.map.get(key) {
            let mut sorting_index = 0;

            for i in 0..self.sorting.len() {
                if self.sorting[i] == index {
                    sorting_index = i;
                    break;
                }
            }

            self.sorting.remove(sorting_index);
            self.orders[index] = None;
            {
                let key = &self.inverse_map[&index];
                self.map.remove(&key);
            }
            self.inverse_map.remove(&index);
            self.free_list.push(index);
        }
    }
}


pub struct OrderbookIterator<'a, K>
where
    K: 'a + Clone + Hash + Eq
{
    orderbook: &'a Orderbook<K>,
    index: usize,
}

impl<'a, K> IntoIterator for &'a Orderbook<K>
where
    K: 'a + Clone + Hash + Eq
{
    type Item = Order;
    type IntoIter = OrderbookIterator<'a, K>;

    fn into_iter(self) -> Self::IntoIter {
        OrderbookIterator {
            orderbook: &self,
            index: 0,
        }
    }
}

impl<'a, K> Iterator for OrderbookIterator<'a, K>
where
    K: 'a + Clone + Hash + Eq
{
    type Item = Order;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.orderbook.len() {
            let index = self.index;
            self.index += 1;
            Some(self.orderbook.get_order_by_index(index))
        } else {
            None
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut ob = Orderbook::new(Direction::Bid);

        ob.insert(
            "first",
            Order {
                price: 10.0,
                volume: 10.0,
            },
        );
        ob.insert(
            "second",
            Order {
                price: 50.0,
                volume: 10.0,
            },
        );
        ob.insert(
            "third",
            Order {
                price: 5.0,
                volume: 10.0,
            },
        );
        ob.insert(
            "fourth",
            Order {
                price: 20.0,
                volume: 10.0,
            },
        );
        ob.insert(
            "fifth",
            Order {
                price: 13.0,
                volume: 10.0,
            },
        );

        ob.remove(&"fourth");

        for order in ob.into_iter() {
            println!("{:?}", order);
        }
    }
}
