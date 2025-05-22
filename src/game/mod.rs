use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{self, Write};
use std::ops::IndexMut;
use std::rc::Rc;
use std::{iter, usize};

mod choice;
mod current_game_state;
mod decision;
use colored::Colorize;

pub use choice::Choice;
pub use current_game_state::CurrentGameState;
pub use decision::Decision;

use crate::bank::Bank;
use crate::card::Card;
use crate::card_type::CardType;
use crate::node::event_listener::EventListener;
use crate::node::node::Node;
pub use crate::node::*;
use crate::player::*;
use crate::runtime_values::*;
use node_template;

pub mod node_operations;

pub struct Game {
    turn_number: u8,
    next_node_id: usize,
    players: Vec<Player>,
    bank: Bank,
    current_state: CurrentGameState,
    current_player_index: usize,
    context_map: HashMap<String, Vec<Card>>,
    game_state: Rc<RefCell<Node>>,
    current_node: Rc<RefCell<Node>>,
    event_listeners: Vec<EventListener>,
}

impl Game {
    pub fn new() -> Game {
        let root_node = Rc::new(RefCell::new(Node::new(NodeType::Root, 0)));
        Game {
            turn_number: 0,
            next_node_id: 1,
            players: Vec::new(),
            bank: Bank::new(),
            current_state: CurrentGameState::GameNotStarted,
            current_player_index: 0,
            context_map: HashMap::new(),
            game_state: Rc::clone(&root_node),
            current_node: Rc::clone(&root_node),
            event_listeners: Vec::new(),
        }
    }

    pub fn add_terminal_player(&mut self, player_name: String) {
        self.players.push(Player::terminal(player_name));
    }

    pub fn add_bot(&mut self, bot_name: String) {
        self.players.push(Player::bot(bot_name));
    }

    pub fn set_bank(&mut self, in_bank: Bank) {
        match self.current_state {
            CurrentGameState::GameNotStarted => self.bank = in_bank,
            _ => {
                println!("WARNING: Game already started. Cannot set the bank")
            }
        }
    }

    pub fn set_game_state(&mut self, node: Node) {
        self.game_state = Rc::new(RefCell::new(node))
    }

    /// Starts the game, locking out the ability to add new players
    pub fn start_game(&mut self) {
        for i in 0..self.players.len() {
            println!("Setup for {i}");
            // Give each player a Setup node
            let game_node = self.resolve_template(NodeTemplate::Setup);
            self.current_node
                .borrow_mut()
                .children
                .push(Rc::new(RefCell::new(game_node)));
            self.current_player_index = i;
        }
        self.current_player_index = 0;
        self.game_state.borrow_mut().visited = true;
        self.bank.finish_population(self.players.len());

        // Give an action phase to player 1
        self.current_node
            .borrow_mut()
            .add_child(Node::new(NodeType::Action, 0));

        self.current_state = CurrentGameState::GameRunning;
    }

    /// The index of the player who's turn it currently is
    pub fn get_current_player_index(&self) -> usize {
        // TODO this should potentially return a Result<usize, Err>
        self.current_player_index
    }

    /// The name of the player who's turn it currently is
    pub fn get_current_player_name(&self) -> String {
        if let Some(player) = self.players.get(self.current_player_index) {
            player.get_name()
        } else {
            panic!("Error getting player name");
        }
    }

    pub fn get_next_unvisited_node(&self) -> Option<Rc<RefCell<Node>>> {
        let next_node = self.game_state.borrow().get_first_unvisited_child();
        next_node
    }

    /// Gets the next available node, and runs it
    /// This may trigger a transition, such as ending a turn or going from action phase to buy
    /// phase
    pub fn run_next_node(&mut self) {
        let maybe_next_node = self.get_next_unvisited_node();

        if let Some(next_node) = maybe_next_node {
            // Calculate any transition that may have occurred
            //self.handle_transition(self.current_node.clone(), next_node.clone());

            println!("Running node: {}", next_node.borrow().clone());
            let node_template = next_node.borrow().node_type.clone();
            next_node.borrow_mut().visited = true;
            self.current_node = next_node.clone();
            self.apply_step(next_node.borrow().clone());
        } else {
            println!("INFO: No more nodes");
            // All of the nodes in the tree have been completed. We need to add more nodes
            match self.get_last_phase() {
                NodeType::Action => {
                    let next_phase = Node::new(NodeType::Buy, self.current_player_index);
                    self.game_state.borrow_mut().add_child(next_phase);
                }
                NodeType::Buy => {
                    // TODO this needs to wrap around
                    self.current_player_index += 1;
                    self.turn_number += 1;
                    let next_phase = Node::new(NodeType::Action, self.current_player_index);
                    self.game_state.borrow_mut().add_child(next_phase);
                }
                _ => panic!("Unexpected node returned: {:?}", self.get_last_phase()),
            }
        }
    }

    /// Get the most recently played phase
    fn get_last_phase(&self) -> NodeType {
        let mut second_layer_nodes = self.game_state.borrow().children.clone();
        second_layer_nodes.reverse();
        for node in second_layer_nodes {
            match node.borrow().node_type {
                NodeType::Action => return NodeType::Action,
                NodeType::Buy => return NodeType::Buy,
                _ => {}
            }
        }
        NodeType::Null
    }

    /// Handle any transitions that may occur from a node that has fired, to the next node
    /// A Transition is a spot where an EventHandler could potentially fire. This will be a border
    /// between nodes, usually
    fn handle_transition(&mut self, old_node: Rc<RefCell<Node>>, new_node: Rc<RefCell<Node>>) {}

    fn fire_event_listener(
        &mut self,
        event_listener_destruct_condition: EventListenerDestructCondition,
    ) {
        for event_listener in self.event_listeners.clone() {
            if event_listener.destruct_condition == event_listener_destruct_condition {
                todo!()
            }
        }
    }

    /// Add NodeTemplates into the current Node
    pub fn insert_into_current_node(&mut self, node_steps: Vec<NodeTemplate>) {
        for step in node_steps {
            let new_node = self.resolve_template(step);
            self.current_node.borrow_mut().add_child(new_node);
        }
    }

    // Visit a node, and apply any atomic operations you might need to
    fn apply_step(&mut self, in_node: Node) {
        let node = in_node.clone();
        match &node.node_type {
            NodeType::Setup => {}
            NodeType::Action => {}
            NodeType::Buy => {}
            NodeType::PlusCoin(runtime_value) => {
                let value = self.resolve_runtime_i32(runtime_value);
                self.atomic_add_coins(node.player_id, value);
            }
            NodeType::PlusAction(runtime_value) => {
                let value = self.resolve_runtime_i32(runtime_value);
                self.atomic_add_actions(node.player_id, value);
            }
            NodeType::PlusBuy(runtime_value) => {
                let value = self.resolve_runtime_i32(runtime_value);
                self.atomic_add_buys(node.player_id, value);
            }
            NodeType::TransferCards(forced, effected_players, filters, from, to) => {
                if !forced {
                    panic!("Optional transfer not implemented")
                }
                for index in self.resolve_effected_players(effected_players).iter() {}
            }
            _ => todo!("apply_step: {}", &node), // ...
        }
    }

    fn atomic_draw_cards(&mut self, player_id: usize, number_of_cards: i32) {
        if let Some(player) = self.players.get_mut(player_id as usize) {
            player.draw_cards(number_of_cards as u32);
        } else {
            println!("ERROR: Failed to index player_id: {player_id}")
        }
    }

    fn atomic_add_coins(&mut self, player_id: usize, amount: i32) {
        if let Some(player) = self.players.get_mut(player_id as usize) {
            player.add_coins(amount);
        } else {
            println!("ERROR: Failed to index player_id: {player_id}")
        }
    }

    fn atomic_add_actions(&mut self, player_id: usize, amount: i32) {
        if let Some(player) = self.players.get_mut(player_id as usize) {
            player.add_actions(amount);
        } else {
            println!("ERROR: Failed to index player_id: {player_id}")
        }
    }

    fn atomic_add_buys(&mut self, player_id: usize, amount: i32) {
        if let Some(player) = self.players.get_mut(player_id as usize) {
            player.add_buys(amount);
        } else {
            println!("ERROR: Failed to index player_id: {player_id}")
        }
    }

    fn atomic_add_debt(&mut self, player_id: usize, amount: i32) {
        if let Some(player) = self.players.get_mut(player_id as usize) {
            player.add_debt(amount);
        } else {
            println!("ERROR: Failed to index player_id: {player_id}")
        }
    }

    pub fn add_action_phase_with_steps(&mut self, node_template: Vec<NodeTemplate>) {}

    fn resolve_template(&mut self, node_template: NodeTemplate) -> Node {
        let current_player_index = self.get_current_player_index();
        match node_template {
            NodeTemplate::Root => Node::new(NodeType::Root, current_player_index),
            NodeTemplate::Setup => {
                let mut setup_node = Node::new(NodeType::Setup, current_player_index);
                setup_node.add_child(Node::new(
                    NodeType::TransferCards(
                        true,
                        EffectedPlayers::You,
                        Some(vec![CardFilter::CardCountEquals(RuntimeI32::Const(5))]),
                        Location::DeckTop,
                        Location::Hand,
                    ),
                    0,
                ));
                setup_node
            }
            NodeTemplate::Action => Node::new(NodeType::Action, current_player_index),
            NodeTemplate::Buy => Node::new(NodeType::Buy, current_player_index),
            NodeTemplate::Night => Node::new(NodeType::Night, current_player_index),
            NodeTemplate::PlusAction(runtime_i32) => {
                Node::new(NodeType::PlusAction(runtime_i32), current_player_index)
            }
            NodeTemplate::PlusCoin(runtime_i32) => {
                Node::new(NodeType::PlusCoin(runtime_i32), current_player_index)
            }
            NodeTemplate::PlusBuy(runtime_i32) => {
                Node::new(NodeType::PlusBuy(runtime_i32), current_player_index)
            }
            NodeTemplate::DrawCard(runtime_i32) => Node::new(
                NodeType::TransferCards(
                    true,
                    EffectedPlayers::You,
                    Some(vec![CardFilter::CardCountEquals(runtime_i32)]),
                    Location::DeckTop,
                    Location::Hand,
                ),
                current_player_index,
            ),
            NodeTemplate::Conditional(condition, then_branch, else_branch) => {
                if self.resolve_condition(&condition) {
                    self.resolve_template(*then_branch)
                } else {
                    self.resolve_template(*else_branch)
                }
            }
            NodeTemplate::None => Node::none(),
            _ => todo!("resolve_template {:?}", node_template),
        }
    }

    // Returns indexes for the players that should be effected
    fn resolve_effected_players(&self, effected_players: &EffectedPlayers) -> Vec<usize> {
        let mut ret_vec = Vec::new();
        match effected_players {
            EffectedPlayers::You => ret_vec.push(self.get_current_player_index()),
            EffectedPlayers::All => {
                for i in 0..self.players.len() {
                    ret_vec.push(i)
                }
            }
            EffectedPlayers::AllOthers => {
                for i in 0..self.players.len() {
                    ret_vec.push(i)
                }
                ret_vec.retain(|value| *value != self.get_current_player_index());
            }
        }
        ret_vec
    }

    fn resolve_runtime_i32(&self, val: &RuntimeI32) -> i32 {
        return match val {
            RuntimeI32::Const(x) => x.clone(),
            RuntimeI32::Add(x, y) => self.resolve_runtime_i32(x) + self.resolve_runtime_i32(y),
            RuntimeI32::Mult(x, y) => self.resolve_runtime_i32(x) * self.resolve_runtime_i32(y),
            _ => todo!(),
        };
    }

    fn resolve_runtime_card_name(&self, name: &RuntimeCardName) -> String {
        return match name {
            RuntimeCardName::Const(str_name) => str_name.to_string(),
            RuntimeCardName::FromContext(context_str) => {
                if let Some(card) = self.context_map.get(context_str) {
                    card.first().unwrap().get_name()
                } else {
                    panic!("Could not find card in context! {context_str}")
                }
            }
        };
    }

    fn resolve_runtime_card_type(&self, in_type: &RuntimeCardType) -> Vec<CardType> {
        return match in_type {
            RuntimeCardType::Const(const_type) => vec![const_type.clone()],
            RuntimeCardType::FromContext(context_str) => {
                if let Some(card) = self.context_map.get(context_str) {
                    card.first().unwrap().get_card_types()
                } else {
                    panic!("Could not find card in context! {context_str}")
                }
            }
        };
    }

    fn resolve_condition(&self, condition: &Condition) -> bool {
        return match condition {
            Condition::EqualI32(x, y) => self.resolve_runtime_i32(x) == self.resolve_runtime_i32(y),
            Condition::EqualCardName(x, y) => {
                self.resolve_runtime_card_name(x) == self.resolve_runtime_card_name(y)
            }
            Condition::ContextContainsCard(context) => self.context_map.contains_key(context),
            Condition::EqualStackSize(location, runtime_size) => {
                let index = self.get_current_player_index();
                let pile_size = match location {
                    Location::Hand => self.players.get(index).unwrap().get_hand().len(),
                    Location::Discard => self.players.get(index).unwrap().get_discard().len(),
                    Location::InPlay => self.players.get(index).unwrap().get_in_play().len(),
                    Location::Trash => self.bank.get_trash().len(),
                    Location::InternalBuffer => {
                        self.players.get(index).unwrap().get_internal_buffer().len()
                    }
                    Location::DeckTop => 1,
                    Location::Supply => panic!("Tried to get size of supply"),
                    Location::SearchDeck => panic!("Tried to get size of Location::SearchDeck"),
                };
                let size = self.resolve_runtime_i32(runtime_size);
                return pile_size as i32 == size;
            }
            Condition::EqualCardType(x, y) => todo!(),
            Condition::NotInBuyPhase => todo!(),
        };
    }

    pub fn get_current_player_hand(&self) -> Vec<Card> {
        if let Some(player) = self.players.get(self.current_player_index) {
            player.get_hand()
        } else {
            panic!("Error getting player printing player hand");
        }
    }

    /// Get the choice that the current player needs to make
    pub fn get_current_choice(&mut self) -> Choice {
        // TODO This should iterate through all nodes of the event_history tree and look for the
        // first node that is not resolved
        //
        // e.x.
        // ActionPhase()
        // - Play Card: Chapel
        // - - ChooseCardsToDiscard()
        // BuyPhase()
        //
        // In this case, ActionPhase and ChooseCardsToDiscard() are both decisions that need to be made at
        // some point, but the ChooseCardsToDiscard() needs to be done first
        todo!()
    }

    /// Make a decision that the game is waiting for
    pub fn make_decision(&mut self, d: Decision) -> Result<(), String> {
        // TODO pull the choice from the event_q, and ensure the decision can only be made for the
        // choice required
        match self.get_current_choice() {
            Choice::ActionPhase => {
                match d {
                    Decision::PlayCard(card) => {
                        self.play_card(&card, Location::Hand, false);
                    }
                    Decision::StartNextPhase => {
                        // TODO
                    }
                    Decision::ChooseCards(x, y, z) => {
                        return Err("Cannot choose a card during action phase".to_owned())
                    }
                    Decision::NameACard(x) => {
                        return Err("Cannot name a card during action phase".to_owned())
                    }
                    Decision::BuyCard(x) => {
                        return Err("Cannot buy a card during action phase".to_owned())
                    }
                }
            }
            Choice::BuyPhase => {
                match d {
                    Decision::PlayCard(card) => {
                        self.play_card(&card, Location::Hand, false);
                    }
                    Decision::BuyCard(card) => {
                        return self.buy_card(self.current_player_index, &card, false);
                    }
                    Decision::StartNextPhase => {
                        // TODO
                    }
                    Decision::ChooseCards(x, y, z) => {
                        return Err("Cannot choose a card during buy phase".to_owned())
                    }
                    Decision::NameACard(x) => {
                        return Err("Cannot name a card during buy phase".to_owned())
                    } //&_ => todo!()
                }
            }
            _ => panic!(),
        }

        Ok(())
    }

    fn location_contains_card(&self, location: Location, card_name: String) -> bool {
        todo!()
    }

    /// Allow the current player to move a card
    /// `play_anyways` allows you to play the card even if the game can't move it to InPlay
    pub fn play_card(
        &mut self,
        card_to_play: &Card,
        from: Location,
        play_anyways: bool,
    ) -> Result<(), String> {
        return match self.current_state {
            CurrentGameState::GameNotStarted => {
                Err("WARNING. Cannot play card. Game not started".to_owned())
            }
            // TODO remove ActionPhase and BuyPhase. Replace with GameStarted.
            // PlayCard is not the place to check the validation of a card.
            CurrentGameState::GameRunning => {
                todo!()
            }
            CurrentGameState::GameFinished => {
                Err("WARNING. Cannot play card. Game over".to_owned())
            }
        };
    }

    pub fn get_player_names(&self) -> Vec<String> {
        let mut ret_vec: Vec<String> = Vec::new();

        for player in &self.players {
            ret_vec.push(player.get_name().clone())
        }
        ret_vec
    }

    /// Returns the player at the given index
    pub fn get_player(&self, index: usize) -> Option<&Player> {
        self.players.get(index)
    }

    pub fn is_finished(&self) -> bool {
        match self.current_state {
            CurrentGameState::GameFinished => true,
            _ => false,
        }
    }

    pub fn print_player_stats(&self) {
        for player in &self.players {
            player.print_state();
        }
    }

    pub fn print_event_listeners(&self) {
        // TODO add a limit for number of entries printed
        println!("Event listeners:");
        for element in &self.event_listeners {
            println!("{:?}", element)
        }
        println!("");
    }

    pub fn print_game_state(&self) {
        println!("Game State:");
        println!("Current Player: {}, ", self.current_player_index);
        for child in &self.game_state.borrow().children {
            self.print_node(&*child.borrow(), 0);
        }
        println!("");
    }

    fn print_node(&self, node: &Node, indent: u32) {
        let repeated: String = iter::repeat(" ").take(indent as usize).collect();
        print!("{}", repeated);
        let mut node_template = node.node_type.to_string();
        if node.player_id == 0 {
            node_template = node_template.blue().to_string()
        } else if node.player_id == 1 {
            node_template = node_template.green().to_string()
        } else if node.player_id == 2 {
            node_template = node_template.red().to_string()
        }
        if !node.visited {
            node_template = node_template.on_truecolor(80, 80, 80).to_string()
        }
        println!("{}", node_template);
        for child in &node.children {
            self.print_node(&*child.borrow(), indent + 2);
        }
    }

    pub fn get_player_victory_points(&self, player: &Player) -> u32 {
        // TODO
        0
    }

    pub fn print_game_results(&self) {
        // TODO Tally VPs for each player, and print the results
        println!("End of game!");
        println!("Here are the total scores");
        for player in &self.players {
            println!(
                "Player: {} Score: {}",
                player.get_name(),
                self.get_player_victory_points(player)
            );
        }
    }
}
