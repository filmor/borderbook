use std::collections::{HashMap};
use std::hash::Hash;

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Buy,
    Sell
}

#[derive(Clone, Copy, Debug)]
pub struct Order {
    pub price: i32,
    pub volume: i32,
}

impl Order {
    pub fn cost(&self) -> i64 {
        (self.price as i64) * (self.volume as i64)
    }
}


// One side of an orderbook
// Need:
// - Fast iteration up to value
pub struct Orderbook<K> {
    direction: Direction,
    map: HashMap<K, usize>,
    orders: Vec<Option<Order>>,
    sorting: Vec<usize>,

    free_list: Vec<usize>
}

impl<K: Hash + Eq> Orderbook<K> {
    pub fn new(dir: Direction) -> Self {
        Self {
            direction: dir,
            map: Default::default(),
            orders: Default::default(),
            sorting: Default::default(),

            free_list: Default::default()
        }
    }

    pub fn get_order(&self, key: &K) -> Option<Order> {
        if let Some(&res) = self.map.get(key) {
            self.orders[res]
        }
        else {
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
            Direction::Buy => { 1 }
            Direction::Sell => { -1 }
        };

        let price = order.price * factor;

        match self.sorting.binary_search_by(
            |index| {
                let other_order = self.orders[*index].unwrap();
                let other_price = other_order.price * factor;
                price.cmp(&other_price)
            }
            ) {
            Ok(i) => { i + 1 }
            Err(i) => { i }
        }
    }

    pub fn insert(&mut self, key: K, order: Order) {
        let sorting_index = self.find_index_for_order(order);

        // Insert before or after depending on direction
        let index =
            if let Some(i) = self.free_list.pop() {
                self.orders[i] = Some(order);
                i
            }
            else {
                self.orders.push(Some(order));
                self.orders.len() - 1
            };

        println!("Inserting {:?} at {}", order, sorting_index);

        self.sorting.insert(sorting_index, index);
        self.map.insert(key, index);

        println!("Sorting: {:?}\nOrders: {:?}", self.sorting, self.orders);
    }

    pub fn remove(&mut self, key: &K) {
        if let Some(&index) = self.map.get(key) {
            let order = self.orders[index].unwrap();

            let sorting_index = self.find_index_for_order(order);

            self.sorting.remove(sorting_index);
            self.orders[index] = None;
            self.free_list.push(index);
        }
    }
}


pub struct OrderbookIterator<'a, K: Hash + Eq> where K: 'a {
    orderbook: &'a Orderbook<K>,
    index: usize
}


impl<'a, K: Hash + Eq> IntoIterator for &'a Orderbook<K> {
    type Item = Order;
    type IntoIter = OrderbookIterator<'a, K>;

    fn into_iter(self) -> Self::IntoIter {
        OrderbookIterator {
            orderbook: &self,
            index: 0
        }
    }
}

impl<'a, K: Hash + Eq> Iterator for OrderbookIterator<'a, K> {
    type Item = Order;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.orderbook.len() {
            let index = self.index;
            self.index += 1;
            Some(self.orderbook.get_order_by_index(index))
        }
        else {
            None
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut ob = Orderbook::new(Direction::Sell);

        ob.insert("first", Order { price: 10, volume: 10 });
        ob.insert("second", Order { price: 50, volume: 10 });
        ob.insert("third", Order { price: 5, volume: 10 });
        ob.insert("fourth", Order { price: 20, volume: 10 });
        ob.insert("fifth", Order { price: 13, volume: 10 });

        for order in ob.into_iter() {
            println!("{:?}", order);
        }
    }
}
