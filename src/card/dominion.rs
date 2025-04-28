use super::*;

impl Card {

    pub fn copper() -> Card {
        Card {
            name: "Copper".to_owned(),
            expansion: Expansion::Dominion,
            card_type: vec![CardType::Treasure(RuntimeValue::FixedValue(1))],
            cost: vec![Cost::Coin(0)],
            on_gain: None,
        }
    }

    pub fn silver() -> Card {
        Card {
            name: "Silver".to_owned(),
            expansion: Expansion::Dominion,
            card_type: vec![CardType::Treasure(RuntimeValue::FixedValue(2))],
            cost: vec![Cost::Coin(3)],
            on_gain: None,
        }
    }

    pub fn gold() -> Card {
        Card {
            name: "Gold".to_owned(),
            on_gain: None,
            expansion: Expansion::Dominion,
            card_type: vec![CardType::Treasure(RuntimeValue::FixedValue(3))],
            cost: vec![Cost::Coin(6)],
        }
    }

    pub fn curse() -> Card {
        Card {
            name: "Curse".to_owned(),
            expansion: Expansion::Dominion,
            card_type: vec![CardType::Victory(RuntimeValue::FixedValue(-1))],
            cost: vec![Cost::Coin(0)],
            on_gain: None
        }
    }

    pub fn estate() -> Card {
        Card {
            name: "Estate".to_owned(),
            expansion: Expansion::Dominion,
            card_type: vec![CardType::Victory(RuntimeValue::FixedValue(1))],
            cost: vec![Cost::Coin(2)],
            on_gain: None,
        }
    }

    pub fn duchy() -> Card {
        Card {
            name: "Duchy".to_owned(),
            expansion: Expansion::Dominion,
            card_type: vec![CardType::Victory(RuntimeValue::FixedValue(3))],
            cost: vec![Cost::Coin(5)],
            on_gain: None,
        }
    }

    pub fn province() -> Card {
        Card {
            name: "Province".to_owned(),
            expansion: Expansion::Dominion,
            card_type: vec![CardType::Victory(RuntimeValue::FixedValue(6))],
            cost: vec![Cost::Coin(8)],
            on_gain: None,
        }
    }

    pub fn gardens() -> Card {
        Card {
            name: "Gardens".to_owned(),
            expansion: Expansion::Dominion,
            card_type: vec![
                CardType::Victory(RuntimeValue::NumberOfCardsInDeck)
            ],
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
            card_type: vec![
                CardType::Action(vec![
                    StepNodeType::GainCard(vec![
                        CardFilter::CoinCostUpto(RuntimeValue::FixedValue(5))
                    ]),
                    StepNodeType::TransferCards(
                        true, 
                        EffectedPlayers::You, 
                        None, 
                        Location::Hand,
                        Location::DeckTop,
                    )
                ])
            ]

        }

    }

    pub fn bureaucrat() -> Card {
        Card {
            name: "Bureaucrat".to_owned(),
            expansion: Expansion::Dominion,
            on_gain: None,
            cost: vec![Cost::Coin(4)],
            card_type: vec![
                CardType::Action(vec![

                    StepNodeType::TransferCards(
                        true, 
                        EffectedPlayers::You, 
                        Some(vec![CardFilter::Name("Silver".to_owned())]), 
                        Location::Supply, 
                        Location::DeckTop,
                    ),
                    StepNodeType::TransferCards(
                        true, 
                        EffectedPlayers::AllOthers, 
                        Some(vec![CardFilter::Type(CardType::Victory(RuntimeValue::Any))]), 
                        Location::Hand, 
                        Location::DeckTop,
                    ),


                ])
            ]
        }
    }

    pub fn merchant() -> Card {
        let new_event_listener = EventListener::new(
            RuntimeValue::CurrentPlayer,
            EventListenerFireCondition::WhenYouPlayCard(vec![CardFilter::Name("Silver".to_owned())]),
            EventListenerDestructCondition::EndOfThisTurn,
            vec![StepNodeType::PlusCoin(RuntimeValue::FixedValue(1))],
            true
        );
        Card {
            name: "Merchant".to_owned(),
            expansion: Expansion::Dominion,
            cost: vec![Cost::Coin(3)],
            on_gain: None,
            card_type: vec![
                CardType::Action(vec![
                    StepNodeType::PlayCard(RuntimeValue::FixedValue(1)),
                    StepNodeType::PlusAction(RuntimeValue::FixedValue(1)),
                    StepNodeType::AddEventListener(
                        new_event_listener
                    )
                ])
            ]
        }
    }

    pub fn militia() -> Card {
        Card {
            name: "Militia".to_owned(),
            expansion: Expansion::Dominion,
            cost: vec![Cost::Coin(4)],
            on_gain: None,
            card_type: vec![
                CardType::Action(vec![
                    StepNodeType::PlusCoin(RuntimeValue::FixedValue(2)),
                    StepNodeType::TransferCards(
                        true, 
                        EffectedPlayers::AllOthers, 
                        Some(vec![
                            CardFilter::DownTo(RuntimeValue::FixedValue(3))
                        ]), 
                        Location::Hand, 
                        Location::Discard,
                    )
                ]),
            ]
        }
    }

    pub fn cellar() -> Card {
        Card {
            name: "Cellar".to_owned(),
            expansion: Expansion::Dominion,
            card_type: vec![
                CardType::Action(vec![
                    StepNodeType::PlusAction(RuntimeValue::FixedValue(1)),
                    
                    StepNodeType::TransferCards(
                        false,
                        EffectedPlayers::You,
                        None,
                        Location::Hand,
                        Location::InternalBuffer,
                    ),
                    StepNodeType::ExtractValue(
                        ExtractedValueType::CardCount, 
                        Location::InternalBuffer, 
                        Box::new(StepNodeType::DrawCard(RuntimeValue::FromAbove))
                    ),
                    StepNodeType::TransferCards(
                        true, 
                        EffectedPlayers::You, 
                        None, 
                        Location::InternalBuffer,
                        Location::Discard, 
                    )

                ])
            ],
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
            card_type: vec![
                CardType::Action(
                    vec![
                        StepNodeType::PlusAction(RuntimeValue::FixedValue(2)),
                        StepNodeType::PlusBuy(RuntimeValue::FixedValue(1)),
                        StepNodeType::PlusCoin(RuntimeValue::FixedValue(2)),
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
            cost: vec![Cost::Coin(4)],
            card_type: vec![
                CardType::Action(vec![
                    StepNodeType::DrawCard(RuntimeValue::FixedValue(3)),
                ])
            ],
        }
    }

    pub fn bandit() -> Card {
        Card {
            name: "Bandit".to_owned(),
            expansion: Expansion::Dominion,
            on_gain: None,
            cost: vec![Cost::Coin(5)],
            card_type: vec![
                CardType::Action(vec![
                    StepNodeType::GainCard(vec![CardFilter::Name("Gold".to_owned())]),
                ]),
                CardType::Attack(vec![
                    StepNodeType::TransferCards(
                        true, 
                        EffectedPlayers::AllOthers, 
                        Some(vec![
                            CardFilter::CardCountEquals(RuntimeValue::FixedValue(2))
                        ]), 
                        Location::DeckTop, 
                        Location::InternalBuffer,
                    ),
                    StepNodeType::TransferCards(
                        true, 
                        EffectedPlayers::AllOthers, 
                        Some(vec![
                            CardFilter::Type(CardType::Treasure(RuntimeValue::Any)),
                            CardFilter::NotName("Copper".to_owned()),
                        ]), 
                        Location::InternalBuffer, 
                        Location::Trash,
                    ),
                    StepNodeType::TransferCards(
                        true, 
                        EffectedPlayers::AllOthers, 
                        None, 
                        Location::InternalBuffer, 
                        Location::Discard,
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
                CardType::Action(vec![
                    StepNodeType::DrawCard(RuntimeValue::FixedValue(2)),
                ]),
                CardType::Reaction(vec![
                    StepNodeType::IgnoreAttacks,
                ]),
            ],
        }
    }
    pub fn laboratory() -> Card {
        Card {
            name: "Laboratory ".to_owned(),
            expansion: Expansion::Dominion,
            on_gain: None,
            cost: vec![Cost::Coin(5)],
            card_type: vec![
                CardType::Action(vec![
                    StepNodeType::DrawCard(RuntimeValue::FixedValue(2)),
                    StepNodeType::PlusAction(RuntimeValue::FixedValue(1)),
                ])
            ],
        }
    }

    pub fn village() -> Card {
        Card {
            name: "Village".to_owned(),
            expansion: Expansion::Dominion,
            on_gain: None,
            cost: vec![Cost::Coin(5)],
            card_type: vec![
                CardType::Action(vec![
                    StepNodeType::DrawCard(RuntimeValue::FixedValue(1)),
                    StepNodeType::PlusAction(RuntimeValue::FixedValue(2)),
                ])
            ],
        }
    }

    pub fn harbinger() -> Card {
        Card {
            name: "Harbinger".to_owned(),
            expansion: Expansion::Dominion,
            on_gain: None,
            cost: vec![Cost::Coin(3)],
            card_type: vec![
                CardType::Action(vec![
                    StepNodeType::DrawCard(RuntimeValue::FixedValue(1)),
                    StepNodeType::PlusAction(RuntimeValue::FixedValue(1)),
                    StepNodeType::TransferCards(
                        false, 
                        EffectedPlayers::You, 
                        None, 
                        Location::Discard, 
                        Location::DeckTop,
                    )
                ])
            ],
        }
    }

    pub fn sentry() -> Card {
        Card {
            name: "Sentry".to_owned(),
            expansion: Expansion::Dominion,
            on_gain: None,
            cost: vec![Cost::Coin(5)],
            card_type: vec![
                CardType::Action(vec![
                    StepNodeType::DrawCard(RuntimeValue::FixedValue(1)),
                    StepNodeType::PlusAction(RuntimeValue::FixedValue(1)),
                    StepNodeType::TransferCards(
                        true, 
                        EffectedPlayers::You, 
                        Some(vec![CardFilter::CardCountEquals(RuntimeValue::FixedValue(2))]),
                        Location::DeckTop, 
                        Location::InternalBuffer
                    ),
                    StepNodeType::TransferCards(
                        false, 
                        EffectedPlayers::You,
                        None,
                        Location::InternalBuffer,
                        Location::Trash
                    ),
                    StepNodeType::TransferCards(
                        false, 
                        EffectedPlayers::You,
                        None,
                        Location::InternalBuffer,
                        Location::Discard
                    ),
                    StepNodeType::TransferCards(
                        true, 
                        EffectedPlayers::You, 
                        None,
                        Location::InternalBuffer,
                        Location::DeckTop,
                    )
                ])
            ],
        }
    }

    pub fn poacher() -> Card {
        Card {
            name: "Poacher".to_owned(),
            expansion: Expansion::Dominion,
            on_gain: None,
            cost: vec![Cost::Coin(4)],
            card_type: vec![
                CardType::Action(vec![
                    StepNodeType::DrawCard(RuntimeValue::FixedValue(1)),
                    StepNodeType::PlusAction(RuntimeValue::FixedValue(1)),
                    StepNodeType::PlusCoin(RuntimeValue::FixedValue(1)),
                    StepNodeType::DiscardCard(RuntimeValue::NumberOfEmptySupplyPiles)
                ])
            ],
        }
    }

    pub fn throne_room() -> Card {
        Card {
            name: "Throne Room".to_owned(),
            expansion: Expansion::Dominion,
            on_gain: None,
            cost: vec![Cost::Coin(4)],
            card_type: vec![
                CardType::Action(vec![
                    StepNodeType::PlayCardXTimes(
                        RuntimeValue::FixedValue(2), 
                        vec![CardFilter::NextCardPlayed],
                    )
                ])
            ],
        }
    }

    pub fn remodel() -> Card {
        Card {
            name: "Remodel".to_owned(),
            expansion: Expansion::Dominion,
            on_gain: None,
            cost: vec![Cost::Coin(4)],
            card_type: vec![
                CardType::Action(vec![
                    StepNodeType::TransferCards(
                        true, 
                        EffectedPlayers::You, 
                        None,
                        Location::Hand, 
                        Location::InternalBuffer,
                    ),
                    StepNodeType::ExtractValue(
                        ExtractedValueType::CoinValue, 
                        Location::InternalBuffer, 
                        Box::new(StepNodeType::GainCard(vec![
                            CardFilter::CoinCostUpto(
                                RuntimeValue::Add(
                                    Box::new(RuntimeValue::FromAbove),
                                    Box::new(RuntimeValue::FixedValue(2)),
                                )
                            )]
                        )),
                    ),
                    StepNodeType::TransferCards(
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


    pub fn chapel() -> Card {
        Card {
            name: "Chapel".to_owned(),
            expansion: Expansion::Dominion,
            cost: vec![Cost::Coin(2)],
            on_gain: None,
            card_type: vec![CardType::Action(vec![
                StepNodeType::TransferCards(
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

    pub fn council_room() -> Card {
        Card {
            name: "Council Room".to_owned(),
            expansion: Expansion::Dominion,
            cost: vec![Cost::Coin(5)],
            on_gain: None,
            card_type: vec![
                CardType::Action(vec![
                    StepNodeType::DrawCard(RuntimeValue::FixedValue(4)),
                    StepNodeType::PlusBuy(RuntimeValue::FixedValue(1)),
                    StepNodeType::TransferCards(
                        true, 
                        EffectedPlayers::AllOthers, 
                        None, 
                        Location::DeckTop, 
                        Location::Hand,
                    )

                ]
            )],
        }
    }

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
    
    pub fn mine() -> Card {
        Card {
            name: "Mine".to_owned(),
            expansion: Expansion::Dominion,
            on_gain: None,
            cost: vec![Cost::Coin(5)],
            card_type: vec![CardType::Action(vec![
                StepNodeType::TransferCards(
                    false, 
                    EffectedPlayers::You, 
                    Some(vec![CardFilter::Type(CardType::Treasure(RuntimeValue::Any))]), 
                    Location::Hand, 
                    Location::InternalBuffer,
                ),
                StepNodeType::ExtractValue(
                    ExtractedValueType::CoinValue, 
                    Location::InternalBuffer, 
                    Box::new(StepNodeType::GainCardToHand(vec![
                        CardFilter::CoinCostUpto(
                            RuntimeValue::Add(
                                Box::new(RuntimeValue::FromAbove),
                                Box::new(RuntimeValue::FixedValue(3)),
                            )
                        ),
                        CardFilter::Type(CardType::Treasure(RuntimeValue::Any))
                    ])),
                ),
                StepNodeType::TransferCards(
                    true, 
                    EffectedPlayers::You, 
                    None, 
                    Location::InternalBuffer, 
                    Location::Trash,
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
            card_type: vec![
                CardType::Action(vec![
                    StepNodeType::DrawCard(RuntimeValue::FixedValue(1)),
                    StepNodeType::PlusAction(RuntimeValue::FixedValue(1)),
                    StepNodeType::PlusBuy(RuntimeValue::FixedValue(1)),
                    StepNodeType::PlusCoin(RuntimeValue::FixedValue(1)),
                ])
            ],
        }
    }

    pub fn moneylender() -> Card {
        Card {
            name: "Moneylender".to_owned(),
            expansion: Expansion::Dominion,
            cost: vec![Cost::Coin(4)],
            on_gain: None,
            card_type: vec![
                CardType::Action(vec![
                    StepNodeType::TransferCards(
                        false,
                        EffectedPlayers::You,
                        Some(vec![CardFilter::Name("Copper".to_owned())]),
                        Location::Hand,
                        Location::InternalBuffer,
                    ),
                    StepNodeType::ExtractValue(
                        ExtractedValueType::CardName, 
                        Location::InternalBuffer, 
                        Box::new(StepNodeType::PlusCoin(
                            RuntimeValue::Mult(
                                Box::new(RuntimeValue::FromAbove), 
                                Box::new(RuntimeValue::FixedValue(3))
                            )
                        )),
                    ),
                    StepNodeType::TransferCards(
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
            cost: vec![Cost::Coin(3)],
            on_gain: None,
            // TODO OptionalPlayAction has been removed. Use Or(Box<Step>, Box<Step>) where one is
            // None
            card_type: vec![
                CardType::Action(vec![
                    StepNodeType::PlusCoin(RuntimeValue::FixedValue(2)),

                    StepNodeType::TransferCards(
                        true,
                        EffectedPlayers::You,
                        None,
                        Location::DeckTop,
                        Location::InternalBuffer,
                    ),
                    StepNodeType::ExtractValue(
                        ExtractedValueType::CardName, 
                        Location::InternalBuffer, 
                        //Box::new(Step::OptionalPlayAction(RuntimeValue::FromAbove, None)),
                        Box::new(
                            StepNodeType::Or(
                                Box::new(StepNodeType::None), 
                                Box::new(StepNodeType::PlayCard(RuntimeValue::FromAbove)))
                            ),
                    ),
                    StepNodeType::TransferCards(
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

    pub fn workshop() -> Card {
        Card {
            name: "Workshop".to_owned(),
            expansion: Expansion::Dominion,
            cost: vec![Cost::Coin(3)],
            on_gain: None,
            card_type: vec![CardType::Action(vec![
                StepNodeType::GainCard(vec![
                    CardFilter::CoinCostUpto(RuntimeValue::FixedValue(4))
                ])
            ])],
        }
    }

    pub fn witch() -> Card {
        Card {
            name: "Witch".to_owned(),
            expansion: Expansion::Dominion,
            cost: vec![Cost::Coin(5)],
            on_gain: None,
            card_type: vec![
                CardType::Action(vec![
                    StepNodeType::DrawCard(RuntimeValue::FixedValue(2)),
                ]),
                CardType::Attack(vec![
                    StepNodeType::TransferCards(
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

    pub fn woodcutter() -> Card {
        Card {
            name: "Woodcutter".to_owned(),
            expansion: Expansion::Dominion,
            cost: vec![Cost::Coin(4)],
            on_gain: None,
            card_type: vec![
                CardType::Action(vec![
                    StepNodeType::PlusBuy(RuntimeValue::FixedValue(1)),
                    StepNodeType::PlusCoin(RuntimeValue::FixedValue(2)),
                ]),
            ]
        }
    }


}
