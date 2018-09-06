use std::fmt::{Display, Formatter, Error};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Ask,
    Bid,
}

impl Display for Direction {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match self {
            Direction::Ask => write!(fmt, "ask"),
            Direction::Bid => write!(fmt, "bid"),
        }
    }
}
