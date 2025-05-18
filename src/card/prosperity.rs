
use super::*;

impl Card {

    pub fn platinum() -> Card {
        Card {
            name: "Platinum".to_owned(),
            expansion: Expansion::Prosperity,
            card_type: vec![
                CardType::Treasure(RuntimeI32::Const(5)),
            ],
            on_gain: None,
            cost: vec![Cost::Coin(9)]
        }

    }

    pub fn colony() -> Card {
        Card {
            name: "Colony".to_owned(),
            expansion: Expansion::Prosperity,
            card_type: vec![
                CardType::Victory(RuntimeI32::Const(10)),
            ],
            on_gain: None,
            cost: vec![Cost::Coin(11)]
        }

    }

    pub fn workers_village() -> Card {
        Card {
            name: "Worker's Village".to_owned(),
            expansion: Expansion::Prosperity,
            card_type: vec![
                CardType::Action(
                    vec![
                        NodeTemplate::DrawCard(RuntimeI32::Const(1)),
                        NodeTemplate::PlusAction(RuntimeI32::Const(2)),
                        NodeTemplate::PlusBuy(RuntimeI32::Const(1)),
                    ]
                ),
            ],
            on_gain: None,
            cost: vec![Cost::Coin(11)]
        }

    }

    pub fn bank() -> Card {
        Card {
            name: "Bank".to_owned(),
            expansion: Expansion::Prosperity,
            card_type: vec![
                CardType::Treasure(RuntimeI32::NumberOfCardsInLocation(
                    Location::InPlay,
                    Some(vec![
                        CardFilter::Type(CardType::Treasure(RuntimeI32::Any)),
                    ])
                ))
            ],
            on_gain: None,
            cost: vec![Cost::Coin(7)]
        }
    }

}
