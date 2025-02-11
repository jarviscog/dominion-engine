use crate::card_type::CardType;

use super::*;
use crate::cost::Cost;

// Used as a filter for certain steps to ensure only some options are allowed
#[derive(Debug, Clone)]
pub enum CardFilter {
    All,
    Name(String),
    Type(CardType),
    NotType(CardType),


    CoinCostUpto(RuntimeValue),
    CoinCostEquals(RuntimeValue),
    ValueUpto(Cost),
    ValueEquals(Cost),

    // If no CardCount is specified in the filters, a default value of 1 will be used
    CardCountUpto(RuntimeValue),
    CardCountEquals(RuntimeValue),
    CardCountAll, // Move all cards

    ThisCard, // Some cards say 'trash this card' or likewise

    // Verify the From Location always contains a certain number of cards. This is used in militia
    DownTo(RuntimeValue),

    // This is commented out for a reason. Please add values elsewhere
    //UpToXMoreValue(RuntimeValue, RuntimeValue), // (x, y) Up to x more value than y card
    
    // Used in steps to get the card passed down from above. For example, a for each
    // will pass the value from the previous step
    FromAbove, // TODO what value do we grab from above? Is it a cost, count, or something else
    // TODO This makes less sense here. Maybe move it?
    NextCardPlayed, // Throne Room
}
