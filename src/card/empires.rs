
use super::*;

impl Card {
    pub fn villa() -> Card {
        Card {
            name: "Villa".to_owned(),
            expansion: Expansion::Empires,
            card_type: vec![CardType::Action(vec![
                NodeTemplate::PlusAction(RuntimeI32::Const(2)),
                NodeTemplate::PlusBuy(RuntimeI32::Const(1)),
                NodeTemplate::PlusCoin(RuntimeI32::Const(1)),
            ])],
            cost: vec![Cost::Coin(6)],
            on_gain: Some(
                NodeTemplate::Conditional(
                    Condition::NotInBuyPhase, 
                    Box::new(NodeTemplate::GotoActionPhase), 
                    Box::new(NodeTemplate::PlusAction(RuntimeI32::Const(1))), 
                ),
            ),
        }
    }

}
