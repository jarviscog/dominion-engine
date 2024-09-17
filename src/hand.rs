use crate::card::Card;

#[derive(Debug)]
pub struct Hand {
}

impl Hand {

    pub fn new() -> Self {
        Hand {
            cards: vec![]
        }
    }
    
    pub fn from_cards(cards: vec) -> Self {
        // Create a deck from a vec of cards
        Hand {
            cards: cards
        }
    }

    pub fn add_card(card: Card) {
        // Add a card to the deck
    }

    pub fn value(&self) -> u8 {
        // Get the value of all of the cards
    }

    pub fn actions(&self) -> ? {
        // Return all of the action cards
    }

    pub fn to_string(&self) -> &str {
        "Hand"
    }

}
