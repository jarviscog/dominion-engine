use std::cell::RefCell;
use std::fmt::{self, Write};
use std::iter;
use std::ops::IndexMut;
use std::rc::Rc;

mod choice;
mod current_game_state;
mod decision;
use colored::Colorize;

pub use choice::Choice;
pub use current_game_state::CurrentGameState;
pub use decision::Decision;

use crate::bank::Bank;
use crate::card::Card;
use crate::node::event_listener::EventListener;
use crate::node::node::Node;
pub use crate::node::*;
use crate::player::*;
use crate::runtime_values::*;
use node_template;

pub mod node_operations;


#[derive(Debug)]
pub enum BuyPhaseStep {
    PlayCard(Card),
    BuyCard(Card),
    StartNextPhase,
}

impl fmt::Display for BuyPhaseStep {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::PlayCard(c) => {
                let mut out_string: String = String::from("PlayCard\r");
                out_string.push_str(&format!("\t{:?}\r", c));
                write!(f, "{}", out_string)
            }
            Self::BuyCard(c) => {
                let mut out_string: String = String::from("BuyCard\r");
                out_string.push_str(&format!("\t{:?}\r", c));
                write!(f, "{}", out_string)
            }
            Self::StartNextPhase => {write!(f, "\nNextPhase\n")}
            //_ => write!(f, "{:?}", self),
        }
    }
}

pub struct Game {
    players: Vec<Player>,
    bank: Bank,
    turn_number: u8,
    current_state: CurrentGameState,
    current_player_index: usize,
    game_state: Vec<GameNode>,
    event_listeners: Vec<EventListener>,
}

impl Game {

    pub fn new() -> Game {
        let new_game_node = GameNode::new(
            GameNodeType::Setup,
            0,
            Vec::new(),
        );
        Game {
            players: Vec::new(),
            bank: Bank::new(),
            current_state: CurrentGameState::GameNotStarted,
            turn_number: 0,
            current_player_index: 0,
            game_state: vec![new_game_node],
            event_listeners: Vec::new(),
        }
    }

    pub fn add_terminal_player(&mut self, player_name: String) {
        self.players.push(Player::terminal(player_name));
    }

    pub fn add_bot(&mut self, bot_name: String) {
        self.players.push(Player::bot(bot_name));
    }

    /// Sets the bank
    pub fn set_bank(&mut self, in_bank: Bank) {
        match self.current_state {
            CurrentGameState::GameNotStarted => {self.bank = in_bank},
            _ => {println!("WARNING: Game already started. Cannot set the bank")}
        }
    }

    /// Starts the game, locking out the ability to add new players
    pub fn start_game(&mut self) {
        for i in 0..self.players.len() {

            // Give each player a Setup node
            let game_node = GameNode::new(
                GameNodeType::Setup,
                i.try_into().unwrap(),
                Vec::new()
            );
            self.push_game_node(game_node)
        }

        self.bank.finish_population(self.players.len());

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
    pub fn make_decision(&mut self, d: Decision) -> Result<(), String>{
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
                    //&_ => todo!()
                }
            }
            _ => panic!()
        }

        Ok(())
    }

    fn location_contains_card(&self, location: Location, card_name: String) -> bool {
        todo!()
    }

    /// Allow the current player to move a card
    pub fn play_card(&mut self, card_to_play: &Card, from: Location, play_anyways: bool) -> Result<(), String> {

        return match self.current_state {
            CurrentGameState::GameNotStarted => {
                Err("WARNING. Cannot play card. Game not started".to_owned())
            }
            // TODO remove ActionPhase and BuyPhase. Replace with GameStarted. 
            // PlayCard is not the place to check the validation of a card.
            CurrentGameState::GameRunning => {

                todo!()
            }
            CurrentGameState::GameFinished => {Err("WARNING. Cannot play card. Game over".to_owned())}
    }

    pub fn resolve_runtime_value(&self, runtime_value: RuntimeValue) -> i32 {
        match runtime_value {
            RuntimeValue::Any => i32::max_value(),
            RuntimeValue::FixedValue(val) => val,
            _ => {
                // TODO finish all match branches
                println!("WARNING: This type of value has not been finished");
                0
            }
        }
    }

    fn push_game_node(&mut self, game_node: GameNode) {
        self.game_state.push(game_node)
    }

    /// See `run_step()`
    pub fn insert_steps(&mut self, steps: Vec<StepNodeType>) {
        //println!("Running steps: ");
        for step in steps {
            println!("  {}", step);
            self.insert_step(step);
        }
    }
    
    /// Inserts a StepNodeType into the game tree
    pub fn insert_step(&mut self, in_step:StepNodeType) {
        // TODO
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
