#![allow(unused)]
mod card;
mod player;
mod hand;
mod deck;
mod game_step;
mod bank;

mod game;

fn main() {

    // Create game
    let mut game = game::Game::new();

    // Add players
    let player1 = player::Player::new("John");
    let player2 = player::Player::new("Cindy");
    game.add_player(player1);
    game.add_player(player2);

    // Play some turns
    game.play_turns(2);

    // Stats can be grabbed at any time
    game.print_game_stats();
    game.print_player_stats();

    // Play the rest of the game
    game.play_to_end();

    game.print_game_stats();
    game.print_player_stats();
    game.print_event_q();

}
