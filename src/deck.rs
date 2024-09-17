
use crate::card::Card;

#[derive(Debug)]
pub struct Deck {
}

impl Deck {

    pub fn new() -> Self {
        Deck {
            cards: vec![]
        }
    }
    
    pub fn from_cards(cards: vec) -> Self {
        // Create a deck from a vec of cards
        Deck {
            cards: cards
        }
    }

    pub fn add_card(card: Card) {
        // Add a card to the deck
    }

    pub fn value(&self) -> u8 {
        // Get the value of all of the cards
    }

    pub fn to_string(&self) -> &str {
        "Deck"
    }

}
