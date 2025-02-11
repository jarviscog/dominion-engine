use super::*;


impl Card {

    pub fn warehouse() -> Card {
        Card {
            name: "Warehouse".to_owned(),
            expansion: Expansion::Seaside,
            cost: Cost::Coin(3),
            card_type: vec![CardType::Action],
            treasure_value: 0,
            action_steps: Some(vec![
                Step::DrawCard(RuntimeValue::FixedValue(3)),
                Step::PlusAction(RuntimeValue::FixedValue(1)),
                Step::DiscardCard(RuntimeValue::FixedValue(3)),
            ]),
        }
    }

}
