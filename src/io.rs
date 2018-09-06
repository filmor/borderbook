use std::fmt::{Display, Formatter, Error};
use std::hash::Hash;
use {Orderbook, Order, Direction, Side};


pub fn parse_orderbook<S: Into<String>>(s: S) -> Orderbook<String> {
    let mut res = Orderbook::new();

    for line in s.into().lines() {
        let line: Vec<_> = line.split(";").collect();

        let order = Order {
            volume: line[2].trim().parse().unwrap(),
            price: line[3].trim().parse().unwrap(),
        };

        let side =
            match line[0].trim() {
                "a" => Direction::Ask,
                "b" => Direction::Bid,
                &_ => panic!("AAAH"),
            };

        let _ = res.insert(line[1].trim().to_string(), (side, order));
    }

    res
}


impl<K: Clone + Eq + Hash> Display for Side<K> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        try!(write!(fmt, "{}\n", self.direction));

        for order in self {
            try!(write!(fmt, "{}\t@\t{}\n", order.volume, order.price))
        }

        Ok(())
    }
}


impl<K: Clone + Eq + Hash> Display for Orderbook<K> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "{}\n\n{}", self.asks, self.bids)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let ob = parse_orderbook(
            "a; a; 10; 15\nb; b; 15; 20\na; c; 3.5; 10.1"
            );

        let (side, order) = ob.get_order(&"a".to_string()).unwrap();
        assert_eq!(order.volume, 10.);
        assert_eq!(order.price, 15.);
        assert_eq!(side, Direction::Ask);

        let (side, order) = ob.get_order(&"b".to_string()).unwrap();
        assert_eq!(order.volume, 15.);
        assert_eq!(order.price, 20.);
        assert_eq!(side, Direction::Bid);

        let (side, order) = ob.get_order(&"c".to_string()).unwrap();
        assert_eq!(order.volume, 3.5);
        assert_eq!(order.price, 10.1);
        assert_eq!(side, Direction::Ask);
    }

    #[test]
    fn format() {
        let ob = parse_orderbook(
            "b; a; 5; 10\nb; b; 15; 20\nb; c; 3.5; 10.1"
            );

        let formatted = format!("{}", ob);
        assert_eq!(
            formatted,
            "ask\n\n\nbid\n5\t@\t10\n3.5\t@\t10.1\n15\t@\t20\n"
            );

        // println!("{}", ob);
    }
}
