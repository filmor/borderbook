use std::fmt::{Display, Formatter, Error};
use std::hash::Hash;
use {Orderbook, Order, Direction};


pub fn parse_orderbook<S: Into<String>>(direction: Direction, s: S) -> Orderbook<String> {
    let mut res = Orderbook::new(direction);

    for line in s.into().lines() {
        let line: Vec<_> = line.split(";").collect();

        let order = Order {
            volume: line[1].trim().parse().unwrap(),
            price: line[2].trim().parse().unwrap(),
        };

        let _ = res.insert(line[0].trim().to_string(), order);
    }

    res
}


impl<K: Clone + Eq + Hash> Display for Orderbook<K> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        try!(write!(fmt, "{}\n", self.direction));

        for order in self {
            try!(write!(fmt, "{}\t@\t{}\n", order.volume, order.price))
        }

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let ob = parse_orderbook(
            Direction::Ask,
            "a; 10; 15\nb; 15; 20\nc; 3.5; 10.1"
            );

        let a = ob.get_order(&"a".to_string()).unwrap();
        assert_eq!(a.volume, 10.);
        assert_eq!(a.price, 15.);

        let b = ob.get_order(&"b".to_string()).unwrap();
        assert_eq!(b.volume, 15.);
        assert_eq!(b.price, 20.);

        let c = ob.get_order(&"c".to_string()).unwrap();
        assert_eq!(c.volume, 3.5);
        assert_eq!(c.price, 10.1);
    }

    #[test]
    fn format() {
        let ob = parse_orderbook(
            Direction::Bid,
            "a; 5; 10\nb; 15; 20\nc; 3.5; 10.1"
            );

        let formatted = format!("{}", ob);
        assert_eq!(
            formatted,
            "bid\n5\t@\t10\n3.5\t@\t10.1\n15\t@\t20\n"
            );
    }
}
