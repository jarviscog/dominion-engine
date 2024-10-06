use crate::card;

#[derive(Debug, Clone)]
pub enum GameStep {

    // Global events
    NextTurn(u8), // Player number 

    ActionPhase(),
    BuyPhase(),

    PlayCard(String), // Name of the card

    BuyCard(card::Card),
    // Housekeeping
    DrawToCardLimit(),

    // Player events
    PlusCard(u8),
    PlusAction(u8),
    PlusBuy(u8),

    TrashCard(u8),
    DiscardCard(u8),

    SearchDeck(u8), // Search through the top n cards of a deck, with some potentially being returned

    GameEnd(),

    // Deferred actions

}
