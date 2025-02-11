use std::fmt;

// These are values that potentially get resolved at runtime
#[derive(Debug, Clone)]
pub enum RuntimeValue {
    Any,
    Unlimited,
    FixedValue(u8),
    NumberOfEmptySupplyPiles, // For Poacher
    NumberOfCardsInDeck, // For Gardens
    FromAbove,
}


impl fmt::Display for RuntimeValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Any => write!(f, "Any"),
            Self::Unlimited => write!(f, "∞"),
            Self::FixedValue(x) => write!(f, "{}", x),
            Self::NumberOfCardsInDeck => write!(f, "NumberOfCardsInDeck"),
            Self::FromAbove => write!(f, "FromAbove"),
            Self::NumberOfEmptySupplyPiles => write!(f, "NumberOfEmptySupplyPiles"),

        }
    }
            //write!(f, "{:?}", self)
}
