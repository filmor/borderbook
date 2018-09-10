mod direction;
mod io;
mod matching;
mod order;
mod orderbook;
mod side;
mod trade;

pub use direction::Direction;
pub use io::parse_orderbook;
pub use order::Order;
pub use orderbook::Orderbook;
pub use side::Side;
pub use trade::Trade;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matching() {
        let mut ob = parse_orderbook(
            "b; order1; 10; 10
             a; order2; 5; 10
             a; order3; 10; 5",
        );

        println!("{}", ob);

        let trades = ob.resolve_matches();

        // assert_eq!(trades.len(), 2);

        for t in trades {
            println!("{:?}", t)
        }

        println!("{}", ob);
    }
}
