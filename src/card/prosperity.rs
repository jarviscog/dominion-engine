use super::*;

impl Card {

    pub fn platinum() -> Card {
        Card {
            name: "Platinum".to_owned(),
            expansion: Expansion::Prosperity,
            card_type: vec![
                CardType::Treasure(RuntimeValue::FixedValue(5)),
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
                CardType::Victory(RuntimeValue::FixedValue(10)),
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
                        NodeTemplate::DrawCard(RuntimeValue::FixedValue(1)),
                        NodeTemplate::PlusAction(RuntimeValue::FixedValue(2)),
                        NodeTemplate::PlusBuy(RuntimeValue::FixedValue(1)),
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
                CardType::Treasure(RuntimeValue::CardsInLocation(
                    vec![
                        CardFilter::Type(CardType::Treasure(RuntimeValue::Any)),
                    ]
                ))
            ],
            on_gain: None,
            cost: vec![Cost::Coin(7)]
        }
    }

}
