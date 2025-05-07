use super::node_types::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct GameNode {
    pub node_type: GameNodeType,
    pub player_id: u32,
    pub children: Vec<PhaseNode>,
}

impl GameNode {
    pub fn new(node_type: GameNodeType, player_id: u32, children: Vec<PhaseNode>) -> GameNode {
        GameNode {
            node_type,
            player_id,
            children,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PhaseNode {
    pub node_type: PhaseNodeType,
    pub player_id: u32,
    pub children: Vec<StepNode>,
}

impl PhaseNode {
    pub fn new(node_type: PhaseNodeType, player_id: u32, children: Vec<StepNode>) -> PhaseNode {
        PhaseNode {
            node_type,
            player_id,
            children,
        }
    }
}


#[derive(Debug, Clone)]
pub struct StepNode {
    node_type: StepNodeType,
    /// The player that played the card
    player_id: u32, 
    is_duration: bool,
    children: Vec<StepNode>,
}

impl StepNode {
    pub fn new(node_type: StepNodeType, player_id: u32, is_duration: bool) -> StepNode {
        StepNode {
            node_type,
            player_id,
            is_duration,
            children: Vec::new(),
        }
    }

    fn add_child(mut self, child: StepNode) {
        self.children.push(child);
    }
}
impl fmt::Display for StepNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.node_type {
            StepNodeType::PlusBuy(x) => write!(f, "+{} Buy", x),
            StepNodeType::DrawCard(x) => write!(f, "+{} Card", x),
            StepNodeType::PlusAction(x) => write!(f, "+{} Action", x),
            _ => write!(f, "{:?}", self),
        }
    }
    //write!(f, "{:?}", self)
}


//#[derive(Debug, Clone)]
//pub struct EventListener {
//    // TODO implement these enums
//    duration: Duration,
//    scope: Scope,
//    effected_players: EffectedPlayers,
//    steps: Vec<Step>,
//
//}
