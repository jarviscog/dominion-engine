use std::usize;

use crate::{hand::Hand, game_step::GameStep, player::Player, bank::Bank};


enum GameError {
    GameStarted,
}

#[derive(Debug)]
pub struct Game {
    // Game
    game_started: bool,
    turn_number: u32,
    next_player_turn: u8, // The index of the player who's turn is next up

    // Players
    number_of_players: u8,
    players: Vec<Player>,

    // Gamestate
    bank: Bank,
    eventq: Vec<GameStep>
}

impl Game {

    pub fn new() -> Self {
        Game { 
            game_started: false,
            turn_number: 0, 
            next_player_turn: 0,
            number_of_players: 0, 
            players: Vec::new(),
            bank: Bank::base_game(),
            eventq: Vec::new(),
        }
    }

    pub fn add_player(&mut self, p: Player) {
        // TODO Stop adding players after game start
        self.number_of_players += 1;
        self.players.push(p);
    }

    pub fn play_turns(&mut self, num_of_turns: u32) {
        for i in 0..num_of_turns {

            if self.check_game_end() { return }

            self.eventq.push(GameStep::NextTurn(self.next_player_turn));

            self.eventq.push(GameStep::ActionPhase());
            let player: &Player = &self.players[self.next_player_turn as usize];
            if let Some(steps) = player.play_action_phase() {
                for step in steps {
                    self.eventq.push(step.clone());
                }
            }

            self.eventq.push(GameStep::BuyPhase());
            let player: &Player = &self.players[self.next_player_turn as usize];
            if let Some(steps) = player.play_buy_phase() {
                for step in steps {
                    self.eventq.push(step);
                }
            }

            self.next_player_turn = (self.next_player_turn + 1) % self.players.len() as u8;


        }

    }

    // Plays one time around the table, giving every player a turn
    pub fn play_rounds(&mut self, num_of_turns: u32) {
        self.play_turns(self.players.len() as u32);
    }

    pub fn play_to_end(&self) {
        // TODO play_to_end
        self.play_turns(u32::max_value());
    }

    fn check_game_end(&self) -> bool {
        // TODO: Update this for other game modes with other win conditions
        
            


    }

    pub fn print_game_stats(&self) {
        // TODO finish get_game_stats
        println!("=== Game Stats ===");
        println!("Number of players: {:?}", self.number_of_players);
        println!("Turn number: {:?}", self.turn_number);
        println!("Current tyrn: {:?}", self.turn_number);
    }

    pub fn print_player_stats(&self) {
        // TODO finish get_player_stats
        println!("=== Players: ({:?}) ===", self.number_of_players);
        for (i, player) in self.players.iter().enumerate() {
            println!("{:?}  {:?}", i, &player.to_string());
        }
        
    }

    pub fn print_event_q(&self) {
        for event in &self.eventq {
            match event {
                GameStep::NextTurn(turn) => println!("{:?}", event),

                GameStep::ActionPhase() => println!("  {:?}", event),
                GameStep::BuyPhase() => println!("  {:?}", event),

                GameStep::BuyCard(card) => println!("    BuyCard: {}", card.name()),
                _ => println!("    {:?}", event)

            }
        }
    }


}
