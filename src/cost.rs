use std::fmt::{self, Write};

// TODO The cost of a coin/debt should be a RuntimeValue
//  this way cards can increase/decrease during runtime
// The cost of a card or other resource
#[derive(Debug, Clone)]
pub enum Cost {
    Coin(u32),
    Debt(u32),
    Potion(u32),
}

impl fmt::Display for Cost {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Coin(x) => write!(f, "{}🪙", x),
            Self::Debt(x) => write!(f, "{}💸", x),
            Self::Potion(x) => write!(f, "{}🧪", x),
        }
    }
}
