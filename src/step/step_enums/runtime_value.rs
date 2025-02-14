use std::fmt;

// These are values that potentially get resolved at runtime
#[derive(Debug, Clone)]
pub enum RuntimeValue {
    Any,
    FixedValue(u32),
    NumberOfEmptySupplyPiles, // For Poacher
    NumberOfCardsInDeck, // For Gardens
    FromAbove,
    Add(Box<RuntimeValue>, Box<RuntimeValue>),
    Mult(Box<RuntimeValue>, Box<RuntimeValue>),
}


impl fmt::Display for RuntimeValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Any => write!(f, "Any"),
            Self::FixedValue(x) => write!(f, "{}", x),
            Self::NumberOfCardsInDeck => write!(f, "NumberOfCardsInDeck"),
            Self::FromAbove => write!(f, "FromAbove"),
            Self::NumberOfEmptySupplyPiles => write!(f, "NumberOfEmptySupplyPiles"),
            Self::Add(x, y) => write!(f, "({} + {})", x, y),
            Self::Mult(x, y) => write!(f, "({} * {})", x, y),
        }
    }
            //write!(f, "{:?}", self)
}
