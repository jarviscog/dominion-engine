use std::fmt;

// These are values that potentially get resolved at runtime. Always return a integer
#[derive(Debug, Clone)]
pub enum RuntimeValue {
    FixedValue(u8),
    NumberOfEmptySupplyPiles, // For Poacher
    NumberOfCardsInDeck, // For Gardens
}


impl fmt::Display for RuntimeValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FixedValue(x) => write!(f, "{}", x),
            Self::NumberOfCardsInDeck => write!(f, "NumberOfCardsInDeck"),
            Self::NumberOfEmptySupplyPiles => write!(f, "NumberOfEmptySupplyPiles"),

        }
    }
            //write!(f, "{:?}", self)
}
