#![allow(unused)]

use std::io::*;
use std::io;
use std::str::FromStr;

mod game;
mod bank;
mod player;

mod expansion;

mod card;
mod card_type;
mod step;
use step::Step;

mod cost;

mod pile;

fn main() {

    let mut new_game = game::Game::new();

    new_game.add_terminal_player("Jarvis".to_owned());
    new_game.add_bot("Bot".to_owned());

    new_game.set_bank(bank::Bank::first_game());
    new_game.start_game();

    let player_names = new_game.get_player_names();
    println!("Players: ");
    println!("  {:#?}", player_names);

    println!("\nMarket:");
    if let Some(steps) = card::Card::market().get_steps() {
        new_game.run_steps(steps);
    }

    println!("\nCellar:");
    if let Some(steps) = card::Card::cellar().get_steps() {
        new_game.run_steps(steps);
    }

    println!("\nMoneylender:");
    if let Some(steps) = card::Card::moneylender().get_steps() {
        new_game.run_steps(steps);
    }

    println!("\nVassal:");
    if let Some(steps) = card::Card::vassal().get_steps() {
        new_game.run_steps(steps);
    }

    //println!("{:#?}", card::Card::copper());


}












