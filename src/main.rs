#![allow(unused)]

use std::cell::{Ref, RefCell};
use std::rc::Rc;
use std::str::FromStr;

mod card;
use card::*;
mod game;
use game::{Choice, Decision};
mod bank;
mod cost;
mod expansion;
mod node;
mod pile;
mod player;
mod runtime_values;
mod terminal_player;
use card::Card;
use game::Game;

use terminal_player::*;

fn test_run_card_steps(in_game: &mut Game) {
    //    println!("\nLab:");
    //    if let Some(steps) = Card::laboratory().get_action_steps() {
    //        in_game.insert_into_current_node(steps);
    //    }

    //    println!("\nVillage:");
    //    if let Some(steps) = Card::village().get_action_steps() {
    //        in_game.insert_into_current_node(steps);
    //    }

    if let Some(steps) = card::Card::plus_action_buy_coin().get_action_steps() {
        in_game.insert_into_current_node(steps);
    }

    if let Some(steps) = card::Card::discard_a_card().get_action_steps() {
        in_game.insert_into_current_node(steps);
    }
    //
    //    println!("\nCellar:");
    //    if let Some(steps) = card::Card::cellar().get_action_steps() {
    //        in_game.insert_into_current_node(steps);
    //    }
    //
    //    println!("\nRemodel:");
    //    if let Some(steps) = card::Card::remodel().get_action_steps() {
    //        in_game.insert_into_current_node(steps);
    //    }
}

fn test_game_1(game: &mut Game) {
    let example_node = Node::new_full(
        NodeType::Root,
        0,
        vec![
            Rc::new(RefCell::new(Node::new_full(NodeType::Setup, 0, vec![]))),
            Rc::new(RefCell::new(Node::new_full(NodeType::Setup, 1, vec![]))),
            Rc::new(RefCell::new(Node::new_full(
                NodeType::Action,
                0,
                vec![
                    Rc::new(RefCell::new(Node::new_full(
                        NodeType::PlusCoin(RuntimeI32::Const(1)),
                        0,
                        Vec::new(),
                    ))),
                    Rc::new(RefCell::new(Node::new_full(
                        NodeType::PlusAction(RuntimeI32::Const(2)),
                        0,
                        Vec::new(),
                    ))),
                    Rc::new(RefCell::new(Node::new_full(
                        NodeType::DrawCard(RuntimeI32::Const(2)),
                        0,
                        Vec::new(),
                    ))),
                ],
            ))),
            Rc::new(RefCell::new(Node::new_full(
                NodeType::Buy,
                0,
                vec![Rc::new(RefCell::new(Node::new_full(
                    NodeType::PlusCoin(RuntimeI32::Const(1)),
                    0,
                    Vec::new(),
                )))],
            ))),
            Rc::new(RefCell::new(Node::new_full(
                NodeType::Action,
                1,
                vec![Rc::new(RefCell::new(Node::new_full(
                    NodeType::PlusCoin(RuntimeI32::Const(1)),
                    1,
                    Vec::new(),
                )))],
            ))),
        ],
    );
    game.set_game_state(example_node);
}

fn main() {
    let mut new_game = Game::new();
    new_game.add_terminal_player("Jarvis".to_owned());
    new_game.add_bot("Bot".to_owned());
    new_game.set_bank(bank::Bank::first_game());
    new_game.start_game();

    //test_run_card_steps(&mut new_game);
    if let Some(steps) = card::Card::laboratory().get_action_steps() {
        new_game.insert_into_current_node(steps);
    }

    new_game.print_player_stats();
    new_game.print_event_listeners();
    new_game.print_game_state();

    while new_game.get_next_unvisited_node().is_some() {
        new_game.run_next_node();
        new_game.print_player_stats();
        new_game.print_game_state();
    }
    new_game.run_next_node();
    new_game.print_player_stats();
    new_game.print_game_state();
}

fn game_loop(in_game: &mut Game) {
    loop {
        in_game.print_player_stats();
        in_game.print_event_listeners();
        in_game.print_game_state();

        println!("Playing turn: {}", in_game.get_current_player_name());
        for card in in_game.get_current_player_hand() {
            println!("{:?}", card);
        }

        let d = match in_game.get_current_choice() {
            Choice::ActionPhase => prompt_for_action_phase_decision(),
            Choice::BuyPhase => prompt_for_buy_phase_decision(),
            _ => todo!(),
        };
        match in_game.make_decision(d) {
            Ok(v) => {}
            Err(e) => {
                println!("{}", e)
            }
        }
    }
}
