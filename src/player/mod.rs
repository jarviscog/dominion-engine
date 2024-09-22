
use crate::{card, hand, deck};
use crate::game_step::{self, GameStep};

#[derive(Debug)]
pub struct Player {
    name: String,
    hand: hand::Hand,
    deck: deck::Deck
}

impl Player {

    pub fn new(name: &str) -> Self {
        Player {
            name: name.to_owned(),
            hand: hand::Hand::new(),
            deck: deck::Deck::starting_deck()
        }
    }

    pub fn play_action_phase(&self) -> Option<Vec<GameStep>> {
        None
    }

    pub fn play_buy_phase(&self) -> Option<Vec<GameStep>> {
        // Return a list of cards to buy
        Some(vec![GameStep::BuyCard(card::dominion::copper())])
    }

    pub fn victory_points(&self) -> u32 {
        self.deck.victory_points() +
        self.hand.victory_points()
    }

    pub fn to_string(&self) -> String {
        String::from(format!("Player: {}", self.name))
    }

}
