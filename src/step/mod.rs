
use std::fmt;

pub mod location;
use location::Location;

pub mod effected_players;
use effected_players::EffectedPlayers;

pub mod foreach_type;
use foreach_type::ForEachType;

pub mod runtime_value;
use runtime_value::RuntimeValue;

pub mod card_filter;
use card_filter::CardFilter;

#[derive(Debug, Clone)]
pub enum Step {

    // Simple Events
    PlusCoin(RuntimeValue),
    PlusAction(RuntimeValue),
    PlusBuy(RuntimeValue),
    DrawCard(RuntimeValue),

    // For each of one step, do another step. This could be based on the number of cards you use,
    // the amount, or something else
    ForEach(ForEachType, Box<Step>, Box<Step>),

    OptionalMoveXCards( // Move cards from one location to another
        EffectedPlayers, 
        CardFilter, 
        Location, // From
        Location, // To
        RuntimeValue // Limit
    ), 

    ForceMoveXCards( // Move cards from one location to another
        EffectedPlayers, 
        CardFilter, 
        Location, // From
        Location, // To
        RuntimeValue // Count
    ), 

    //// Operators
    //And(Box<Step>, Box<Step>),
    //Or(Box<Step>, Box<Step>),
//
//
    //DrawTo(number::Number),
    //DiscardTo(number::Number),
//
    //LimitCardCount(Box<Step>, number::Number),
    //LimitCardName(Box<Step>, number::Number),
    //LimitCardValue(Box<Step>, number::Number),

}


impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::PlusBuy(x) => write!(f, "+{} Buy", x),
            Self::DrawCard(x) => write!(f, "+{} Card", x),
            Self::PlusAction(x) => write!(f, "+{} Action", x),
            Self::PlusCoin(x) => write!(f, "+{} 🪙", x),
            _ => write!(f, "{:?}", self),

        }
    }
            //write!(f, "{:?}", self)
}


