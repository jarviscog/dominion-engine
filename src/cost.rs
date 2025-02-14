
// TODO The cost of a coin/debt should be a RuntimeValue
//  this way cards can increase/decrease during runtime
// The cost of a card or other resource
#[derive(Debug, Clone)]
pub enum Cost {
    Coin(u8),
    Debt(u8),
}
