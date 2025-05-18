use std::cell::{Ref, RefCell};
use std::fmt;
use std::rc::Rc;

use super::*;

#[derive(Debug, Clone)]
pub struct Node {
    pub node_type: NodeType,
    /// The player that played the card
    pub player_id: usize,
    pub children: Vec<Rc<RefCell<Node>>>,
    pub visited: bool,
}

impl Node {
    pub fn new(node_type: NodeType, player_id: usize) -> Node {
        Node {
            node_type,
            player_id,
            children: Vec::new(),
            visited: false,
        }
    }

    pub fn new_full(
        node_type: NodeType,
        player_id: usize,
        children: Vec<Rc<RefCell<Node>>>,
    ) -> Node {
        Node {
            node_type,
            player_id,
            children,
            visited: false,
        }
    }

    pub fn none() -> Node {
        Node {
            node_type: NodeType::Null,
            player_id: 0,
            children: Vec::new(),
            visited: true,
        }
    }

    pub fn add_child(&mut self, child: Node) {
        self.children.push(Rc::new(RefCell::new(child)));
    }

    /// Return the first child found that has not been visited
    pub fn get_first_unvisited_child(&self) -> Option<Rc<RefCell<Node>>> {
        for child in &self.children {
            let child_borrow = child.borrow();

            // If a child is not visited, grab it
            if !child_borrow.visited {
                return Some(child.clone());
            }

            if let Some(subchild) = child_borrow.get_first_unvisited_child() {
                return Some(subchild);
            }
        }

        None
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.node_type {
            NodeType::Root => write!(f, "Root"),
            NodeType::Root => write!(f, "Root"),
            NodeType::Setup => write!(f, "Setup"),
            NodeType::Action => write!(f, "Action Phase for player: {}", self.player_id),
            NodeType::Buy => write!(f, "Buy Phase for player: {}", self.player_id),
            NodeType::Night => write!(f, "Night Phase for player: {}", self.player_id),
            NodeType::PlusCoin(x) => write!(f, "+{} 🪙", x),
            NodeType::PlusBuy(x) => write!(f, "+{} Buy", x),
            NodeType::PlusAction(x) => write!(f, "+{} Action", x),
            _ => write!(f, "{:?}", self),
        }
    }
    //write!(f, "{:?}", self)
}
