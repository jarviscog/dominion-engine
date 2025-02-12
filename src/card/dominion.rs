use super::*;

impl Card {

    pub fn cellar() -> Card {
        Card {
            name: "Cellar".to_owned(),
            expansion: Expansion::Dominion,
            card_type: vec![
                CardType::Action(vec![
                    Step::PlusAction(RuntimeValue::FixedValue(1)),
                    
                    Step::TransferCards(
                        false,
                        EffectedPlayers::You,
                        None,
                        Location::Hand,
                        Location::InternalBuffer,
                    ),
                    Step::ExtractValue(
                        ExtractedValueType::CardCount, 
                        Location::InternalBuffer, 
                        Box::new(Step::DrawCard(RuntimeValue::FromAbove))
                    ),
                    Step::TransferCards(
                        true, 
                        EffectedPlayers::You, 
                        None, 
                        Location::InternalBuffer,
                        Location::Discard, 
                    )

                ])
            ],
            on_gain: None,
            cost: Cost::Coin(2),
        }
    }

    pub fn festival() -> Card {
        Card {
            name: "Festival".to_owned(),
            expansion: Expansion::Dominion,
            on_gain: None,
            cost: Cost::Coin(5),
            card_type: vec![
                CardType::Action(
                    vec![
                        Step::PlusAction(RuntimeValue::FixedValue(2)),
                        Step::PlusBuy(RuntimeValue::FixedValue(1)),
                        Step::PlusCoin(RuntimeValue::FixedValue(2)),
                    ]
                )
            ],
        }
    }

    pub fn smithy() -> Card {
        Card {
            name: "Smithy".to_owned(),
            expansion: Expansion::Dominion,
            on_gain: None,
            cost: Cost::Coin(5),
            card_type: vec![
                CardType::Action(vec![
                    Step::DrawCard(RuntimeValue::FixedValue(3)),
                ])
            ],
        }
    }

    pub fn village() -> Card {
        Card {
            name: "Village".to_owned(),
            expansion: Expansion::Dominion,
            on_gain: None,
            cost: Cost::Coin(5),
            card_type: vec![
                CardType::Action(vec![
                    Step::DrawCard(RuntimeValue::FixedValue(1)),
                    Step::PlusAction(RuntimeValue::FixedValue(2)),
                ])
            ],
        }
    }

    pub fn remodel() -> Card {
        Card {
            name: "Remodel".to_owned(),
            expansion: Expansion::Dominion,
            on_gain: None,
            cost: Cost::Coin(4),
            card_type: vec![
                CardType::Action(vec![
                    Step::TransferCards(
                        true, 
                        EffectedPlayers::You, 
                        None,
                        Location::Hand, 
                        Location::InternalBuffer,
                    ),
                    Step::ExtractValue(
                        ExtractedValueType::CoinValue, 
                        Location::InternalBuffer, 
                        Box::new(Step::GainCard(vec![
                            CardFilter::CoinCostUpto(
                                RuntimeValue::Add(
                                    Box::new(RuntimeValue::FromAbove),
                                    Box::new(RuntimeValue::FixedValue(2)),
                                )
                            )]
                        )),
                    ),
                    Step::TransferCards(
                        true, 
                        EffectedPlayers::You, 
                        None,
                        Location::InternalBuffer, 
                        Location::Trash,
                    ),
                
                ])
            ],
        }
    }

    pub fn mine() -> Card {
        Card {
            name: "Mine".to_owned(),
            expansion: Expansion::Dominion,
            on_gain: None,
            cost: Cost::Coin(5),
            card_type: vec![CardType::Action(vec![
                Step::TransferCards(
                    false, 
                    EffectedPlayers::You, 
                    Some(vec![CardFilter::Type(CardType::Treasure(RuntimeValue::Any))]), 
                    Location::Hand, 
                    Location::InternalBuffer,
                ),
                Step::ExtractValue(
                    ExtractedValueType::CoinValue, 
                    Location::InternalBuffer, 
                    Box::new(Step::GainCardToHand(vec![
                        CardFilter::CoinCostUpto(
                            RuntimeValue::Add(
                                Box::new(RuntimeValue::FromAbove),
                                Box::new(RuntimeValue::FixedValue(3)),
                            )
                        ),
                        CardFilter::Type(CardType::Treasure(RuntimeValue::Any))
                    ])),
                ),
                Step::TransferCards(
                    true, 
                    EffectedPlayers::You, 
                    None, 
                    Location::InternalBuffer, 
                    Location::Trash,
                ),

            ])],
        }
    }

    pub fn chapel() -> Card {
        Card {
            name: "Chapel".to_owned(),
            expansion: Expansion::Dominion,
            cost: Cost::Coin(2),
            on_gain: None,
            card_type: vec![CardType::Action(vec![
                Step::TransferCards(
                    false, 
                    EffectedPlayers::You, 
                    Some(vec![CardFilter::CardCountUpto(RuntimeValue::FixedValue(4))]), 
                    Location::Hand, 
                    Location::Trash,
                ),

            ])
            ],
        }
    }

    //pub fn council_room() -> Card {
        //Card {
            //name: "Council Room".to_owned(),
            //expansion: Expansion::Dominion,
            //action_steps: Some(vec![
                //Step::DrawCard(RuntimeValue::FixedValue(4)),
                //Step::PlusBuy(RuntimeValue::FixedValue(1)),
                //Step::ForceMoveXCards(
                    //EffectedPlayers::AllOthers, 
                    //CardFilter::All,
                    //Location::DeckTop, 
                    //Location::Hand,
                    //RuntimeValue::FixedValue(1),
                //)
            //]),
            //treasure_value: 0,
            //cost: Cost::Coin(2),
            //card_type: vec![CardType::Action],
        //}
    //}
//
    //pub fn poacher() -> Card {
        //Card {
            //name: "Poacher".to_owned(),
            //expansion: Expansion::Dominion,
            //action_steps: Some(vec![
                //Step::DrawCard(RuntimeValue::FixedValue(1)),
                //Step::PlusAction(RuntimeValue::FixedValue(1)),
                //Step::PlusCoin(RuntimeValue::FixedValue(1)),
                //Step::DiscardCard(RuntimeValue::NumberOfEmptySupplyPiles),
            //]),
            //treasure_value: 0,
            //cost: Cost::Coin(4),
            //card_type: vec![CardType::Action],
        //}
    //}
//
    //pub fn workshop() -> Card {
        //Card {
            //name: "Workshop".to_owned(),
            //expansion: Expansion::Dominion,
            //action_steps: Some(vec![
                //Step::ForceMoveXCards(
                    //EffectedPlayers::You, 
                    //CardFilter::UpToValue(RuntimeValue::FixedValue(4)),
                    //Location::Supply,
                    //Location::Discard,
                    //RuntimeValue::FixedValue(1),
                //)
            //]),
            //treasure_value: 0,
            //cost: Cost::Coin(3),
            //card_type: vec![CardType::Action],
        //}
    //}

    pub fn market() -> Card {
        Card {
            name: "Market".to_owned(),
            expansion: Expansion::Dominion,
            on_gain: None,
            cost: Cost::Coin(5),
            card_type: vec![
                CardType::Action(vec![
                    Step::DrawCard(RuntimeValue::FixedValue(1)),
                    Step::PlusAction(RuntimeValue::FixedValue(1)),
                    Step::PlusBuy(RuntimeValue::FixedValue(1)),
                    Step::PlusCoin(RuntimeValue::FixedValue(1)),
                ])
            ],
        }
    }

    pub fn moneylender() -> Card {
        Card {
            name: "Moneylender".to_owned(),
            expansion: Expansion::Dominion,
            cost: Cost::Coin(4),
            on_gain: None,
            card_type: vec![
                CardType::Action(vec![
                    Step::TransferCards(
                        false,
                        EffectedPlayers::You,
                        Some(vec![CardFilter::Name("Copper".to_owned())]),
                        Location::Hand,
                        Location::InternalBuffer,
                    ),
                    Step::ExtractValue(
                        ExtractedValueType::CardName, 
                        Location::InternalBuffer, 
                        Box::new(Step::PlusCoin(
                            RuntimeValue::Mult(
                                Box::new(RuntimeValue::FromAbove), 
                                Box::new(RuntimeValue::FixedValue(3))
                            )
                        )),
                    ),
                    Step::TransferCards(
                        true, 
                        EffectedPlayers::You, 
                        None, 
                        Location::InternalBuffer,
                        Location::Trash, 
                    )
                ])
            ],
        }
    }

    // TODO I don't think this works exactly right. If the card is played, it should to into
    // InPlay, not Discard. This happens now, but the last TransferCards will not move any cards
    pub fn vassal() -> Card {
        Card {
            name: "Vassal".to_owned(),
            expansion: Expansion::Dominion,
            cost: Cost::Coin(3),
            on_gain: None,
            card_type: vec![
                CardType::Action(vec![
                    Step::PlusCoin(RuntimeValue::FixedValue(2)),

                    Step::TransferCards(
                        true,
                        EffectedPlayers::You,
                        None,
                        Location::DeckTop,
                        Location::InternalBuffer,
                    ),
                    Step::ExtractValue(
                        ExtractedValueType::CardName, 
                        Location::InternalBuffer, 
                        Box::new(Step::OptionalPlayAction(RuntimeValue::FromAbove, None)),
                    ),
                    Step::TransferCards(
                        true, 
                        EffectedPlayers::You, 
                        None, 
                        Location::InternalBuffer,
                        Location::Discard, 
                    )

                ]),
            ],
        }
    }

    pub fn witch() -> Card {
        Card {
            name: "Witch".to_owned(),
            expansion: Expansion::Dominion,
            cost: Cost::Coin(5),
            on_gain: None,
            card_type: vec![
                CardType::Action(vec![
                    Step::DrawCard(RuntimeValue::FixedValue(2)),
                ]),
                CardType::Attack(vec![
                    Step::TransferCards(
                        true, 
                        EffectedPlayers::AllOthers,
                        Some(vec![CardFilter::Name("Curse".to_owned())]), 
                        Location::Supply, 
                        Location::Discard,
                    ),
                ])
            ],
        }
    }


}
