

pub enum Step {
    // Global events
    NextTurn(u8), 
    PlayCard(),

    // Player events
    PlusCard(u8), 
    PlusAction(u8),
    PlusBuy(u8),
    TrashCard(u8), 

    SearchDeck(u8) // Search through the top of a deck, with some potentially being returned


    // Deferred actions

}
