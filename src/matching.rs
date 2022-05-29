use std::cmp::Ordering;
use std::hash::Hash;
use crate::trade::Trade;
use crate::{Direction, Order, Orderbook};

pub fn match_sides<K: Hash + Eq + Clone>(ob: &mut Orderbook<K>) -> Vec<Trade<K>> {
    let mut res = vec![];

    let mut asks_dropped = 0;
    let mut bids_dropped = 0;

    let mut ask_modified = None;
    let mut bid_modified = None;

    {
        let mut ask_iter = ob.asks.into_iter();
        let mut bid_iter = ob.bids.into_iter();

        let mut ask = ask_iter.next();
        let mut bid = bid_iter.next();

        loop {
            let (ask_key, mut ask_order) = if let Some(val) = ask { val } else { break };

            let (bid_key, mut bid_order) = if let Some(val) = bid { val } else { break };

            macro_rules! push_trade {
                ($vol:ident) => {
                    let (ts, aggressor, price) =
                        match Ord::cmp(&ask_order.timestamp, &bid_order.timestamp) {
                            Ordering::Less => {
                                (bid_order.timestamp, Some(Direction::Ask), ask_order.price)
                            }
                            Ordering::Greater => {
                                (ask_order.timestamp, Some(Direction::Bid), bid_order.price)
                            }
                            Ordering::Equal => (
                                ask_order.timestamp,
                                None,
                                (ask_order.price + bid_order.price) / 2.0,
                            ),
                        };

                    res.push(Trade {
                        buy_key: bid_key.clone(),
                        sell_key: ask_key.clone(),
                        price: price,
                        volume: $vol,
                        // TODO:
                        timestamp: ts,
                        aggressor_side: aggressor,
                    });
                };
            }

            match match_orders(&bid_order, &ask_order) {
                MatchResult::Unmatched => break,
                MatchResult::Partial {
                    left: Direction::Ask,
                    volume,
                } => {
                    ask_order.volume -= volume;
                    ask_modified = Some(ask_order.volume);

                    push_trade!(volume);

                    bid = bid_iter.next();
                    bids_dropped += 1;
                }
                MatchResult::Partial {
                    left: Direction::Bid,
                    volume,
                } => {
                    bid_order.volume -= volume;
                    bid_modified = Some(bid_order.volume);

                    push_trade!(volume);

                    ask = ask_iter.next();
                    asks_dropped += 1;
                }
                MatchResult::Full { volume } => {
                    asks_dropped += 1;
                    bids_dropped += 1;

                    push_trade!(volume);

                    ask = ask_iter.next();
                    bid = bid_iter.next();
                }
            }

            if ask_order.volume == 0.0 {
                asks_dropped += 1;
                ask = ask_iter.next();
            }

            if bid_order.volume == 0.0 {
                bids_dropped += 1;
                bid = bid_iter.next();
            }
        }
    }

    ob.asks.remove_first_n(asks_dropped);
    ob.bids.remove_first_n(bids_dropped);

    if let Some(vol) = ask_modified {
        ob.asks.set_first_volume(vol)
    }
    if let Some(vol) = bid_modified {
        ob.bids.set_first_volume(vol)
    }

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
    },
}

fn match_orders(bid: &Order, ask: &Order) -> MatchResult {
    if ask.price > bid.price {
        return MatchResult::Unmatched;
    }

    assert!(bid.volume > 0.0);
    assert!(ask.volume > 0.0);

    match ask.volume.partial_cmp(&bid.volume) {
        None => panic!("Volume is NaN"),
        Some(Ordering::Less) => MatchResult::Partial {
            left: Direction::Bid,
            volume: ask.volume,
        },
        Some(Ordering::Greater) => MatchResult::Partial {
            left: Direction::Ask,
            volume: bid.volume,
        },
        Some(Ordering::Equal) => MatchResult::Full { volume: ask.volume },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_order_match() {
        let a = Order::new(5., 10.);
        let b = Order::new(10., 5.);
        let c = Order::new(5., 5.);
        let d = Order::new(10., 10.);

        assert_eq!(match_orders(&a, &b), MatchResult::Unmatched);
        assert_eq!(
            match_orders(&b, &a),
            MatchResult::Partial {
                left: Direction::Ask,
                volume: 5.
            }
        );

        assert_eq!(match_orders(&a, &d), MatchResult::Unmatched);
        assert_eq!(match_orders(&d, &a), MatchResult::Full { volume: 10. });

        assert_eq!(
            match_orders(&a, &c),
            MatchResult::Partial {
                left: Direction::Bid,
                volume: 5.
            }
        );
    }
}
