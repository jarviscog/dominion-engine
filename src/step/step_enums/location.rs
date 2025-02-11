
use std::fmt::{self, Write};

#[derive(Debug, Clone)]
pub enum Location {
    Hand,
    Discard,

    /// The top of the deck
    DeckTop,
    SearchDeck,
    InPlay,
    Trash,
    Supply,

    // Buffer internal to the lifetime of the Card playing it
    // This location MUST be empty once the card is done being played
    InternalBuffer,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Hand => write!(f, "your hand"),
            Self::Discard => write!(f, "discard pile"),
            Self::DeckTop => write!(f, "the top of the deck"),
            Self::InPlay => write!(f, "in play"),
            Self::Trash => write!(f, "trash mat"),
            Self::InternalBuffer => write!(f, "<BUFFER>"),
            Self::Supply => write!(f, "the supply"),
            Self::SearchDeck => write!(f, "search your deck"),
            //_ => write!(f, "{:?}"),

        }
    }
    //write!(f, "{:?}", self)
}
