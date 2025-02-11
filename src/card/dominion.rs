use super::*;

impl Card {

    pub fn village() -> Card {
        Card {
            name: "Village".to_owned(),
            expansion: Expansion::BaseGame,
            action_steps: Some(vec![
                Step::DrawCard(RuntimeValue::FixedValue(1)),
                Step::PlusAction(RuntimeValue::FixedValue(2)),
            ]),
            treasure_value: 0,
            cost: Cost::Coin(5),
            card_type: vec![CardType::Action],
        }
    }

    pub fn mine() -> Card {
        Card {
            name: "Mine".to_owned(),
            expansion: Expansion::BaseGame,
            action_steps: Some(vec![
                Step::OptionalMoveXCards(
                    EffectedPlayers::You, 
                    CardFilter::Type(CardType::Treasure),
                    Location::Hand,
                    Location::Discard,
                    RuntimeValue::FixedValue(1),
                )
            ]),
            treasure_value: 0,
            cost: Cost::Coin(3),
            card_type: vec![CardType::Action],
        }
    }

    pub fn witch() -> Card {
        Card {
            name: "Witch".to_owned(),
            expansion: Expansion::BaseGame,
            action_steps: Some(vec![
                Step::DrawCard(RuntimeValue::FixedValue(2)),
                Step::ForceMoveXCards(
                    EffectedPlayers::AllOthers, 
                    CardFilter::Name("Curse".to_owned()), 
                    Location::Supply,
                    Location::Discard,
                    RuntimeValue::FixedValue(1),
                )
            ]),
            treasure_value: 0,
            cost: Cost::Coin(5),
            card_type: vec![
                CardType::Action,
                CardType::Attack
            ],
        }
    }

    pub fn chapel() -> Card {
        Card {
            name: "Chapel".to_owned(),
            expansion: Expansion::BaseGame,
            action_steps: Some(vec![
                Step::OptionalMoveXCards(
                    EffectedPlayers::You, 
                    CardFilter::All,
                    Location::Hand,
                    Location::Trash,
                    RuntimeValue::FixedValue(4),
                )
            ]),
            treasure_value: 0,
            cost: Cost::Coin(2),
            card_type: vec![CardType::Action],
        }
    }

    pub fn council_room() -> Card {
        Card {
            name: "Council Room".to_owned(),
            expansion: Expansion::BaseGame,
            action_steps: Some(vec![
                Step::DrawCard(RuntimeValue::FixedValue(4)),
                Step::PlusBuy(RuntimeValue::FixedValue(1)),
                Step::ForceMoveXCards(
                    EffectedPlayers::AllOthers, 
                    CardFilter::All,
                    Location::DeckTop, 
                    Location::Hand,
                    RuntimeValue::FixedValue(1),
                )
            ]),
            treasure_value: 0,
            cost: Cost::Coin(2),
            card_type: vec![CardType::Action],
        }
    }

    pub fn cellar() -> Card {
        Card {
            name: "Cellar".to_owned(),
            expansion: Expansion::BaseGame,
            action_steps: Some(vec![
                Step::PlusAction(RuntimeValue::FixedValue(1)),
                
                Step::ForEach(
                    ForEachType::CardCount, 
                    Box::new(Step::OptionalMoveXCards(
                        EffectedPlayers::You, 
                        CardFilter::All,
                        Location::Hand,
                        Location::Discard,
                        RuntimeValue::Unlimited,
                    )),
                    Box::new(Step::DrawCard(RuntimeValue::FromAbove)),
                ),
            ]),
            treasure_value: 0,
            cost: Cost::Coin(2),
            card_type: vec![CardType::Action],
        }
    }

    pub fn poacher() -> Card {
        Card {
            name: "Poacher".to_owned(),
            expansion: Expansion::BaseGame,
            action_steps: Some(vec![
                Step::DrawCard(RuntimeValue::FixedValue(1)),
                Step::PlusAction(RuntimeValue::FixedValue(1)),
                Step::PlusCoin(RuntimeValue::FixedValue(1)),
                Step::DiscardCard(RuntimeValue::NumberOfEmptySupplyPiles),
            ]),
            treasure_value: 0,
            cost: Cost::Coin(4),
            card_type: vec![CardType::Action],
        }
    }

    pub fn workshop() -> Card {
        Card {
            name: "Workshop".to_owned(),
            expansion: Expansion::BaseGame,
            action_steps: Some(vec![
                Step::ForceMoveXCards(
                    EffectedPlayers::You, 
                    CardFilter::UpToValue(RuntimeValue::FixedValue(4)),
                    Location::Supply,
                    Location::Discard,
                    RuntimeValue::FixedValue(1),
                )
            ]),
            treasure_value: 0,
            cost: Cost::Coin(3),
            card_type: vec![CardType::Action],
        }
    }

    pub fn market() -> Card {
        Card {
            name: "Market".to_owned(),
            expansion: Expansion::BaseGame,
            action_steps: Some(vec![
                Step::DrawCard(RuntimeValue::FixedValue(1)),
                Step::PlusAction(RuntimeValue::FixedValue(1)),
                Step::PlusBuy(RuntimeValue::FixedValue(1)),
                Step::PlusCoin(RuntimeValue::FixedValue(1)),
            ]),
            treasure_value: 0,
            cost: Cost::Coin(5),
            card_type: vec![CardType::Action],
        }
    }

    pub fn festival() -> Card {
        Card {
            name: "Festival".to_owned(),
            expansion: Expansion::BaseGame,
            action_steps: Some(vec![
                Step::PlusAction(RuntimeValue::FixedValue(2)),
                Step::PlusBuy(RuntimeValue::FixedValue(1)),
                Step::PlusCoin(RuntimeValue::FixedValue(2)),
            ]),
            treasure_value: 0,
            cost: Cost::Coin(5),
            card_type: vec![CardType::Action],
        }
    }

    pub fn moneylender() -> Card {
        Card {
            name: "Moneylender".to_owned(),
            expansion: Expansion::BaseGame,
            action_steps: Some(vec![
                Step::ForEach(
                    ForEachType::CardCount, 
                    Box::new(
                        Step::OptionalMoveXCards(
                            EffectedPlayers::You, 
                            CardFilter::Name("Copper".to_owned()), 
                            Location::Hand,
                            Location::Trash,
                            RuntimeValue::FixedValue(1),
                        )
                    ), 
                    Box::new(Step::PlusCoin(RuntimeValue::FixedValue(3)))
                )
            ]),
            treasure_value: 0,
            cost: Cost::Coin(4),
            card_type: vec![CardType::Action],
        }
    }

    pub fn vassal() -> Card {
        Card {
            name: "Vassal".to_owned(),
            expansion: Expansion::BaseGame,
            action_steps: Some(vec![
                Step::PlusCoin(RuntimeValue::FixedValue(2)),
                Step::ForEach(
                    ForEachType::Card, 
                    Box::new(Step::ForceMoveXCards(
                        EffectedPlayers::You, 
                        CardFilter::All, 
                        Location::DeckTop, 
                        Location::Discard, 
                        RuntimeValue::FixedValue(1),
                    )),
                    Box::new(Step::PlayCardXTimes(
                        RuntimeValue::FixedValue(1),
                        CardFilter::FromAbove, 
                    ))
                )

            ]),
            treasure_value: 0,
            cost: Cost::Coin(3),
            card_type: vec![
                CardType::Action,
                CardType::Attack
            ],
        }
    }

}
