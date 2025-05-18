use super::*;

impl Card {
    pub fn plus_action_buy_coin() -> Card {
        Card {
            name: "Plus action, buy, coin".to_owned(),
            expansion: Expansion::Custom,
            cost: vec![Cost::Coin(3)],
            card_type: vec![CardType::Action(vec![
                NodeTemplate::PlusAction(RuntimeI32::Const(1)),
                NodeTemplate::PlusBuy(RuntimeI32::Const(1)),
                NodeTemplate::PlusCoin(RuntimeI32::Const(1)),
            ])],
            on_gain: None,
        }
    }

    pub fn discard_a_card() -> Card {
        Card {
            name: "Discard a card".to_owned(),
            expansion: Expansion::Custom,
            cost: vec![Cost::Coin(3)],
            card_type: vec![CardType::Action(vec![NodeTemplate::DiscardCard(
                RuntimeI32::Const(1),
            )])],
            on_gain: None,
        }
    }

    pub fn all_players_discard_a_card() -> Card {
        Card {
            name: "All players discard a card".to_owned(),
            expansion: Expansion::Custom,
            cost: vec![Cost::Coin(3)],
            card_type: vec![CardType::Action(vec![
                NodeTemplate::TransferCards(
                    true,
                    EffectedPlayers::All,
                    None,
                    Location::Hand,
                    Location::Discard,
                    None,
                ),
                NodeTemplate::DiscardCard(RuntimeI32::Const(1)),
            ])],
            on_gain: None,
        }
    }
}
