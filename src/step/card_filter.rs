use crate::card_type::CardType;

use super::runtime_value::RuntimeValue;


// Used as a filter for certain steps to ensure only some options are allowed
#[derive(Debug, Clone)]
pub enum CardFilter {
    All,
    Name(String),
    Type(CardType),
    UpToValue(RuntimeValue),
    // Used in steps to get the card passed down from above. For example, a for each
    // will pass the value from the previous step
    FromAbove,
    // TODO This makes less sense here. Maybe move it?
    NextCardPlayed, // Throne Room
}
