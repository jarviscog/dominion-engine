
use crate::card::Card;

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>
}

impl Deck {

    pub fn new() -> Self {
        Deck {
            cards: vec![]
        }
    }
    
    pub fn from_cards(cards: vec) -> Self {
        // Create a deck from a vec of cards
        // TODO from_cards
    }

    pub fn starting_deck() -> Self {
        // TODO starting_deck        
        Deck {
            cards: vec![
                Card::copper() * 3,
                Card::estate() * 7
            ]
        }
    }

    pub fn add_card(card: Card) {
        // Add a card to the deck
        // TODO add_card
    }

    pub fn value(&self) -> u32 {
        // Get the value of all of the cards
        // TODO value
    }

    pub fn victory_points(&self) -> u32 {
        // TODO value
    }
    
    pub fn shuffle(&self) {
        // A perfect shuffle of the deck
        // TODO shuffle
    }

    pub fn human_shuffle(&self) {
        // Results in a more human shuffle
        // TODO human_shuffle
    }

    pub fn average_draw_value(&self) {
        // TODO average_hand_value
    }

    pub fn to_string(&self) -> &str {
        // TODO to_string
        "Deck"
    }

}
