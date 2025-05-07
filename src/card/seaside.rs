use super::*;

impl Card {
    pub fn warehouse() -> Card {
        Card {
            name: "Warehouse".to_owned(),
            expansion: Expansion::Seaside,
            cost: vec![Cost::Coin(3)],
            card_type: vec![CardType::Action(vec![
                NodeType::DrawCard(RuntimeValue::FixedValue(3)),
                NodeType::PlusAction(RuntimeValue::FixedValue(1)),
                NodeType::DiscardCard(RuntimeValue::FixedValue(3)),
            ])],
            on_gain: None,
        }
    }
}
