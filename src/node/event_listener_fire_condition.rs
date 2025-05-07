use super::node_enums::CardFilter;

// Comment
// https://wiki.dominionstrategy.com/index.php/Triggered_effects
#[derive(Debug, Clone)]
pub enum EventListenerFireCondition {
    StartOfNextTurn,
    StartOfNextActionPhase,
    EndOfNextActionPhase,
    StartOfNextBuyPhase,
    EndOfNextBuyPhase,
    WhenYouGainThisCard,
    WhenYouPlayCard(Vec<CardFilter>),
}
