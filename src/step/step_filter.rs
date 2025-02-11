use crate::card_type::CardType;


pub enum StepFilter {
    OnlyCardType(CardType),
    NotCardType(CardType),
}
