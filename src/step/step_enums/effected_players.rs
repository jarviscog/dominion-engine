
/// Players effected by a step
#[derive(Debug, Clone)]
pub enum EffectedPlayers {
    You,
    All, // All players, including the one playing it
    AllOthers, // All players that didn't play the card
}

