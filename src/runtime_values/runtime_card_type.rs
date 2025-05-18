

use std::fmt;

use crate::card::card_type::CardType;

#[derive(Debug, Clone)]
pub enum RuntimeCardType {
    Const(CardType),
    FromContext(String),
}

impl fmt::Display for RuntimeCardType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Const(card_type) => write!(f, "({})", card_type),
            Self::FromContext(str) => write!(f, "FromContext({})", str),
        }
    }
}
