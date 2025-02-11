
// The cost of a card or other resource
#[derive(Debug, Clone)]
pub enum Cost {
    Coin(u8),
    Debt(u8),
}
