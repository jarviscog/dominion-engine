use super::*;

pub fn register() -> Vec<(&'static str, fn() -> Card)> {
    vec![
        ("Courtyard", Card::courtyard),
        //("Baron", Card::baron),
        //("Bridge", Card::bridge),
        //("Conspirator", Card::conspirator),
        //("Courtier", Card::courtier),
        //("Diplomat", Card::diplomat),
        ("Duke", Card::duke),
        ("Farm", Card::farm),
        //("Ironworks", Card::ironworks),
        //("Lurker", Card::lurker),
        //("Masquerade", Card::masquerade),
        //("Mill", Card::mill),
        //("Mining Village", Card::mining_village),
        //("Minion", Card::minion),
        //("Nobles", Card::nobles),
        //("Patrol", Card::patrol),
        //("Pawn", Card::pawn),
        //("Replace", Card::replace),
        //("Secret Passage", Card::secret_passage),
        //("Shanty Town", Card::shanty_town),
        //("Steward", Card::steward),
        //("Swindler", Card::swindler),
        //("Torturer", Card::torturer),
        //("Trading Post", Card::tradingpost),
        //("Upgrade", Card::upgrade),
        //("Wishing Well", Card::wishing_well),
    ]
}

impl Card {
    pub fn duke() -> Card {
        Card {
            name: "Duke".to_owned(),
            expansion: Expansion::Intrigue,
            card_type: vec![
                CardType::Treasure(RuntimeI32::Const(2)),
                CardType::Victory(RuntimeI32::Const(2)),
            ],
            cost: vec![Cost::Coin(6)],
            on_gain: None,
        }
    }

    pub fn farm() -> Card {
        Card {
            name: "Farm".to_owned(),
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
