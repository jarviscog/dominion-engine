
use super::*;

impl Card {

    pub fn warehouse() -> Card {
        Card {
            name: "Warehouse".to_owned(),
            expansion: Expansion::Seaside,
            cost: vec![Cost::Coin(3)],
            card_type: vec![
                CardType::Action(vec![
                    StepNodeType::DrawCard(RuntimeValue::FixedValue(3)),
                    StepNodeType::PlusAction(RuntimeValue::FixedValue(1)),
                    StepNodeType::DiscardCard(RuntimeValue::FixedValue(3)),
                ])
            ],
            on_gain: None,
        }
    }

}
