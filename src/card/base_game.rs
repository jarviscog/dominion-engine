use super::CardType;
use super::Card;
use crate::game_step::GameStep;

pub fn copper() -> Card {
    Card::new("Copper", None, 0, None, vec![CardType::Treasure(1)])
}
pub fn silver() -> Card {
    Card::new("Silver", None, 3, None, vec![CardType::Treasure(2)])
}
pub fn gold() -> Card {
    Card::new("Gold", None, 6, None, vec![CardType::Treasure(3)])
}

pub fn village() -> Card {
    let steps = Some(vec![
        GameStep::PlusCard(1),
        GameStep::PlusAction(2)
    ]);
    let types = vec![
        CardType::Action
    ];
    Card::new("Village", None, 3, steps, types)
}

pub fn estate() -> Card {
    Card::new("Estate", None, 2, None, vec![CardType::Victory(1)])
}

pub fn duchy() -> Card {
    // TODO this is the wrong cost
    Card::new("Duchy", None, 2, None, vec![CardType::Victory(3)])
}

pub fn province() -> Card {
    Card::new("Province", None, 8, None, vec![CardType::Victory(6)])
}
