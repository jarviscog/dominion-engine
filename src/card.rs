 use std::str::FromStr;
 

 use crate::card_type::CardType;
 use crate::cost::Cost;
 use crate::expansion::Expansion;

 use crate::step::*;

 use crate::step::location::Location;
 use crate::step::effected_players::EffectedPlayers;
 use crate::step::foreach_type::ForEachType;
 use crate::step::card_filter::CardFilter;
 use crate::step::runtime_value::RuntimeValue;
 
 
 #[derive(Debug, Clone)]
 pub struct Card {
     name: String,
     expansion: Expansion,
     card_type: Vec<CardType>,
     cost: Cost,

     action_steps: Option<Vec<Step>>, // If a card is an action card
 }
 
 impl Card {

     pub fn get_steps(&self) -> Option<Vec<Step>> {
         self.action_steps.clone()
     }

     pub fn copper() -> Card {
         Card {
             name: "Copper".to_owned(),
             expansion: Expansion::BaseGame,
             card_type: vec![CardType::Treasure(1)],
             cost: Cost::Coin(0),
             action_steps: None,
         }
     }

     pub fn silver() -> Card {
         Card {
             name: "Silver".to_owned(),
             expansion: Expansion::BaseGame,
             card_type: vec![CardType::Treasure(2)],
             cost: Cost::Coin(3),
             action_steps: None,
         }
     }

     pub fn gold() -> Card {
         Card {
             name: "Gold".to_owned(),
             expansion: Expansion::BaseGame,
             card_type: vec![CardType::Treasure(3)],
             cost: Cost::Coin(6),
             action_steps: None,
         }
     }

     pub fn estate() -> Card {
         Card {
             name: "Estate".to_owned(),
             expansion: Expansion::BaseGame,
             card_type: vec![CardType::Victory(1)],
             cost: Cost::Coin(2),
             action_steps: None,
         }
     }

     pub fn duchy() -> Card {
         Card {
             name: "Duchy".to_owned(),
             expansion: Expansion::BaseGame,
             card_type: vec![CardType::Victory(3)],
             cost: Cost::Coin(5),
             action_steps: None,
         }
     }

     pub fn province() -> Card {
         Card {
             name: "Province".to_owned(),
             expansion: Expansion::BaseGame,
             card_type: vec![CardType::Victory(6)],
             cost: Cost::Coin(8),
             action_steps: None,
         }
     }

     pub fn village() -> Card {
         Card {
             name: "Village".to_owned(),
             expansion: Expansion::BaseGame,
             action_steps: Some(vec![
                 Step::DrawCard(RuntimeValue::FixedValue(1)),
                 Step::PlusAction(RuntimeValue::FixedValue(2)),
             ]),
             cost: Cost::Coin(5),
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
                     ForEachType::Card, 
                     Box::new(
                         Step::OptionalMoveXCards(
                             EffectedPlayers::You, 
                             CardFilter::Name(&"Copper"), 
                             Location::Hand,
                             Location::Trash,
                             RuntimeValue::FixedValue(1),
                         )
                     ), 
                     Box::new(Step::PlusCoin(RuntimeValue::FixedValue(3)))
                 )
             ]),
             cost: Cost::Coin(4),
             card_type: vec![CardType::Action],
         }
     }

     pub fn witch() -> Card {
         Card {
             name: "Witch".to_owned(),
             expansion: Expansion::BaseGame,
             action_steps: Some(vec![
                 Step::DrawCard(2),
                 Step::ForceMoveXCards(
                     EffectedPlayers::AllOthers, 
                     CardFilter::Name(&"Curse"), 
                     Location::Supply,
                     Location::Discard,
                     RuntimeValue::FixedValue(1),
                 )
             ]),
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
             cost: Cost::Coin(2),
             card_type: vec![CardType::Action],
         }
     }
 
 }




