use std::str::FromStr;


use crate::cost::Cost;
use crate::expansion::Expansion;
use super::card_type::CardType;

use crate::step::*;

// functions to generate cards
pub mod dominion;
pub mod intrigue;
pub mod seaside;

#[derive(Debug, Clone)]
pub struct Card {
    name: String,
    expansion: Expansion,
    card_type: Vec<CardType>,
    on_gain: Option<Step>, // If steps are required when you gain the card
    cost: Cost,
}

impl Card {

    pub fn get_steps(&self) -> Option<Vec<Step>> {
        for c_type in &self.card_type {
            return match c_type {
                CardType::Action(steps) => {Some(steps.clone())}
                _ => None
            }
        }
        None
    }

    pub fn copper() -> Card {
        Card {
            name: "Copper".to_owned(),
            expansion: Expansion::Dominion,
            card_type: vec![CardType::Treasure(RuntimeValue::FixedValue(1))],
            cost: Cost::Coin(0),
            on_gain: None,
        }
    }

    pub fn silver() -> Card {
        Card {
            name: "Silver".to_owned(),
            expansion: Expansion::Dominion,
            card_type: vec![CardType::Treasure(RuntimeValue::FixedValue(2))],
            cost: Cost::Coin(3),
            on_gain: None,
        }
    }

    pub fn gold() -> Card {
        Card {
            name: "Gold".to_owned(),
            on_gain: None,
            expansion: Expansion::Dominion,
            card_type: vec![CardType::Treasure(RuntimeValue::FixedValue(3))],
            cost: Cost::Coin(6),
        }
    }

    pub fn estate() -> Card {
        Card {
            name: "Estate".to_owned(),
            expansion: Expansion::Dominion,
            card_type: vec![CardType::Victory(RuntimeValue::FixedValue(1))],
            cost: Cost::Coin(2),
            on_gain: None,
        }
    }

    pub fn duchy() -> Card {
        Card {
            name: "Duchy".to_owned(),
            expansion: Expansion::Dominion,
            card_type: vec![CardType::Victory(RuntimeValue::FixedValue(3))],
            cost: Cost::Coin(5),
            on_gain: None,
        }
    }

    pub fn province() -> Card {
        Card {
            name: "Province".to_owned(),
            expansion: Expansion::Dominion,
            card_type: vec![CardType::Victory(RuntimeValue::FixedValue(6))],
            cost: Cost::Coin(8),
            on_gain: None,
        }
    }

    pub fn garden() -> Card {
        Card {
            name: "Garden".to_owned(),
            expansion: Expansion::Dominion,
            card_type: vec![
                CardType::Victory(RuntimeValue::NumberOfCardsInDeck)
            ],
            cost: Cost::Coin(8),
            on_gain: None,
        }
    }


}




