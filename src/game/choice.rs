use super::*;

/// Any choice a player needs to make
pub enum Choice {
    /// The game is currently in the action phase
    /// Choose to play a card, or enter buy phase
    ActionPhase,
    /// The game is currently in the buy phase
    /// Choose to play a card, or go to next turn
    BuyPhase,
    /// Choose a list of cards to transfer from one location to another
    /// The cards must be in that location
    ChooseCards(
        CardFilter,
        Location, // From
        Location, // To
    ),
    NameACard(Option<CardFilter>),
}
