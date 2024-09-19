use crate::archetypes;
use crate::hand::Hand;
use crate::deck::Deck;

#[derive(Debug)]
pub struct Player {
  name: String,
  hand: Hand,
  deck: Deck
}

impl Player {

  pub fn new(name: &str) -> Self {
    Player {
      name: name.to_owned(),
      hand: hand::Hand::new(),
      deck: deck::Deck::new(),
    }
  }

  pub fn victory_points(&self) -> u32 {
    self.deck.victory_points() +
    self.hand.victory_points()
  }

  pub fn to_string(&self) -> &str {
    // TODO to_string
    &format!("Player: {:?}", self.name)
  }

}
