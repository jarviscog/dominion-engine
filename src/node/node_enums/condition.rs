use std::fmt;
use super::*;
use crate::runtime_values::*;

/// Condition used for `Step::RepeatUntil`
#[derive(Debug, Clone)]
pub enum Condition {

    EqualI32(RuntimeI32, RuntimeI32),
    EqualCardName(RuntimeCardName, RuntimeCardName),
    EqualCardType(RuntimeCardType, RuntimeCardType),
    /// Check if a stack contains a certain number of cards
    EqualStackSize(Location, RuntimeI32),
    ContextContainsCard(String),
    NotInBuyPhase
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::EqualI32(x, y) => write!(f, "({} == {})", x, y),
            Self::EqualCardName(x, y) => write!(f, "({} == {})", x, y),
            Self::EqualCardType(x, y) => write!(f, "({} == {})", x, y),
            Self::EqualStackSize(x, y) => write!(f, "(Size({}) == {})", x, y),
            Self::ContextContainsCard(context) => write!(f, "(\"{}\" has card)", context),
            Self::NotInBuyPhase => write!(f, "(!BuyPhase)"),
        }
    }
}
