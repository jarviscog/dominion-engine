

/// The scope for when this event should stop being listened for.
/// This does not guarantee that the TriggeredEvent will last this long, only that it will
/// automatically be destructed once it goes out of scope. 
/// Eg. Merchant will be destructed once you play a silver, so the effect does not happen twice
/// If you do not play a silver during your turn, then it will automatically be 
/// destructed at the end of your turn
#[derive(Debug, Clone)]
pub enum EventListenerDestructCondition {
    EndOfGame,
    EndOfActionPhase,
    EndOfBuyPhase,
    EndOfThisTurn,
    OneTime,
}

