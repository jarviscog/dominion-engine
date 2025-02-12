
use std::fmt;

use crate::step::Step;
use crate::step::RuntimeValue;


#[derive(Debug, Clone)]
pub enum CardType {
    Action(Vec<Step>),
    Treasure(RuntimeValue), // Steps to follow to get the treasure value
    Victory(RuntimeValue), // Steps to follow to get the victory point value
    Curse,
    Attack(Vec<Step>), // Steps to follow for the attack
    Reaction(Vec<Step>), // Steps to follow for the reaction
}

impl fmt::Display for CardType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            //Self::Action(x) => write!(f, "{}", x),
            _ => write!(f, "{:?}", self),
        }
    }
}
