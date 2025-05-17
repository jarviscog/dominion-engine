
use super::*;

impl Card {
    pub fn villa() -> Card {
        Card {
            name: "Villa".to_owned(),
            expansion: Expansion::Empires,
            card_type: vec![CardType::Action(vec![
                NodeTemplate::PlusAction(RuntimeValue::FixedValue(2)),
                NodeTemplate::PlusBuy(RuntimeValue::FixedValue(1)),
                NodeTemplate::PlusCoin(RuntimeValue::FixedValue(1)),
            ])],
            cost: vec![Cost::Coin(6)],
            on_gain: Some(NodeTemplate::Conditional {
                condition: Condition::NotInBuyPhase,
                then_branch: vec![
                    NodeTemplate::GotoActionPhase,
                    NodeTemplate::PlusAction(RuntimeValue::FixedValue(1)),
                ],
                else_branch: vec![],
            }),
        }
    }

}
