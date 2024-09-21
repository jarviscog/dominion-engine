use crate::{hand::Hand, step::Step, player::Player};


enum GameError {
    GameStarted,
}

#[derive(Debug)]
pub struct Game {
    game_started: bool,
    turn_number: u32,
    next_player_turn: u8, // The index of the player who's turn is next up

    number_of_players: u8,
    players: vec![Player],

    eventq: vec![Step]
}

impl Game {

    pub fn new() -> Self {
        Game { 
            game_started: false,
            turn_number: 0, 
            next_player_turn: 0,
            number_of_players: 0, 
            players: Vec::new(),
            eventq: Vec::new(),
        }
    }

    pub fn add_player(&self, p: &mut Player) {
        // TODO Stop adding players after game start
        self.number_of_players += 1;
        self.players.append(p);
    }

    pub fn play_turn(&self, num_of_turns: u32) {
        for i in 0..num_of_turns {
            self.eventq.append(Step::NextTurn(i));

            self.eventq.append(Step::ActionPhase());
            for step in self.players.get(self.next_player_turn).unwrap().play_action_phase() {
                self.eventq.append(step);
            }

            self.eventq.append(Step::BuyPhase());
            for step in self.players.get(self.next_player_turn).unwrap().play_buy_phase() {
                self.eventq.append(step);
            }

            self.next_player_turn = (self.next_player_turn + 1) % self.players.count()
        }

    }

    // Plays one time around the table, giving every player a turn
    pub fn play_round(&self, num_of_turns: u32) {
        self.play_turn(self.players.count())
    }

    pub fn play_to_end(&self, &self, &self, &self, &self, &self, &self, &self, num_of_turns: u32) {
        // TODO play_to_end
    }

    pub fn get_game_stats(&self) -> &str {
        // TODO get_game_stats

    }
    pub fn get_player_stats(&self) -> &str {
        // TODO get_player_stats
    }


}
