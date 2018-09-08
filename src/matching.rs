use {Orderbook, Order, Direction};
use std::hash::Hash;
use std::cmp::Ordering;
use trade::Trade;


pub fn match_sides<K: Hash + Eq + Clone>(ob: &mut Orderbook<K>) -> Vec<Trade<K>>
{
    let mut res = vec![];

    macro_rules! next {
        ($iter:ident) =>
        {
            match ($iter).next() {
                Some(v) => v,
                None => return res
            }
        }
    }

    let mut asks_dropped = 0;
    let mut bids_dropped = 0;

    let mut ask_modified = None;
    let mut bid_modified = None;

    {
        let mut ask_iter = ob.asks.into_iter();
        let mut bid_iter = ob.bids.into_iter();

        let mut ask = next!(ask_iter);
        let mut bid = next!(bid_iter);

        macro_rules! push_trade {
            ($vol:ident) => {
                // TODO: Proper pricing logic, for this we need to store the
                //       age of the orders
                res.push(
                    Trade {
                        buy_key: bid.0.clone(),
                        sell_key: ask.0.clone(),
                        price: (ask.1.price + bid.1.price) / 2.0,
                        volume: $vol
                    }
                );
            }
        }

        loop {
            match match_orders(&bid.1, &ask.1) {
                MatchResult::Unmatched => break,
                MatchResult::Partial { left: Direction::Ask, volume } => {
                    ask.1.volume -= volume;
                    ask_modified = Some(ask.1.volume);

                    push_trade!(volume);
                    
                    bid = next!(bid_iter);
                    bids_dropped += 1;
                },
                MatchResult::Partial { left: Direction::Bid, volume } => {
                    bid.1.volume -= volume;
                    bid_modified = Some(bid.1.volume);

                    push_trade!(volume);

                    ask = next!(ask_iter);
                    asks_dropped += 1;
                },
                MatchResult::Full { volume } => {
                    asks_dropped += 1;
                    bids_dropped += 1;

                    push_trade!(volume);

                    ask = next!(ask_iter);
                    bid = next!(bid_iter);
                }
            }

            if ask.1.volume == 0.0 {
                asks_dropped += 1;
                ask = next!(ask_iter);
            }

            if bid.1.volume == 0.0 {
                bids_dropped += 1;
                bid = next!(bid_iter);
            }
        }
    }

    ob.asks.remove_first_n(asks_dropped);
    ob.bids.remove_first_n(bids_dropped);

    ask_modified.map(|vol| ob.asks.set_first_volume(vol));
    bid_modified.map(|vol| ob.bids.set_first_volume(vol));

    res
}


#[derive(Debug, PartialEq)]
enum MatchResult {
    Unmatched,
    Partial {
        // Side that still has a non-zero volume
        left: Direction,
        volume: f64,
    },
    Full {
        volume: f64,
    }
}


fn match_orders(bid: &Order, ask: &Order) -> MatchResult {
    if ask.price > bid.price {
        return MatchResult::Unmatched;
    }

    assert!(bid.volume > 0.0);
    assert!(ask.volume > 0.0);

    let res =
    match ask.volume.partial_cmp(&bid.volume) {
        None =>
            panic!("Volume is NaN"),
        Some(Ordering::Less) =>
            MatchResult::Partial { left: Direction::Bid, volume: ask.volume },
        Some(Ordering::Greater) =>
            MatchResult::Partial { left: Direction::Ask, volume: bid.volume },
        Some(Ordering::Equal) =>
            MatchResult::Full { volume: ask.volume }
    };

    // println!("{:?} + {:?} => {:?}", bid, ask, res);

    res
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_order_match() {
        let a = Order { price: 5., volume: 10. };
        let b = Order { price: 10., volume: 5. };
        let c = Order { price: 5., volume: 5. };
        let d = Order { price: 10., volume: 10. };

        assert_eq!(match_orders(&a, &b), MatchResult::Unmatched);
        assert_eq!(
            match_orders(&b, &a),
            MatchResult::Partial { left: Direction::Ask, volume: 5. }
        );

        assert_eq!(match_orders(&a, &d), MatchResult::Unmatched);
        assert_eq!(
            match_orders(&d, &a),
            MatchResult::Full { volume: 10. }
        );

        assert_eq!(
            match_orders(&a, &c),
            MatchResult::Partial { left: Direction::Bid, volume: 5. }
        );
    }
}
