use crate::game_step::GameStep;

pub mod base_game;

#[derive(Debug, Clone)]
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


#[derive(Debug, Clone)]
pub struct Card {
    name: String,
    description: Option<String>,
    cost: u8,
    steps: Option<Vec<GameStep>>,
    types: Vec<CardType>,
}

impl Card {

    pub fn new(name: &str, description: Option<&str>, cost: u8, steps: Option<Vec<GameStep>>, types: Vec<CardType>) -> Self {
        let mut desc: Option<String>;
        desc = match description {
            Some(pulled_desc) => Some(String::from(pulled_desc)),
            None => None
        };
        Card {
            name: name.to_owned(),
            description: desc,
            cost,
            steps,
            types,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn cost(&self) -> u8 {
        self.cost
    }

    pub fn victory_points(&self) -> Option<u8> {
        self.types.iter().filter_map(|c| match c {
            CardType::Victory(points) => Some(points),
            _ => None
        }).next().copied()
    }

    pub fn value(&self) -> Option<u8> {
        self.types.iter().filter_map(|c| match c {
            CardType::Treasure(value) => Some(value),
            _ => None
        }).next().copied()
    }

    pub fn to_string(&self) -> &str {
        &self.name
    }




}
