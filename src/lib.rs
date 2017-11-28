mod order;
mod orderbook;

pub use order::{Order, Direction};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
