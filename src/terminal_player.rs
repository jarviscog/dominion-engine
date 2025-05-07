use crate::*;
use std::io::stdin;

/// Prompt the user to choose a card
pub fn prompt_for_card(filters: Option<CardFilter>) -> Card {
    // TODO Add an in_list param to choose from a list of cards
    // TODO Apply the filters to the card to choose from
    let mut input_string = String::new();
    println!("What card would you like to play?");
    stdin().read_line(&mut input_string)
        .ok()
        .expect("Failed to read line");
    match input_string.trim().trim_end_matches(&['\r', '\n'][..]).as_ref() {
        "Copper" => Card::copper(),
        "Silver" => Card::silver(),
        "Gold" => Card::gold(),
        "Artisan" => Card::artisan(),
        "Bureaucrat" => Card::bureaucrat(),
        "Bandit" => Card::bandit(),
        "Cellar" => Card::cellar(),
        "Chapel" => Card::chapel(),
        "Council Room" => Card::council_room(),
        "Festival" => Card::festival(),
        "Harbinger" => Card::harbinger(),
        "Smithy" => Card::smithy(),
        "Laboratory" => Card::laboratory(),
        "Militia" => Card::militia(),
        "Market" => Card::market(),
        "Mine" => Card::mine(),
        "Moat" => Card::moat(),
        "Village" => Card::village(),
        &_ => todo!()
    }
}

pub fn prompt_for_action_phase_decision() -> Decision {
    // TODO Add an in_list param to choose from a list of cards
    // TODO Apply the filters to the card to choose from
    let mut input_string = String::new();
    loop {
        println!("Action phase: [p]: Play a card    [n]: Go to next phase");
        stdin().read_line(&mut input_string)
            .ok()
            .expect("Failed to read line");
        match input_string.trim().trim_end_matches(&['\r', '\n'][..]).as_ref() {
            "p" => return Decision::PlayCard(prompt_for_card(None)),
            "n" => return Decision::StartNextPhase,
            _ => {}
        }
        input_string.clear();
    }
}

pub fn prompt_for_buy_phase_decision() -> Decision {
    let mut input_string = String::new();
    loop {
        println!("Buy phase: [p]: Play a card [b]: Buy a Card   [n]: Next Turn");
        stdin().read_line(&mut input_string)
            .ok()
            .expect("Failed to read line");
        match input_string.trim().trim_end_matches(&['\r', '\n'][..]).as_ref() {
            "p" => return Decision::PlayCard(prompt_for_card(None)),
            "b" => return Decision::BuyCard(prompt_for_card(None)),
            "n" => return Decision::StartNextPhase,
            _ => {}
        }
        input_string.clear();
    }
}

pub fn make_decision(c: Choice) -> Decision {
    match c {
        Choice::ActionPhase => prompt_for_action_phase_decision(),
        Choice::BuyPhase => prompt_for_buy_phase_decision(),
        Choice::ChooseCards(x, y, z) => {
            panic!("ERROR: Choose cards not implemented")
        }
        Choice::NameACard(card_filters) => {
            Decision::NameACard(prompt_for_card(card_filters))
        }

    }

}

