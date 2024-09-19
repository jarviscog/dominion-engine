use crate::{hand::Hand, step::Step, player::Player};


#[derive(Debug)]
pub struct Game {
    turn_number: u32,

    number_of_players: u8,
    players: vec![Player],

    eventq: vec![Step]
}

impl Game {

    pub fn new() -> Self {
        Game { 
            turn_number: 0, 
            number_of_players: 0, 
            players: Vec::new(),
            eventq: Vec::new(),
        }
    }

    pub fn add_player(&self, p: &mut Player) {
        self.number_of_players += 1;
        self.players.append(p);
    }

    pub fn play_turns(num_of_turns: u32) {
        // TODO play_turns
    }

    pub fn play_to_end(num_of_turns: u32) {
        // TODO play_to_end
    }

    pub fn get_game_stats() -> &str {
        // TODO get_game_stats

    }
    pub fn get_player_stats() -> &str {
        // TODO get_player_stats
    }


}
