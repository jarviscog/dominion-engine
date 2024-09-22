
use crate::card;

#[derive(Debug)]
pub struct Deck {
    cards: Vec<card::Card>
}

impl Deck {

    pub fn new() -> Self {
        Deck {
            cards: vec![]
        }
    }
    
    pub fn from_cards(cards: Vec<card::Card>) -> Self {
        Deck {
            cards
        }
    }

    pub fn starting_deck() -> Self {
        Deck {
            cards: vec![
                card::dominion::copper() * 7,
                card::dominion::estate() * 3
            ]
        }
    }

    pub fn add_card(&self, card: &mut Card) {
        self.cards.append(card)
    }

    pub fn victory_points(&self) -> u32 {
        let mut sum = 0;
        for card in self.cards {
            if let Some(amount) = card.victory_points() {
                sum += u32::from(amount);
            }
        }
        sum
    }

    pub fn value(&self) -> u32 {
        let mut sum = 0;
        for card in self.cards {
            if let Some(amount) = card.value() {
                sum += u32::from(amount);
            }
        }
        sum
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
