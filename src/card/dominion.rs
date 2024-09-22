use super::CardType;
use super::Card;
use crate::game_step::GameStep;

pub fn copper() -> Card {
    Card {
        name: String::from("Copper"),
        description: None,
        cost: 0,
        steps: None,
        types: vec![
            CardType::Treasure(1)
        ]
    }
}
pub fn silver() -> Card {
    Card {
        name: String::from("Silver"),
        description: None,
        cost: 3,
        steps: None,
        types: vec![
            CardType::Treasure(2)
        ]
    }
}
pub fn gold() -> Card {
    Card {
        name: String::from("Gold"),
        description: None,
        cost: 6,
        steps: None,
        types: vec![
            CardType::Treasure(3)
        ]
    }
}

pub fn village() -> Card {
    Card {
        name: String::from("Village"),
        description: None,
        cost: 3,
        steps: Some(vec![
            GameStep::PlusCard(1),
            GameStep::PlusAction(2)
        ]),
        types: vec![
            CardType::Action
        ]
    }
}

pub fn estate() -> Card {
    Card {
        name: String::from("Estate"),
        description: None,
        cost: 2,
        steps: None,
        types: vec![
            CardType::Victory(1)
        ]
    }
}

pub fn duchy() -> Card {
    Card {
        name: String::from("Duchy"),
        description: None,
        cost: 2,
        steps: None,
        types: vec![
            CardType::Victory(3)
        ]
    }
}

pub fn province() -> Card {
    Card {
        name: String::from("Province"),
        description: None,
        cost: 8,
        steps: None,
        types: vec![
            CardType::Victory(6)
        ]
    }
}
