
# dominion-engine
> A simple game engine for the card game Dominion

This is a hobby project used to learn more about large-scale Rust projects


## Usage

No external dependencies are required. Just run `cargo run` 

Example code:
```rust
// Create a new game
let mut new_game = game::Game::new();

// Add players
new_game.add_terminal_player("Jarvis".to_owned());
new_game.add_bot("Bot".to_owned());
// Set up the bank to be the "First Game" preset
new_game.set_bank(bank::Bank::first_game());

// Start the game
new_game.start_game();

while !new_game.is_finished() {
    new_game.get_next_step();
    new_game.print_status();
}

let player_names = new_game.get_player_names();
println!("Players: ");
println!("  {:#?}", player_names);

println!("\nMarket:");
if let Some(steps) = card::Card::market().get_steps() {
    new_game.run_steps(steps);
}
```

`Player` encapsulates all logic for what a player should do in a given situation.  

