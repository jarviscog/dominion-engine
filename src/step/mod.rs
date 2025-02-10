

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


}
