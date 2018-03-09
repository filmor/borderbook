mod order;
mod orderbook;
mod trade;
mod matching;

pub use order::{Order, Direction};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
