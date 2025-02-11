
#[derive(Debug, Clone)]
pub enum ForEachType {
    Coin, // Coin value of the card
    CardCount, // Number of cards used in first step
    Card, // Use the card in the next step
}
