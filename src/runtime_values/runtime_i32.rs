use std::fmt;

use crate::card::Location;
use crate::CardFilter;

#[derive(Debug, Clone)]
pub enum RuntimeI32 {
    Any, // TODO Do I still need Any?
    Const(i32),
    CostFromContext(String),
    TreasureValueFromContext(String),
    NumberOfEmptySupplyPiles, // For Poacher
    NumberOfCardsInDeck,      // For Gardens
    NumberOfCardsInLocation(Location, Option<Vec<CardFilter>>),
    Add(Box<RuntimeI32>, Box<RuntimeI32>),
    Mult(Box<RuntimeI32>, Box<RuntimeI32>),
}

impl fmt::Display for RuntimeI32 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Any => write!(f, "Any"),
            Self::Const(x) => write!(f, "{}", x),
            Self::CostFromContext(x) => write!(f, "CostFromContext({})", x),
            Self::TreasureValueFromContext(x) => write!(f, "TreasureValueFromContext({})", x),
            Self::NumberOfCardsInDeck => write!(f, "NumberOfCardsInDeck"),
            Self::NumberOfEmptySupplyPiles => write!(f, "NumberOfEmptySupplyPiles"),
            Self::NumberOfCardsInLocation(loc, filters) => write!(f, "NumberOfCardsIn({})", loc),
            Self::Add(x, y) => write!(f, "({} + {})", x, y),
            Self::Mult(x, y) => write!(f, "({} * {})", x, y),
        }
    }
}
