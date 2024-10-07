
# dominion-engine
> A simple game engine for the card game Dominion

This is a hobby project used to learn more about large-scale Rust projects


## Usage

No external dependencies are required. Just run `cargo run` 

Example code:
```rust
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
```

`Player` encapsulates all logic for what a player should do in a given situation.
Two important functions are used. `play_action_phase()` `play_buy_phase()`, which will both return a `Option<Vec<GameStep>>`
These functions can either use a `RuleSet` to decide on what actions to take, or get it from a different source (human input, websocket connection, etc.)

All events that happen in a game will be held in an event queue. This queue holds all information about what steps were taken throughout a game. This includes events like `BuyCard()`, `BuyPhase()` or `EndGame()`
