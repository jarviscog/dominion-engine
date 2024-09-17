use crate::step::Step;

#[derive(Debug)]
pub struct Card {
    name: String,
    description: String,
    cost: u8,
}

impl Card {

    pub fn new(name: &str, description: &str, cost: u8) -> Self {
        Card {
            name: name.to_owned(),
            description: description.to_owned(),
            cost
        }
    }

    pub fn to_string(&self) -> &str {
        &self.name
    }

    pub fn cost(&self) -> u8 {
        self.cost
    }

}
