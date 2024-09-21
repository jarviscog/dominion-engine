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
    self.cards.append(card)
  }

  pub fn value(&self) -> u32 {
    // Get the value of all of the cards

  }

  pub fn victory_points(&self) -> u32 {
    // TODO victory_points
  }

  pub fn actions(&self) {
    // Return all of the action cards
  }

  pub fn to_string(&self) -> &str {
    "Hand"
  }

}
