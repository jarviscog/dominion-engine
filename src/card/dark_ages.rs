
use super::*;


impl Card {
    pub fn necropolis() -> Card {
        Card {
            name: "Necropolis".to_owned(),
            expansion: Expansion::Intrigue,
            card_type: vec![
                CardType::Action(
                    vec![
                        NodeTemplate::PlusAction(RuntimeI32::Const(2))
                    ]
                ),
                CardType::Shelter,
            ],
            cost: vec![Cost::Coin(1)],
            on_gain: None,
        }
    }

}
