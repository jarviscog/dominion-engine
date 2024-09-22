mod card;
mod player;
mod hand;
mod deck;
mod step;

use game::Game;

fn main() {

    // Create game
    let game = Game::new();

    // Add players
    let player1 = player::Player::new("Player 1");
    let player2 = player::Player::new("Player 2");
    game.add_player(&mut player1);
    game.add_player(&mut player2);

    // Play some turns
    game.play_turns(2);

    // Stats can be grabbed at any time
    let game_stats = game.get_game_stats();
    let player_stats = game.get_player_stats();

    // Play the rest of the game
    game.play_to_end();

    let game_stats = game.get_game_stats();
    let player_stats = game.get_player_stats();

}
