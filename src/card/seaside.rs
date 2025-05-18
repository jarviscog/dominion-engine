use super::*;

impl Card {

    pub fn bazaar() -> Card {
        Card {
            name: "Bazaar".to_owned(),
            expansion: Expansion::Seaside,
            cost: vec![Cost::Coin(5)],
            card_type: vec![
                CardType::Action(vec![
                    NodeTemplate::DrawCard(RuntimeI32::Const(1)),
                    NodeTemplate::PlusAction(RuntimeI32::Const(2)),
                    NodeTemplate::PlusCoin(RuntimeI32::Const(1)),

                ])
            ],
            on_gain: None,
        }
    }

    pub fn warehouse() -> Card {
        Card {
            name: "Warehouse".to_owned(),
            expansion: Expansion::Seaside,
            cost: vec![Cost::Coin(3)],
            card_type: vec![CardType::Action(vec![
                NodeTemplate::DrawCard(RuntimeI32::Const(3)),
                NodeTemplate::PlusAction(RuntimeI32::Const(1)),
                NodeTemplate::DiscardCard(RuntimeI32::Const(3)),
            ])],
            on_gain: None,
        }
    }
}
