
use crate::player::Player;
use crate::step::Step;
use crate::bank::Bank;
use crate::player;

/// The game may need to wait until
pub enum CurrentState {
    GameNotStarted,
    ActionPhase,
    RunningStep,
    GameFinished,
}

pub struct Game {
    players: Vec<player::Player>,
    bank: Bank,
    turn_number: u8,
    current_state: CurrentState
}

impl Game {

    pub fn new() -> Game {
        Game {
            players: Vec::new(),
            bank: Bank::new(),
            current_state: CurrentState::GameNotStarted,
            turn_number: 0,
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
            CurrentState::GameNotStarted => {self.bank = in_bank},
            _ => {println!("WARNING: Game already started. Cannot set the bank")}
        }
    }

    /// Starts the game, locking out the ability to add new players
    pub fn start_game(&mut self) {
        self.bank.finish_population(self.players.len());
        self.current_state = CurrentState::ActionPhase;
    }

    /// Runs steps contained on a card
    pub fn run_steps(&self, steps: Vec<Step>) {
        println!("Running steps: ");
        for step in steps {
            println!("  {}", step);
            self.run_step(step);
        }
    }
    
    pub fn run_step(&self, step: Step) {

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
            CurrentState::GameFinished => true,
            _ => false,
        }
    }

    pub fn print_status(&self) {
        // TODO print_status()
    }

}










