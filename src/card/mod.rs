pub mod dominion;

pub enum CardType {
    
    // Basic
    Action, 
    Treasure(u8), // Value
    Victory(u8), // 
    Curse(u8),

    Attack, 
    Duration, 
    Reaction, 
    Command, 
}


#[derive(Debug)]
pub struct Card {
  name: String,
  description: Option<String>,
  cost: u8,
  steps: Option<Vec<Step>>,
  types: Vec<CardType>,
}

impl Card {

  pub fn new(name: &str, description: Option<&str>, cost: u8, steps: Option<Vec<Step>>, types: Vec<CardType>) -> Self {
    Card {
      name: name.to_owned(),
      description: description.to_owned(),
      cost,
      steps,
      types,
    }
  }

  pub fn cost(&self) -> u8 {
    self.cost
  }

  pub fn to_string(&self) -> &str {
    &self.name
  }



}
