

pub fn copper() -> Card {
    Card {
        name: String::from("Copper"),
        description: None,
        cost: 0,
        steps: None
    }
}
pub fn silver() -> Card {
    Card {
        name: String::from("Silver"),
        description: None,
        cost: 3,
        steps: None
    }
}
pub fn gold() -> Card {
    Card {
        name: String::from("Gold"),
        description: None,
        cost: 6,
        steps: None
    }
}

pub fn village() -> Card {
    Card {
        name: String::from("Village"),
        description: None,
        cost: 3,
        steps: Some(vec![
            Step::PlusCard(1),
            Step::PlusAction(2)
        ])
    }
}

pub fn estate() -> Card {
    Card {
        name: String::from("Estate"),
        description: None,
        cost: 2,
        steps: None
    }
}

pub fn duchy() -> Card {
    Card {
        name: String::from("Duchy"),
        description: None,
        cost: 2,
        steps: None
    }
}
