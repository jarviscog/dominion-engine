

use super::*;


impl Card {

    pub fn harem() -> Card {
        Card {
            name: "Harem".to_owned(),
            expansion: Expansion::Intrigue,
            card_type: vec![
                CardType::Treasure(RuntimeValue::FixedValue(2)),
                CardType::Victory(RuntimeValue::FixedValue(2))
            ],
            cost: Cost::Coin(6),
            on_gain: None,
        }
    }

}
