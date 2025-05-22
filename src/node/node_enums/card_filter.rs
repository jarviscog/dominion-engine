use std::fmt::{self, Write};

use crate::{card_type::CardType, runtime_values::RuntimeI32};

use super::*;
use crate::cost::Cost;

// Used as a filter for certain steps to ensure only some options are allowed
#[derive(Debug, Clone)]
pub enum CardFilter {
    All,
    Name(String),
    NotName(String),
    Type(CardType),
    NotType(CardType),

    CoinCostUpto(RuntimeI32),
    CoinCostEquals(RuntimeI32),
    ValueUpto(Cost),
    ValueEquals(Cost),

    // If no CardCount is specified in the filters, a default value of 1 will be used
    CardCountUpto(RuntimeI32),
    CardCountEquals(RuntimeI32),
    CardCountAll, // Move all cards

    ThisCard, // Some cards say 'trash this card' or likewise

    // Verify the From Location always contains a certain number of cards. This is used in militia
    DownTo(RuntimeI32),

    // This is commented out for a reason. Please add values elsewhere
    //UpToXMoreValue(RuntimeValue, RuntimeValue), // (x, y) Up to x more value than y card

    // Used in steps to get the card passed down from above. For example, a for each
    // will pass the value from the previous step
    FromAbove, // TODO what value do we grab from above? Is it a cost, count, or something else
    // TODO This makes less sense here. Maybe move it?
    NextCardPlayed, // Throne Room
}

impl fmt::Display for CardFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::All => write!(f, ""),
            Self::CoinCostUpto(x) => write!(f, "Cost <= {}", x),
            Self::CoinCostEquals(x) => write!(f, "Cost == {}", x),
            Self::ValueUpto(x) => write!(f, "Value <= {}", x),
            Self::ValueEquals(x) => write!(f, "Value == {}", x),
            Self::CardCountUpto(x) => write!(f, "Count <= {}", x),
            Self::CardCountEquals(x) => write!(f, "Count == {}", x),
            Self::CardCountAll => write!(f, "All cards"),
            Self::ThisCard => write!(f, "This card"),
            Self::Type(x) => write!(f, "Type == {}", x),
            Self::NotType(x) => write!(f, "Type != {}", x),
            Self::Name(x) => write!(f, "Name == {}", x),
            Self::NotName(x) => write!(f, "Name != {}", x),
            _ => write!(f, "{:?}", self),
        }
    }
}
