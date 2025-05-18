use std::fmt;

use super::NodeTemplate;
use crate::{runtime_values::RuntimeI32};

// TODO There's a lot of spots that I don't like that you have to insert an empty vec to check a
// card type. Maybe change that?
#[derive(Debug, Clone)]
pub enum CardType {
    Action(Vec<NodeTemplate>),
    Treasure(RuntimeI32),
    Victory(RuntimeI32),
    Shelter,
    Curse,
    Attack(Vec<NodeTemplate>),   // Steps to follow for the attack
    Reaction(Vec<NodeTemplate>), // Steps to follow for the reaction
}

impl fmt::Display for CardType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            //Self::Action(x) => write!(f, "{}", x),
            Self::Action(v) => {
                let mut out_string: String = String::from("Action: [");
                for step in v {
                    out_string.push_str(&format!("{}, ", step));
                }
                out_string.push_str("]");
                write!(f, "{}", out_string)
            }
            Self::Attack(v) => {
                let mut out_string: String = String::from("Action: [");
                for step in v {
                    out_string.push_str(&format!("{}, ", step));
                }
                out_string.push_str("]");
                write!(f, "{}", out_string)
            }
            Self::Reaction(v) => {
                let mut out_string: String = String::from("Action: [");
                for step in v {
                    out_string.push_str(&format!("{}, ", step));
                }
                out_string.push_str("]");
                write!(f, "{}", out_string)
            }
            Self::Treasure(v) => write!(f, "{}", v),
            Self::Victory(v) => write!(f, "{}", v),
            Self::Curse => write!(f, "Curse"),
            Self::Shelter => write!(f, "Shelter"),
            //_ => write!(f, "{:?}", self),
        }
    }
}
