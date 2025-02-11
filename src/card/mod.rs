use std::str::FromStr;


use crate::card_type::CardType;
use crate::cost::Cost;
use crate::expansion::Expansion;

use crate::step::*;

use crate::step::location::Location;
use crate::step::effected_players::EffectedPlayers;
use crate::step::foreach_type::ForEachType;
use crate::step::card_filter::CardFilter;
use crate::step::runtime_value::RuntimeValue;

// functions to generate cards
mod dominion;
mod seaside;

#[derive(Debug, Clone)]
pub struct Card {
    name: String,
    expansion: Expansion,
    card_type: Vec<CardType>,
    cost: Cost,
    treasure_value: u8,
    action_steps: Option<Vec<Step>>, // If a card is an action card
}

impl Card {

    pub fn get_steps(&self) -> Option<Vec<Step>> {
        self.action_steps.clone()
    }

    pub fn copper() -> Card {
        Card {
            name: "Copper".to_owned(),
            expansion: Expansion::BaseGame,
            card_type: vec![CardType::Treasure],
            cost: Cost::Coin(0),
            treasure_value: 1,
            action_steps: None,
        }
    }

    pub fn silver() -> Card {
        Card {
            name: "Silver".to_owned(),
            expansion: Expansion::BaseGame,
            card_type: vec![CardType::Treasure],
            cost: Cost::Coin(3),
            treasure_value: 2,
            action_steps: None,
        }
    }

    pub fn gold() -> Card {
        Card {
            name: "Gold".to_owned(),
            expansion: Expansion::BaseGame,
            card_type: vec![CardType::Treasure],
            cost: Cost::Coin(6),
            treasure_value: 3,
            action_steps: None,
        }
    }

    pub fn estate() -> Card {
        Card {
            name: "Estate".to_owned(),
            expansion: Expansion::BaseGame,
            card_type: vec![CardType::Victory(1)],
            cost: Cost::Coin(2),
            treasure_value: 0,
            action_steps: None,
        }
    }

    pub fn duchy() -> Card {
        Card {
            name: "Duchy".to_owned(),
            expansion: Expansion::BaseGame,
            card_type: vec![CardType::Victory(3)],
            cost: Cost::Coin(5),
            treasure_value: 0,
            action_steps: None,
        }
    }

    pub fn province() -> Card {
        Card {
            name: "Province".to_owned(),
            expansion: Expansion::BaseGame,
            card_type: vec![CardType::Victory(6)],
            cost: Cost::Coin(8),
            treasure_value: 0,
            action_steps: None,
        }
    }


}




