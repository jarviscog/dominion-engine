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
    let mut game = game::Game::base_game();

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

}


// GRAVEYARD
//
// The game gets sent a Decision
// The game validates this, then sets it's state to the next
// state. This will set the next players turn, and what sort of desision they need to make
//
// Types of decisions:
// Actions - Play some list of actions, ending in either a card that needs to make a decision, or
// and EndAction
// Buy - The player chooses to buy a certain number of cards. The game validates this, and then
// buys the cards
// Discard cards - The player must discard a number of cards
