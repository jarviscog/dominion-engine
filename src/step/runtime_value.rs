
// These are values that potentially get resolved at runtime. Always return a integer
#[derive(Debug, Clone)]
pub enum RuntimeValue {
    FixedValue(u8),
    NumberOfEmptySupplyPiles, // For Poacher
    NumberOfCardsInDeck, // For Gardens
}
