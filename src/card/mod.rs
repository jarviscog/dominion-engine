use crate::game_step::GameStep;

pub mod base_game;
pub mod action;
pub mod components;
pub mod action_type;
pub mod filters;


#[derive(Debug, Clone)]
pub struct Card {
    name: String,
    description: Option<String>,
    cost: u8,
    steps: Option<Vec<GameStep>>,
    components: Vec<CardComponent>,
}

impl Card {

    pub fn new(name: &str, description: Option<&str>, cost: u8, steps: Option<Vec<GameStep>>, components: Vec<CardComponent>) -> Self {
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
            components,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn cost(&self) -> u8 {
        self.cost
    }

    pub fn victory_points(&self) -> Option<u8> {
        self.components.iter().filter_map(|c| match c {
            CardComponent::Victory(points) => Some(points),
            _ => None
        }).next().copied()
    }

    pub fn value(&self) -> Option<u8> {
        self.components.iter().filter_map(|c| match c {
            CardComponent::Treasure(value) => Some(value),
            _ => None
        }).next().copied()
    }

    pub fn to_string(&self) -> &str {
        &self.name
    }




}
