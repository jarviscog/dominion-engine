use std::collections::HashMap;
use crate::card::{self, base_game};

pub mod base_game;

struct Bank {
    // TODO this should contain a list of treasure, victory points, and actions, and have funtions
    // for returning 
    action: HashMap<Card, u32>
}

impl Bank {

    pub fn new() -> Self {
        Bank { 
            action: HashMap::new(),
        }
    }

    pub fn from_cards(cards: Vec<Card>) -> Self {
        // Create a deck from a vec of cards.
        // Starts with a completely empty bank. Use base_game() to include all of the
        // money and victory cards for a base game.
        let hm = HashMap::new();
        for card in cards {
            hm.insert(k, v);
        }
        Bank {
            action: hm
        }
    }

    pub fn base_game() -> Self {
        let mut bank = Bank {
            action: HashMap::new(),
        };
        // TODO: Check these are the right values
        bank.action.insert(card::base_game::copper(), 60);
        bank.action.insert(card::base_game::silver(), 40);
        bank.action.insert(card::base_game::gold(), 30);

        bank.action.insert(card::base_game::estate(), 12);
        bank.action.insert(card::base_game::duchy(), 12);
        bank.action.insert(card::base_game::province(), 12);
        bank
    }

    pub fn buy_card(&self) -> {
        // TODO buy_card()
    }

}
