// NOTE: Keeping this around for now for legacy reasons, but this code is dead now in favor of the
// ECS

use crate::card;

#[derive(Debug, Clone)]
pub enum GameStep {


    // Got a good idea for this:
    // Make some sort of system where a step has a 'target', conditions, and a destination 
    // Target is the place to target:
    //   this could be your deck, the other players deck, the other players hand, the top 3 cards of
    //   your deck...etc.
    // Conditions
    //   the conditions of the operation. this could be minimum(2) or 
    // Destination: 
    //   where to put the cards after the operation is done
    // Deferred actions

}
