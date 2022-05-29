mod direction;
mod io;
mod matching;
mod order;
mod orderbook;
mod side;
mod timestamp;
mod trade;

pub use crate::direction::Direction;
pub use crate::io::parse_orderbook;
pub use crate::order::Order;
pub use crate::orderbook::Orderbook;
pub use crate::side::Side;
pub use crate::timestamp::Timestamp;
pub use crate::trade::Trade;

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
