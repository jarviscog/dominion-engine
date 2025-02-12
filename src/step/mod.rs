
use std::fmt::{self, Write};


mod step_enums;
pub use step_enums::*;

/// A step to follow on a card
/// The steps to follow will generate a tree which must be resolved in order, b
#[derive(Debug, Clone)]
pub enum Step {

    PlusCoin(RuntimeValue),
    PlusAction(RuntimeValue),
    PlusBuy(RuntimeValue),

    /// Shorthand for TransferCards(true, You, [], Deck, Hand)
    DrawCard(RuntimeValue),
    /// Shorthand for TransferCards(true, You, [], Hand, Discard)
    DiscardCard(RuntimeValue),
    /// Gain a card from the supply piles
    /// `Vec<CardFilter>` limit the options that the player can choose from
    GainCard(Vec<CardFilter>),
    /// Gain a card to your hand from the supply piles
    /// `Vec<CardFilter>` limit the options that the player can choose from
    GainCardToHand(Vec<CardFilter>),
    
    /// For each of one step, do another step. This could be based on the number of cards you use,
    /// the amount, or something else
    /// DEPRECATED
    /// ForEach(ForEachType, Box<Step>, Box<Step>),

    /// Let the player decide if they would like to play an action card or not (e.g. Vassal)
    /// `RuntimeValue` name of the card to be played
    /// `Option<Step>` If the player does play the card, optionally do another step
    OptionalPlayAction(RuntimeValue, Option<Box<Step>>),

    /// Transfer cards from one location to another
    /// Defaults to transferring one card, but can be set to more/less using filters
    /// `bool` Forced -> true, Optional -> false
    /// `EffectedPlayers` The players effected by the transfer,
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
    // Gain a Silver onto your deck.
    //      TransferCards(true, You, Name("Silver"), Supply, Deck)
    // Look through your discard pile. You may put a card from it onto your deck.
    //      TransferCards(false, You, CardCountEquals(1), SearchDiscard, DeckTop)
    // Trash up to x cards from your hand.
    //      TransferCards(false, You, CardCountUpto(x), Hand, Trash)
    // Gain a card costing up to $x.
    //      TransferCards(true, You, CostUpto(x), Supply, Trash)
    // Each other player draws a card.
    //      TransferCards(true, AllOthers, CardCountEquals(1), Supply, Trash)
    // Put a card from your hand onto your deck.
    //      TransferCards(true, You, CardCountEquals(1), Hand, DeckTop)
    // Each other player gains a Curse.
    //      TransferCards(true, AllOthers, [CardCountEquals(1), Name("Curse")], Supply, Discard)
    // Each other player discards down to 3 cards in hand.
    //      TransferCards(true, AllOthers, DownTo(3), Hand, Discard)
    // Discard a card per empty Supply pile.
    //      TransferCards(true, You, CardCountEquals(NumberOfEmpty), Hand, Discard)
    // Gain a Gold. 
    //      TransferCards(true, You, Name("Gold"), Supply, Discard)
    // Trash this card. Gain a card costing up to $5.
    //      TransferCards(true, You, ThisCard, InPlay, Trash)
    //      TransferCards(true, You, CostUpto(5), Supply, Discard)
    
    /// Apply some step based on some condition in a pile
    /// `ExtractedValueType` The value to extract. This could be number of cards, value of cards, etc.
    /// `Location` The location to extract the value from
    /// `Step` The step that will use the resulting value. This will use RuntimeValue::FromAbove
    /// e.x. 
    ///     (Number of cards in deck)/10 -> VP for gardens
    ///     (number of cards in InternalBuffer) -> +# Card for Cellar
    ///     (cost of cards in InternalBuffer)+3 -> +# Card for Mine
    ExtractValue(ExtractedValueType, Location, Box<Step>),

    /// https://wiki.dominionstrategy.com/index.php/Throne_Room_variant
    PlayCardXTimes(RuntimeValue, CardFilter),
    // You may play an Action card from your hand twice.
    //      PlayCardXTimes(2, Type(Action))

    RepeatUntil(Condition, Box<Step>),

    //And(Box<Step>, Box<Step>),
    /// Choose between two steps
    Or(Box<Step>, Box<Step>),

}


impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::PlusBuy(x) => write!(f, "+{} Buy", x),
            Self::DrawCard(x) => write!(f, "+{} Card", x),
            Self::PlusAction(x) => write!(f, "+{} Action", x),
            Self::GainCard(x) => {
                write!(
                    f, 
                    "Gain card with filters: {}", 
                    x.iter().map(|item| format!("{}", item)).collect::<Vec<String>>().join(", ")
                )
            },
            Self::GainCardToHand(x) => {
                write!(
                    f, 
                    "Gain card to your hand with filters: {}", 
                    x.iter().map(|item| format!("{}", item)).collect::<Vec<String>>().join(", ")
                )
            },
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
                } else {
                    output_string.push_str(&format!(" a"));
                }
                output_string.push_str(&format!(" card(s) from {} to {}", from, to));
                if let Some(filters) = optional_filters {
                    output_string.push_str(
                        &format!(" with the following filters: {}", 
                            filters.iter().map(|item| format!("{}", item)).collect::<Vec<String>>().join(", ")
                        )
                    );
                } 
                
                write!(f, "{}", output_string)

            }
            Self::ExtractValue(value_to_extract, location_to_extract_from, step_to_play) => {
                let mut output_string = String::from(format!("{}", step_to_play));
                let function = format!("{:?}({})", value_to_extract, location_to_extract_from);
                output_string = output_string.replace("FromAbove", &function);
                write!(f, "  {}", output_string)
            }
            _ => write!(f, "{:?}", self),

        }
    }
    //write!(f, "{:?}", self)
}


// Each other player reveals a Victory card from their hand and puts it onto their deck (or reveals a hand with no Victory cards).


// TRANSFERRING A CARD PROVIDES SOME BENEFIT
// You may trash a Copper from your hand for +$3.
//      TransferCards(false, You, Name("Copper"), Hand, InternalBuffer)
//      ExtractValue(CardCount, InternalBuffer, PlusCoin(3*CardCount))
//      TransferCards(true, You, [], InternalBuffer, Trash)
// Discard any number of cards. +1 Card per card discarded.
//      TransferCards(false, You, All, Hand, InternalBuffer)
//      ExtractValue(CardCount, InternalBuffer, DrawCard(1))
//      TransferCards(true, You, [], InternalBuffer, Discard)
// Discard the top card of your deck. If it's an Action card, you may play it.
//      TransferCards(true, You, [], DeckTop, InternalBuffer)
//      ExtractValue(CardName, InternalBuffer, OptionalPlayAction(FromAbove, None))
//      TransferCards(true, You, [], InternalBuffer, Discard)
// You may trash a Treasure from your hand. Gain a Treasure to your hand costing up to $3 more than it.
//      TransferCards(false, You, Type(Treasure), Hand, InternalBuffer)
//      ExtractValue(CardCost, InternalBuffer, GainCard([CardType(Treasure),
//      CardValueUpto(Add(FromAbove, 3)]))
//      TransferCards(true, You, [], InternalBuffer, Trash)
// Trash a card from your hand. Gain a card costing up to $2 more than it.
//      TransferCards(true, You, [], Hand, InternalBuffer)
//      ExtractValue(CardCost, InternalBuffer, GainCard(CardValueUpto(Add(FromAbove, 2)))
//      TransferCards(true, You, [], InternalBuffer, Trash)
// Each other player reveals the top 2 cards of their deck, trashes a revealed Treasure other than Copper, and discards the rest.
// Look at the top 2 cards of your deck. Trash and/or discard any number of them. Put the rest back on top in any order.
// The first time you play a Silver this turn, +$1.


// REACTION
// When another player plays an Attack card, you may first reveal this from your hand, to be unaffected by it.





// Draw until you have 7 cards in hand, skipping any Action cards you choose to; set those aside, discarding them afterwards.


// From version 1
// You may immediately put your deck into your discard pile.
// Each player (including you) reveals the top card of their deck and either discards it or puts it back, your choice.
// Each other player reveals the top 2 cards of their deck. If they revealed any Treasure cards, they trash one of them that you choose. You may gain any or all of these trashed cards. They discard the other revealed cards.
// Reveal cards from your deck until you reveal 2 Treasure cards. Put those Treasure cards into your hand and discard the other revealed cards.
