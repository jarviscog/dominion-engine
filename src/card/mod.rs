pub mod dominion;
mod type;


#[derive(Debug)]
pub struct Card {
  name: String,
  description: Option<String>,
  cost: u8,
  steps: Option<Vec<Step>>,
}

impl Card {

  pub fn new(name: &str, description: Option<&str>, cost: u8, steps: Option<Vec<Step>>) -> Self {
    Card {
      name: name.to_owned(),
      description: description.to_owned(),
      cost,
      steps
    }
  }

  pub fn cost(&self) -> u8 {
    self.cost
  }

  pub fn to_string(&self) -> &str {
    &self.name
  }



}
