use std::str::FromStr;


use crate::cost::Cost;
use crate::expansion::Expansion;
use super::card_type::CardType;
use std::fmt::{self, Write};

use crate::step::*;

// functions to generate cards
pub mod dominion;
pub mod intrigue;
pub mod seaside;

#[derive(Debug, Clone)]
pub struct Card {
    name: String,
    expansion: Expansion,
    card_type: Vec<CardType>,
    on_gain: Option<Step>, // If steps are required when you gain the card
    cost: Vec<Cost>,
}

impl Card {

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_action_steps(&self) -> Option<Vec<Step>> {
        for c_type in &self.card_type {
            return match c_type {
                CardType::Action(steps) => {Some(steps.clone())}
                _ => None
            }
        }
        None
    }

    pub fn get_attack_steps(&self) -> Option<Vec<Step>> {
        for c_type in &self.card_type {
            return match c_type {
                CardType::Attack(steps) => {Some(steps.clone())}
                _ => None
            }
        }
        None
    }

    pub fn get_victory_points(&self) -> Option<RuntimeValue> {
        for c_type in &self.card_type {
            return match c_type {
                CardType::Victory(vps) => {Some(vps.clone())}
                _ => None
            }
        }
        None
    }

    pub fn get_value(&self) -> Option<RuntimeValue> {
        // TODO this function should return the value in either potions or coins.
        for c_type in &self.card_type {
            return match c_type {
                CardType::Treasure(value) => {Some(value.clone())}
                _ => None
            }
        }
        None
    }

    pub fn get_coin_cost(&self) -> u32 {
        for cost in &self.cost {
            match cost {
                Cost::Coin(x) => {return x.clone()}
                _ => {}
            }
        }
        0
    }

    pub fn get_debt_cost(&self) -> u32 {
        for cost in &self.cost {
            match cost {
                Cost::Debt(x) => {return x.clone()}
                _ => {}
            }
        }
        0
    }

}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out_string: String = String::from("{");
        out_string.push_str(&format!("\n\t\tname:{}", self.name));
        //out_string.push_str(&format!(" expansion:{:?}", self.expansion));
        out_string.push_str(&format!("\n\t\tcost:{:?}", self.cost));
        out_string.push_str(&format!("\n\t\ton_gain:{:?}", self.on_gain));

        out_string.push_str(&format!("\n\t\tcard_types:",));
        for c_type in &self.card_type {
            out_string.push_str(&format!("\n\t\t\t{}, ", c_type));
        }
        //out_string.push_str(&format!("\n\t\t]",));

        out_string.push_str("\n\t\t}");
        
        write!(f, "{}", out_string)

    }
}







