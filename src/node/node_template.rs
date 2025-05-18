use std::fmt;

use crate::card::{RuntimeCardName, RuntimeI32};

use super::*;

/// A node that has not yet been created
#[derive(Debug, Clone)]
pub enum NodeTemplate {
    // ROOT NODE
    Root,
    // GAME NODE TYPES
    Setup,
    Turn,
    ScoreCount,
    // PHASE NODE TYPES
    Action,
    Buy,
    Night,
    // CHOICE NODE TYPES
    ActionPhaseChoice,
    BuyPhaseChoice,
    // STEP NODE TYPES
    None,
    PlusCoin(RuntimeI32),
    PlusAction(RuntimeI32),
    PlusBuy(RuntimeI32),
    /// Choose between two step paths
    Or(Box<NodeTemplate>, Box<NodeTemplate>),

    /// `RuntimeValue` name of the card to be played
    PlayCard(RuntimeCardName), // Play a card without modifying location of the card
    /// Shorthand for TransferCards(true, You, [], Deck, Hand)
    DrawCard(RuntimeI32),
    /// Shorthand for TransferCards(true, You, [], Hand, Discard)
    DiscardCard(RuntimeI32),
    /// Gain a card from the supply piles
    /// `Vec<CardFilter>` limit the options that the player can choose from
    GainCard(Vec<CardFilter>),
    /// Gain a card to your hand from the supply piles
    /// `Vec<CardFilter>` limit the options that the player can choose from
    GainCardToHand(Vec<CardFilter>),

    GotoActionPhase,
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

    /// https://wiki.dominionstrategy.com/index.php/Throne_Room_variant
    /// `RuntimeValue`, number of times to play the card
    /// `Vec<CardFilter>` List of filters for the card to be played
    PlayCardXTimes(RuntimeI32, Vec<CardFilter>),

    // You may play an Action card from your hand twice.
    //      PlayCardXTimes(2, Type(Action))
    /// Repeat a step until a condition is met
    RepeatUntil(Condition, Box<NodeTemplate>),

    /// Transfer cards from one location to 2 other locations depending on filters
    /// Defaults to transferring one card, but can be set to more/less using filters
    ///
    /// `bool` `True` The transfer is forced to move to fork 1
    ///        `False` The transfer is optional. The player decides between fork1 and 2 for each
    ///        card
    ///        Either way, for cards to make it into fork1 the cards must follow to TO filter rules
    /// `EffectedPlayers` The players effected by the transfer
    /// `Option<Vec<CardFilter>>` Optional Filters to limit cards coming FROM
    /// `Location`, From
    /// `Option<Vec<CardFilter>>` Optional Filters to limit cards going TO. All other cards will go
    /// to
    /// `Location`, Fork1
    /// `Location`, Fork2
    ForkTransferCards(
        bool,
        EffectedPlayers,
        Option<Vec<CardFilter>>,
        Location,
        Option<Vec<CardFilter>>,
        Location,
    ),

    Conditional (
        Condition,
        Box<NodeTemplate>,
        Box<NodeTemplate>,
    ),

    IgnoreAttacks,

    //TrashCard,
    //GainActions,
    //PromptPlayer,
    AddEventListener(EventListener), //ProcessionEffect,
}

impl fmt::Display for NodeTemplate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::PlusBuy(x) => write!(f, "+{} Buy", x),
            Self::DrawCard(x) => write!(f, "+{} Card", x),
            Self::PlusAction(x) => write!(f, "+{} Action", x),
            Self::GainCard(x) => {
                write!(
                    f,
                    "Gain card with filters: {}",
                    x.iter()
                        .map(|item| format!("{}", item))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            Self::GainCardToHand(x) => {
                write!(
                    f,
                    "Gain card to your hand with filters: {}",
                    x.iter()
                        .map(|item| format!("{}", item))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            Self::DiscardCard(x) => write!(f, "Discard card with filters: {:?}", x),
            Self::PlusCoin(x) => write!(f, "+{} 🪙", x),
            Self::Or(step1, step2) => write!(f, "Choose:\n\r{}\n\r{}", step1, step2),
            Self::RepeatUntil(cond, s) => write!(f, "Repeat until {:?}:\n\t{}\n", cond, s),

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
