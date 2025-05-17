use super::*;

impl Card {
    pub fn harem() -> Card {
        Card {
            name: "Harem".to_owned(),
            expansion: Expansion::Intrigue,
            card_type: vec![
                CardType::Treasure(RuntimeValue::FixedValue(2)),
                CardType::Victory(RuntimeValue::FixedValue(2)),
            ],
            cost: vec![Cost::Coin(6)],
            on_gain: None,
        }
    }

    pub fn courtyard() -> Card {
        Card {
            name: "Courtyard".to_owned(),
            expansion: Expansion::Intrigue,
            card_type: vec![CardType::Action(vec![
                NodeTemplate::DrawCard(RuntimeValue::FixedValue(3)),
                NodeTemplate::TransferCards(
                    true,
                    EffectedPlayers::You,
                    None,
                    Location::Hand,
                    Location::DeckTop,
                ),
            ])],
            cost: vec![Cost::Coin(6)],
            on_gain: None,
        }
    }
}
