
use super::*;

/// The answer to a Choice. Each Choice sent from the game has a corresponding Decision that
/// the player needs to return
pub enum Decision {
    PlayCard(Card), // Choose a card to play
    BuyCard(Card), // Choose a card to buy 
    /// Start the next phase of play. 
    /// e.g. ActionPhase -> BuyPhase
    /// BuyPhase -> NextPlayersTurn
    StartNextPhase,
    /// Choose a list of cards to transfer from one location to another
    /// `CardFilter` Any filters to limit the cards to choose from, including the type, name, or
    /// amount
    /// `Location` From
    /// `Location` To
    ChooseCards(
        CardFilter,
        Location, // From
        Location, // To
    ),
    /// Decide on a card to name
    NameACard(Card),
}
