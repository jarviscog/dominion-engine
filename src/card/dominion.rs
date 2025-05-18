use super::*;

pub fn register() -> Vec<(&'static str, fn() -> Card)> {
    vec![
        ("Copper", Card::copper),
        ("Curse", Card::curse),
        ("Estate", Card::estate),
        ("Silver", Card::silver),
        ("Duchy", Card::duchy),
        ("Gold", Card::gold),
        ("Province", Card::province),
        ("Cellar", Card::cellar),
        ("Chapel", Card::chapel),
        ("Moat", Card::moat),
        ("Harbinger", Card::harbinger),
        ("Merchant", Card::merchant),
        ("Vassal", Card::vassal),
        ("Village", Card::village),
        ("Workshop", Card::workshop),
        ("Bureaucrat", Card::bureaucrat),
        ("Gardens", Card::gardens),
        ("Militia", Card::militia),
        ("Moneylender", Card::moneylender),
        ("Poacher", Card::poacher),
        ("Remodel", Card::remodel),
        ("Smithy", Card::smithy),
        ("Throne Room", Card::throne_room),
        ("Bandit", Card::bandit),
        ("Council Room", Card::council_room),
        ("Festival", Card::festival),
        ("Laboratory", Card::laboratory),
        //("Library", Card::library),
        ("Market", Card::market),
        ("Mine", Card::mine),
        ("Sentry", Card::sentry),
        ("Witch", Card::witch),
        ("Artisan", Card::artisan),
    ]
}

impl Card {
    pub fn copper() -> Card {
        Card {
            name: "Copper".to_owned(),
            expansion: Expansion::Dominion,
            card_type: vec![CardType::Treasure(RuntimeI32::Const(1))],
            cost: vec![Cost::Coin(0)],
            on_gain: None,
        }
    }

    pub fn silver() -> Card {
        Card {
            name: "Silver".to_owned(),
            expansion: Expansion::Dominion,
            card_type: vec![CardType::Treasure(RuntimeI32::Const(2))],
            cost: vec![Cost::Coin(3)],
            on_gain: None,
        }
    }

    pub fn gold() -> Card {
        Card {
            name: "Gold".to_owned(),
            on_gain: None,
            expansion: Expansion::Dominion,
            card_type: vec![CardType::Treasure(RuntimeI32::Const(3))],
            cost: vec![Cost::Coin(6)],
        }
    }

    pub fn curse() -> Card {
        Card {
            name: "Curse".to_owned(),
            expansion: Expansion::Dominion,
            card_type: vec![CardType::Victory(RuntimeI32::Const(-1))],
            cost: vec![Cost::Coin(0)],
            on_gain: None,
        }
    }

    pub fn estate() -> Card {
        Card {
            name: "Estate".to_owned(),
            expansion: Expansion::Dominion,
            card_type: vec![CardType::Victory(RuntimeI32::Const(1))],
            cost: vec![Cost::Coin(2)],
            on_gain: None,
        }
    }

    pub fn duchy() -> Card {
        Card {
            name: "Duchy".to_owned(),
            expansion: Expansion::Dominion,
            card_type: vec![CardType::Victory(RuntimeI32::Const(3))],
            cost: vec![Cost::Coin(5)],
            on_gain: None,
        }
    }

    pub fn province() -> Card {
        Card {
            name: "Province".to_owned(),
            expansion: Expansion::Dominion,
            card_type: vec![CardType::Victory(RuntimeI32::Const(6))],
            cost: vec![Cost::Coin(8)],
            on_gain: None,
        }
    }

    pub fn gardens() -> Card {
        Card {
            name: "Gardens".to_owned(),
            expansion: Expansion::Dominion,
            card_type: vec![CardType::Victory(RuntimeI32::NumberOfCardsInDeck)],
            cost: vec![Cost::Coin(4)],
            on_gain: None,
        }
    }

    pub fn artisan() -> Card {
        Card {
            name: "Artisan".to_owned(),
            expansion: Expansion::Dominion,
            on_gain: None,
            cost: vec![Cost::Coin(6)],
            card_type: vec![CardType::Action(vec![
                NodeTemplate::GainCard(vec![CardFilter::CoinCostUpto(RuntimeI32::Const(5))]),
                NodeTemplate::TransferCards(
                    true,
                    EffectedPlayers::You,
                    None,
                    Location::Hand,
                    Location::DeckTop,
                    None,
                ),
            ])],
        }
    }

    pub fn bureaucrat() -> Card {
        Card {
            name: "Bureaucrat".to_owned(),
            expansion: Expansion::Dominion,
            on_gain: None,
            cost: vec![Cost::Coin(4)],
            card_type: vec![CardType::Action(vec![
                NodeTemplate::TransferCards(
                    true,
                    EffectedPlayers::You,
                    Some(vec![CardFilter::Name("Silver".to_owned())]),
                    Location::Supply,
                    Location::DeckTop,
                    None,
                ),
                NodeTemplate::TransferCards(
                    true,
                    EffectedPlayers::AllOthers,
                    Some(vec![CardFilter::Type(CardType::Victory(RuntimeI32::Any))]),
                    Location::Hand,
                    Location::DeckTop,
                    None,
                ),
            ])],
        }
    }

    pub fn merchant() -> Card {
        let new_event_listener = EventListener::new(
            RuntimePlayerIndex::CurrentPlayer,
            EventListenerFireCondition::WhenYouPlayCard(vec![CardFilter::Name(
                "Silver".to_owned(),
            )]),
            EventListenerDestructCondition::EndOfThisTurn,
            vec![NodeTemplate::PlusCoin(RuntimeI32::Const(1))],
            true,
        );
        Card {
            name: "Merchant".to_owned(),
            expansion: Expansion::Dominion,
            cost: vec![Cost::Coin(3)],
            on_gain: None,
            card_type: vec![CardType::Action(vec![
                NodeTemplate::DrawCard(RuntimeI32::Const(1)),
                NodeTemplate::PlusAction(RuntimeI32::Const(1)),
                NodeTemplate::AddEventListener(new_event_listener),
            ])],
        }
    }

    pub fn militia() -> Card {
        Card {
            name: "Militia".to_owned(),
            expansion: Expansion::Dominion,
            cost: vec![Cost::Coin(4)],
            on_gain: None,
            card_type: vec![CardType::Action(vec![
                NodeTemplate::PlusCoin(RuntimeI32::Const(2)),
                NodeTemplate::TransferCards(
                    true,
                    EffectedPlayers::AllOthers,
                    Some(vec![CardFilter::DownTo(RuntimeI32::Const(3))]),
                    Location::Hand,
                    Location::Discard,
                    None,
                ),
            ])],
        }
    }

    pub fn cellar() -> Card {
        Card {
            name: "Cellar".to_owned(),
            expansion: Expansion::Dominion,
            card_type: vec![CardType::Action(vec![
                NodeTemplate::PlusAction(RuntimeI32::Const(1)),
                NodeTemplate::TransferCards(
                    false,
                    EffectedPlayers::You,
                    None,
                    Location::Hand,
                    Location::Discard,
                    Some("discarded-cards".to_string()),
                ),
                NodeTemplate::TransferCards(
                    true,
                    EffectedPlayers::You,
                    Some(vec![CardFilter::CardCountEquals(
                        RuntimeI32::CostFromContext("discarded-cards".to_string()),
                    )]),
                    Location::InternalBuffer,
                    Location::Discard,
                    None,
                ),
            ])],
            on_gain: None,
            cost: vec![Cost::Coin(2)],
        }
    }

    pub fn festival() -> Card {
        Card {
            name: "Festival".to_owned(),
            expansion: Expansion::Dominion,
            on_gain: None,
            cost: vec![Cost::Coin(5)],
            card_type: vec![CardType::Action(vec![
                NodeTemplate::PlusAction(RuntimeI32::Const(2)),
                NodeTemplate::PlusBuy(RuntimeI32::Const(1)),
                NodeTemplate::PlusCoin(RuntimeI32::Const(2)),
            ])],
        }
    }

    pub fn smithy() -> Card {
        Card {
            name: "Smithy".to_owned(),
            expansion: Expansion::Dominion,
            on_gain: None,
            cost: vec![Cost::Coin(4)],
            card_type: vec![CardType::Action(vec![NodeTemplate::DrawCard(
                RuntimeI32::Const(3),
            )])],
        }
    }

    pub fn bandit() -> Card {
        Card {
            name: "Bandit".to_owned(),
            expansion: Expansion::Dominion,
            on_gain: None,
            cost: vec![Cost::Coin(5)],
            card_type: vec![
                CardType::Action(vec![NodeTemplate::GainCard(vec![CardFilter::Name(
                    "Gold".to_owned(),
                )])]),
                CardType::Attack(vec![
                    NodeTemplate::TransferCards(
                        true,
                        EffectedPlayers::AllOthers,
                        Some(vec![CardFilter::CardCountEquals(RuntimeI32::Const(2))]),
                        Location::DeckTop,
                        Location::InternalBuffer,
                        None,
                    ),
                    NodeTemplate::TransferCards(
                        true,
                        EffectedPlayers::AllOthers,
                        Some(vec![
                            CardFilter::Type(CardType::Treasure(RuntimeI32::Any)),
                            CardFilter::NotName("Copper".to_owned()),
                        ]),
                        Location::InternalBuffer,
                        Location::Trash,
                        None,
                    ),
                    NodeTemplate::TransferCards(
                        true,
                        EffectedPlayers::AllOthers,
                        None,
                        Location::InternalBuffer,
                        Location::Discard,
                        None,
                    ),
                ]),
            ],
        }
    }

    pub fn moat() -> Card {
        Card {
            name: "Moat".to_owned(),
            expansion: Expansion::Dominion,
            on_gain: None,
            cost: vec![Cost::Coin(2)],
            card_type: vec![
                CardType::Action(vec![NodeTemplate::DrawCard(RuntimeI32::Const(2))]),
                CardType::Reaction(vec![NodeTemplate::IgnoreAttacks]),
            ],
        }
    }
    pub fn laboratory() -> Card {
        Card {
            name: "Laboratory ".to_owned(),
            expansion: Expansion::Dominion,
            on_gain: None,
            cost: vec![Cost::Coin(5)],

            card_type: vec![CardType::Action(vec![
                NodeTemplate::DrawCard(RuntimeI32::Const(2)),
                NodeTemplate::PlusAction(RuntimeI32::Const(1)),
            ])],
        }
    }

    pub fn village() -> Card {
        Card {
            name: "Village".to_owned(),
            expansion: Expansion::Dominion,
            on_gain: None,
            cost: vec![Cost::Coin(5)],
            card_type: vec![CardType::Action(vec![
                NodeTemplate::DrawCard(RuntimeI32::Const(1)),
                NodeTemplate::PlusAction(RuntimeI32::Const(2)),
            ])],
        }
    }

    pub fn harbinger() -> Card {
        Card {
            name: "Harbinger".to_owned(),
            expansion: Expansion::Dominion,
            on_gain: None,
            cost: vec![Cost::Coin(3)],
            card_type: vec![CardType::Action(vec![
                NodeTemplate::DrawCard(RuntimeI32::Const(1)),
                NodeTemplate::PlusAction(RuntimeI32::Const(1)),
                NodeTemplate::TransferCards(
                    false,
                    EffectedPlayers::You,
                    None,
                    Location::Discard,
                    Location::DeckTop,
                    None,
                ),
            ])],
        }
    }

    pub fn sentry() -> Card {
        Card {
            name: "Sentry".to_owned(),
            expansion: Expansion::Dominion,
            on_gain: None,
            cost: vec![Cost::Coin(5)],
            card_type: vec![CardType::Action(vec![
                NodeTemplate::DrawCard(RuntimeI32::Const(1)),
                NodeTemplate::PlusAction(RuntimeI32::Const(1)),
                NodeTemplate::TransferCards(
                    true,
                    EffectedPlayers::You,
                    Some(vec![CardFilter::CardCountEquals(RuntimeI32::Const(2))]),
                    Location::DeckTop,
                    Location::InternalBuffer,
                    None,
                ),
                NodeTemplate::TransferCards(
                    false,
                    EffectedPlayers::You,
                    None,
                    Location::InternalBuffer,
                    Location::Trash,
                    None,
                ),
                NodeTemplate::TransferCards(
                    false,
                    EffectedPlayers::You,
                    None,
                    Location::InternalBuffer,
                    Location::Discard,
                    None,
                ),
                NodeTemplate::TransferCards(
                    true,
                    EffectedPlayers::You,
                    None,
                    Location::InternalBuffer,
                    Location::DeckTop,
                    None,
                ),
            ])],
        }
    }

    pub fn poacher() -> Card {
        Card {
            name: "Poacher".to_owned(),
            expansion: Expansion::Dominion,
            on_gain: None,
            cost: vec![Cost::Coin(4)],
            card_type: vec![CardType::Action(vec![
                NodeTemplate::DrawCard(RuntimeI32::Const(1)),
                NodeTemplate::PlusAction(RuntimeI32::Const(1)),
                NodeTemplate::PlusCoin(RuntimeI32::Const(1)),
                NodeTemplate::DiscardCard(RuntimeI32::NumberOfEmptySupplyPiles),
            ])],
        }
    }

    pub fn throne_room() -> Card {
        Card {
            name: "Throne Room".to_owned(),
            expansion: Expansion::Dominion,
            on_gain: None,
            cost: vec![Cost::Coin(4)],
            card_type: vec![CardType::Action(vec![NodeTemplate::PlayCardXTimes(
                RuntimeI32::Const(2),
                vec![CardFilter::NextCardPlayed],
            )])],
        }
    }

    pub fn remodel() -> Card {
        Card {
            name: "Remodel".to_owned(),
            expansion: Expansion::Dominion,
            on_gain: None,
            cost: vec![Cost::Coin(4)],
            card_type: vec![CardType::Action(vec![
                NodeTemplate::TransferCards(
                    true,
                    EffectedPlayers::You,
                    None,
                    Location::Hand,
                    Location::Trash,
                    Some("remodeled-card".to_string()),
                ),
                NodeTemplate::TransferCards(
                    true,
                    EffectedPlayers::You,
                    Some(vec![CardFilter::CoinCostUpto(RuntimeI32::Add(
                        Box::new(RuntimeI32::CostFromContext("remodeled-card".to_string())),
                        Box::new(RuntimeI32::Const(2)),
                    ))]),
                    Location::Supply,
                    Location::Discard,
                    None,
                ),
            ])],
        }
    }

    pub fn chapel() -> Card {
        Card {
            name: "Chapel".to_owned(),
            expansion: Expansion::Dominion,
            cost: vec![Cost::Coin(2)],
            on_gain: None,
            card_type: vec![CardType::Action(vec![NodeTemplate::TransferCards(
                false,
                EffectedPlayers::You,
                Some(vec![CardFilter::CardCountUpto(RuntimeI32::Const(4))]),
                Location::Hand,
                Location::Trash,
                None,
            )])],
        }
    }

    pub fn council_room() -> Card {
        Card {
            name: "Council Room".to_owned(),
            expansion: Expansion::Dominion,
            cost: vec![Cost::Coin(5)],
            on_gain: None,
            card_type: vec![CardType::Action(vec![
                NodeTemplate::DrawCard(RuntimeI32::Const(4)),
                NodeTemplate::PlusBuy(RuntimeI32::Const(1)),
                NodeTemplate::TransferCards(
                    true,
                    EffectedPlayers::AllOthers,
                    Some(vec![CardFilter::CardCountEquals(RuntimeI32::Const(1))]),
                    Location::DeckTop,
                    Location::Hand,
                    None,
                ),
            ])],
        }
    }

    //pub fn poacher() -> Card {
    //Card {
    //name: "Poacher".to_owned(),
    //expansion: Expansion::Dominion,
    //action_steps: Some(vec![
    //::DrawCard(RuntimeI32::Const(1)),
    //::PlusAction(RuntimeI32::Const(1)),
    //::PlusCoin(RuntimeI32::Const(1)),
    //::DiscardCard(RuntimeValue::NumberOfEmptySupplyPiles),
    //]),
    //treasure_value: 0,
    //cost: Cost::Coin(4),
    //card_type: vec![CardType::Action],
    //}
    //}

    pub fn mine() -> Card {
        Card {
            name: "Mine".to_owned(),
            expansion: Expansion::Dominion,
            on_gain: None,
            cost: vec![Cost::Coin(5)],
            card_type: vec![CardType::Action(vec![
                NodeTemplate::TransferCards(
                    false,
                    EffectedPlayers::You,
                    Some(vec![CardFilter::Type(CardType::Treasure(RuntimeI32::Any))]),
                    Location::Hand,
                    Location::Trash,
                    Some("mined-card".to_string()),
                ),
                NodeTemplate::TransferCards(
                    true,
                    EffectedPlayers::You,
                    Some(vec![CardFilter::CoinCostUpto(RuntimeI32::Add(
                        Box::new(RuntimeI32::CostFromContext("mined-card".to_string())),
                        Box::new(RuntimeI32::Const(3)),
                    ))]),
                    Location::Supply,
                    Location::Trash,
                    None,
                ),
            ])],
        }
    }

    pub fn market() -> Card {
        Card {
            name: "Market".to_owned(),
            expansion: Expansion::Dominion,
            on_gain: None,
            cost: vec![Cost::Coin(5)],
            card_type: vec![CardType::Action(vec![
                NodeTemplate::DrawCard(RuntimeI32::Const(1)),
                NodeTemplate::PlusAction(RuntimeI32::Const(1)),
                NodeTemplate::PlusBuy(RuntimeI32::Const(1)),
                NodeTemplate::PlusCoin(RuntimeI32::Const(1)),
            ])],
        }
    }

    pub fn moneylender() -> Card {
        Card {
            name: "Moneylender".to_owned(),
            expansion: Expansion::Dominion,
            cost: vec![Cost::Coin(4)],
            on_gain: None,
            card_type: vec![CardType::Action(vec![
                NodeTemplate::TransferCards(
                    false,
                    EffectedPlayers::You,
                    Some(vec![CardFilter::Name("Copper".to_owned())]),
                    Location::Hand,
                    Location::Trash,
                    Some("moneylender-trash".to_string()),
                ),
                NodeTemplate::Conditional(
                    Condition::ContextContainsCard("moneylender-trash".to_string()),
                    Box::new(NodeTemplate::PlusCoin(RuntimeI32::Const(3))),
                    Box::new(NodeTemplate::None),
                ),
            ])],
        }
    }

    // TODO I don't think this works exactly right. If the card is played, it should to into
    // InPlay, not Discard. This happens now, but the last TransferCards will not move any cards
    pub fn vassal() -> Card {
        Card {
            name: "Vassal".to_owned(),
            expansion: Expansion::Dominion,
            cost: vec![Cost::Coin(3)],
            on_gain: None,
            // TODO OptionalPlayAction has been removed. Use Or(Box<Step>, Box<Step>) where one is
            // None
            card_type: vec![CardType::Action(vec![
                NodeTemplate::PlusCoin(RuntimeI32::Const(2)),
                NodeTemplate::TransferCards(
                    true,
                    EffectedPlayers::You,
                    None,
                    Location::DeckTop,
                    Location::Discard,
                    None,
                ),
                NodeTemplate::Conditional(
                    Condition::EqualCardType(
                        RuntimeCardType::FromContext("vassal-discard".to_string()),
                        RuntimeCardType::Const(CardType::Action(vec![])),
                    ),
                    Box::new(NodeTemplate::PlayCard(RuntimeCardName::FromContext(
                        "vassal-discard".to_string(),
                    ))),
                    Box::new(NodeTemplate::None),
                ),
                NodeTemplate::TransferCards(
                    true,
                    EffectedPlayers::You,
                    None,
                    Location::InternalBuffer,
                    Location::Discard,
                    None,
                ),
            ])],
        }
    }

    pub fn workshop() -> Card {
        Card {
            name: "Workshop".to_owned(),
            expansion: Expansion::Dominion,
            cost: vec![Cost::Coin(3)],
            on_gain: None,
            card_type: vec![CardType::Action(vec![NodeTemplate::GainCard(vec![
                CardFilter::CoinCostUpto(RuntimeI32::Const(4)),
            ])])],
        }
    }

    pub fn witch() -> Card {
        Card {
            name: "Witch".to_owned(),
            expansion: Expansion::Dominion,
            cost: vec![Cost::Coin(5)],
            on_gain: None,
            card_type: vec![
                CardType::Action(vec![NodeTemplate::DrawCard(RuntimeI32::Const(2))]),
                CardType::Attack(vec![NodeTemplate::TransferCards(
                    true,
                    EffectedPlayers::AllOthers,
                    Some(vec![CardFilter::Name("Curse".to_owned())]),
                    Location::Supply,
                    Location::Discard,
                    None,
                )]),
            ],
        }
    }

    pub fn woodcutter() -> Card {
        Card {
            name: "Woodcutter".to_owned(),
            expansion: Expansion::Dominion,
            cost: vec![Cost::Coin(4)],
            on_gain: None,
            card_type: vec![CardType::Action(vec![
                NodeTemplate::PlusBuy(RuntimeI32::Const(1)),
                NodeTemplate::PlusCoin(RuntimeI32::Const(2)),
            ])],
        }
    }
}
