use {Orderbook, Order, Direction};
use std::hash::Hash;
use trade::Trade;
use std::iter::Peekable;

pub fn match_sides<K: Hash + Eq + Clone>(ob: &mut Orderbook<K>)
-> Vec<Trade<K>>
{
    let ref mut ask = ob.asks;
    let ref mut bid = ob.bids;

    let mut ask_iter = ask.into_iter().peekable();
    let mut bid_iter = bid.into_iter().peekable();

    let mut res = vec![];

    loop {
        // Remove orders as we go?
        let mut ask_or_bid = false;
        if let Some(t) = match_single_order(ask_iter.peek(), Direction::Ask, &mut bid_iter) {
            ask_iter.next();
            res.push(t);
            ask_or_bid = true;
        }

        if let Some(t) = match_single_order(bid_iter.peek(), Direction::Bid, &mut ask_iter) {
            bid_iter.next();
            res.push(t);
            ask_or_bid = true;
        }

        if !ask_or_bid {
            break;
        }
    }

    res
}


fn match_single_order<I, K: Hash + Eq + Clone>(
    order: Option<&Order>, direction: Direction, iter: &mut Peekable<I>
    )
    -> Option<Trade<K>>
    where I: Iterator<Item=Order>
{
    if let Some(order) = order {
        if let Some(other_order) = iter.peek() {
            let _ = order;
            let _ = other_order; () // TODO
        }
    }

    None
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_sides() {
    }
}
