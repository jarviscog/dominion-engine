use super::*;

impl Card {
    pub fn harem() -> Card {
        Card {
            name: "Harem".to_owned(),
            expansion: Expansion::Intrigue,
            card_type: vec![
                CardType::Treasure(RuntimeI32::Const(2)),
                CardType::Victory(RuntimeI32::Const(2)),
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
                NodeTemplate::DrawCard(RuntimeI32::Const(3)),
                NodeTemplate::TransferCards(
                    true,
                    EffectedPlayers::You,
                    None,
                    Location::Hand,
                    Location::DeckTop,
                    None,
                ),
            ])],
            cost: vec![Cost::Coin(6)],
            on_gain: None,
        }
    }
}
