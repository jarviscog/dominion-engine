

pub enum ActionType {
    // Player events
    PlusCard(u8),
    PlusAction(u8),
    PlusBuy(u8),

    TrashCard(u8),
    DiscardCard(u8),

    GainCard(),

    SearchDeck(u8), // Search through the top n cards of a deck, with some potentially being returned
}

