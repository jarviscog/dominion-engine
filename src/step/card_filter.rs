use crate::card_type::CardType;


// Used as a filter for certain steps to ensure only some options are allowed
#[derive(Debug, Clone)]
pub enum CardFilter {
    All,
    Name(String),
    Type(CardType)
}
