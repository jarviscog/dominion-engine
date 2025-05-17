use std::fmt;
use super::*;
use crate::RuntimeValue;

#[derive(Clone, Debug)]
pub enum NodeType {
    Null,

    Root,
    Setup,
    Action,
    Buy,
    Night,

    // === Basic effects ===
    PlusAction(RuntimeValue),
    PlusCard(RuntimeValue),
    PlusBuy(RuntimeValue),
    PlusCoin(RuntimeValue),

    /// Transfer cards from one location to another
    /// Defaults to transferring one card, but can be set to more/less using filters
    /// `bool` Forced -> true, Optional -> false
    /// `EffectedPlayers` The players effected by the transfer
    /// `Option<Vec<CardFilter>>` Optional Filters for what cards need to be transferred,
    /// `Location`, From
    /// `Location`, To
    TransferCards(
        bool,
        EffectedPlayers,
        Option<Vec<CardFilter>>,
        Location,
        Location,
    ),
    DrawCard(RuntimeValue),
    DiscardCard(RuntimeValue),

    GotoActionPhase,
    ThroneRoom,
    Conditional {
        condition: Condition,
        then_branch: Vec<Node>,
        else_branch: Vec<Node>,
    },
    Choice {
        options: Vec<Node>, // e.g., "Choose one"
    },

}

impl fmt::Display for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::PlusBuy(x) => write!(f, "+{} Buy", x),
            Self::DrawCard(x) => write!(f, "+{} Card", x),
            Self::PlusAction(x) => write!(f, "+{} Action", x),
            Self::PlusCoin(x) => write!(f, "+{} 🪙", x),

            Self::TransferCards(force, effected_players, optional_filters, from, to) => {
                let mut output_string = String::new();

                output_string.push_str(match effected_players {
                    EffectedPlayers::You => "You",
                    EffectedPlayers::All => "All players",
                    EffectedPlayers::AllOthers => "All other players",
                });
                if *force {
                    output_string.push_str(" must move")
                } else {
                    output_string.push_str(" can move")
                }

                if let Some(filters) = optional_filters {
                    output_string.push_str(" card(s) ");
                } else {
                    output_string.push_str(&format!(" a card "));
                }
                output_string.push_str(&format!("from {} to {}", from, to));
                if let Some(filters) = optional_filters {
                    output_string.push_str(&format!(
                        " with the following filters: {}",
                        filters
                            .iter()
                            .map(|item| format!("{}", item))
                            .collect::<Vec<String>>()
                            .join(", ")
                    ));
                }

                write!(f, "{}", output_string)
            }
            _ => write!(f, "{:?}", self),
        }
    }
    //write!(f, "{:?}", self)
}
