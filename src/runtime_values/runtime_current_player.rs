use std::fmt;

use crate::card::Location;
use crate::CardFilter;

#[derive(Debug, Clone)]
pub enum RuntimePlayerIndex {
    CurrentPlayer,
}

impl fmt::Display for RuntimePlayerIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::CurrentPlayer => write!(f, "CurrentPlayer"),
        }
    }
}
