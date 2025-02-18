#![allow(unused)]

use std::str::FromStr;

mod game;
mod bank;
mod player;
mod expansion;
mod card;
mod card_type;
mod step;
use card::Card;
use step::*;
mod cost;
mod pile;
mod triggered_event;

mod terminal_player;
use terminal_player::*;


use game::{Choice, Decision};

fn test_run_card_steps(in_game: &mut game::Game) {
    println!("\nMarket:");
    if let Some(steps) = card::Card::market().get_action_steps() {
        in_game.run_steps(steps);
    }

    println!("\nCellar:");
    if let Some(steps) = card::Card::cellar().get_action_steps() {
        in_game.run_steps(steps);
    }

    println!("\nRemodel:");
    if let Some(steps) = card::Card::remodel().get_action_steps() {
        in_game.run_steps(steps);
    }

    println!("\nMoneylender:");
    if let Some(steps) = card::Card::moneylender().get_action_steps() {
        in_game.run_steps(steps);
    }

    println!("\nHarbinger:");
    if let Some(steps) = card::Card::harbinger().get_action_steps() {
        in_game.run_steps(steps);
    }

    println!("\nSentry:");
    if let Some(steps) = card::Card::sentry().get_action_steps() {
        in_game.run_steps(steps);
    }

    println!("\nBandit:");
    if let Some(steps) = card::Card::bandit().get_action_steps() {
        in_game.run_steps(steps);
    }


}


fn main() {

    let mut new_game = game::Game::new();

    new_game.add_terminal_player("Jarvis".to_owned());
    new_game.add_bot("Bot".to_owned());

    new_game.set_bank(bank::Bank::first_game());
    new_game.start_game();

    //test_run_card_steps(&mut new_game);
    loop {
        new_game.print_player_stats();
        new_game.print_event_listeners();
        new_game.print_event_history();
        
        println!("Playing turn: {}", new_game.get_current_player_name());
        for card in new_game.get_current_player_hand() {
            println!("{:?}", card);
        }

        let d = match new_game.get_current_choice() {
            Choice::ActionPhase => {
                prompt_for_action_phase_decision()
            }
            Choice::BuyPhase => {
                prompt_for_buy_phase_decision()
            }
            _ => todo!()

        };
        match new_game.make_decision(d) {
            Ok(v) => {},
            Err(e) => {println!("{}", e)}
        }

    }


}








