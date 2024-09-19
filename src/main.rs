mod card;
mod hand;
mod deck;
mod step;
mod game;
mod player;
mod archetypes;

use card;

fn main() {

    // Create game
    let game = Game::new();

    // Add players
    let player1 = player::archetypes::Money::new();
    let player2 = player::archetypes::Engine::new();
    game.add_player(&mut player1);
    game.add_player(&mut player2);

    // Play a turn
    game.play_turns(2);

    // Stats can be grabbed at any time
    let game_stats = game.get_game_stats();
    let player_stats = game.get_player_stats();

    // Play the rest of the game
    game.play_to_end();

    let game_stats = game.get_game_stats();
    let player_stats = game.get_player_stats();



}
