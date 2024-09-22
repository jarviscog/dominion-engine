use crate::card::Card;

#[derive(Debug)]
pub struct Hand {
    cards: Vec<Card>
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
            cards
        }
    }

    pub fn add_card(&self, card: Card) {
        // Add a card to the deck
        self.cards.push(card)
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

    pub fn victory_points(&self) -> u32 {
        let mut sum = 0;
        for card in self.cards {
            if let Some(amount) = card.victory_points() {
                sum += u32::from(amount);
            }
        }
        sum
    }

    pub fn actions(&self) {
        // Return all of the action cards
    }

    pub fn to_string(&self) -> &str {
        "Hand"
    }

}
