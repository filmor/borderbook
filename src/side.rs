use crate::{Direction, Order};

use std::cmp::{min, Ordering::Equal};
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

// One side of an orderbook
// Need:
// - Fast iteration up to value
pub struct Side<K>
where
    K: Hash + Eq + Clone,
{
    pub direction: Direction,

    map: HashMap<K, usize>,
    inverse_map: HashMap<usize, K>,

    orders: Vec<Option<Order>>,
    sorting: VecDeque<usize>,

    free_list: Vec<usize>,
}

impl<K: Hash + Eq + Clone> Side<K> {
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

    pub fn get_key(&self, position: usize) -> &K {
        &self.inverse_map[&self.sorting[position]]
    }

    pub fn len(&self) -> usize {
        self.sorting.len()
    }

    pub fn is_empty(&self) -> bool {
        self.sorting.is_empty()
    }

    pub fn get_order_by_index(&self, index: usize) -> Option<Order> {
        self.orders[self.sorting[index]]
    }

    fn find_index_for_order(&self, order: Order) -> usize {
        let factor = match self.direction {
            Direction::Ask => 1.0,
            Direction::Bid => -1.0,
        };

        let price = order.price * factor;

        match deque_binary_search_by(&self.sorting, |index| {
            let other_order = self.orders[*index].unwrap();
            let other_price = other_order.price * factor;
            price.partial_cmp(&other_price).unwrap_or(Equal)
        }) {
            Ok(i) => i + 1,
            Err(i) => i,
        }
    }

    pub fn insert(&mut self, key: K, order: Order) -> usize {
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

        sorting_index
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

    pub fn remove_first_n(&mut self, n: usize) {
        let n = min(n, self.len());
        for i in 0..n {
            let index = self.sorting[i];
            let key = &self.inverse_map[&index];
            self.map.remove(&key);
            self.orders[index] = None;
            self.free_list.push(index);
        }
        for _ in 0..n {
            self.sorting.pop_front();
        }
    }

    pub fn set_first_volume(&mut self, volume: f64) {
        if self.is_empty() {
            return;
        }

        if let Some(ref mut order) = self.orders[self.sorting[0]] {
            order.volume = volume;
        }
    }
}

pub struct SideIterator<'a, K>
where
    K: 'a + Clone + Hash + Eq,
{
    side: &'a Side<K>,
    index: usize,
}

impl<'a, K> IntoIterator for &'a Side<K>
where
    K: 'a + Clone + Hash + Eq,
{
    type Item = (&'a K, Order);
    type IntoIter = SideIterator<'a, K>;

    fn into_iter(self) -> Self::IntoIter {
        SideIterator {
            side: &self,
            index: 0,
        }
    }
}

impl<'a, K> Iterator for SideIterator<'a, K>
where
    K: 'a + Clone + Hash + Eq,
{
    type Item = (&'a K, Order);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.side.len() {
            let index = self.index;
            self.index += 1;
            self.side
                .get_order_by_index(index)
                .map(|order| (self.side.get_key(index), order))
        } else {
            None
        }
    }
}

use std::cmp::Ordering;

fn deque_binary_search_by<'a, T, F>(q: &'a VecDeque<T>, f: F) -> Result<usize, usize>
where
    F: Clone + FnMut(&'a T) -> Ordering,
{
    let (first, second) = q.as_slices();
    if let Ok(res) = first.binary_search_by(f.clone()) {
        Ok(res)
    } else {
        let first_len = first.len();
        match second.binary_search_by(f) {
            Ok(res) => Ok(res + first_len),
            Err(res) => Err(res + first_len),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut ob = Side::new(Direction::Bid);

        ob.insert("first", Order::new(10., 10.));
        ob.insert("second", Order::new(50., 10.));
        ob.insert("third", Order::new(5., 10.));
        ob.insert("fourth", Order::new(20., 10.));
        ob.insert("fifth", Order::new(13., 10.));

        ob.remove(&"fourth");

        for order in ob.into_iter() {
            println!("{:?}", order);
        }
    }

    #[test]
    fn test_set_first_volume() {
        let mut side = Side::<u32>::new(Direction::Ask);

        side.set_first_volume(5.0);

        assert_eq!(side.len(), 0);

        side.insert(0, Order::new(10., 1.));

        assert_eq!(side.len(), 1);

        let first = side.into_iter().next().unwrap().1;
        assert_eq!(first.volume, 1.0);

        side.set_first_volume(2.0);
        let first = side.into_iter().next().unwrap().1;
        assert_eq!(first.volume, 2.0);
    }
}
