use crate::card;
use super::Bank;

pub fn first_game() -> Self {
    let bank = Bank::base_game();
    bank.action.insert(card::base_game::cellar(), 10);
    bank.action.insert(card::base_game::market(), 10);
    bank.action.insert(card::base_game::merchant(), 10);
    bank.action.insert(card::base_game::militia(), 10);
    bank.action.insert(card::base_game::mine(), 10);
    bank.action.insert(card::base_game::moat(), 10);
    bank.action.insert(card::base_game::remodel(), 10);
    bank.action.insert(card::base_game::smithy(), 10);
    bank.action.insert(card::base_game::village(), 10);
    bank.action.insert(card::base_game::workshop(), 10);
    bank
}

pub fn size_distortion() -> Self {
    let bank = Bank::base_game();
    bank.action.insert(card::base_game::artisan(), 10);
    bank.action.insert(card::base_game::bandit(), 10);
    bank.action.insert(card::base_game::bureaucrat(), 10);
    bank.action.insert(card::base_game::chapel(), 10);
    bank.action.insert(card::base_game::festival(), 10);
    bank.action.insert(card::base_game::gardens(), 10);
    bank.action.insert(card::base_game::sentry(), 10);
    bank.action.insert(card::base_game::throne_room(), 10);
    bank.action.insert(card::base_game::witch(), 10);
    bank.action.insert(card::base_game::workshop(), 10);
    bank
}

pub fn deck_top() -> Self {
    let bank = Bank::base_game();
    bank.action.insert(card::base_game::artisan(), 10);
    bank.action.insert(card::base_game::bureaucrat(), 10);
    bank.action.insert(card::base_game::council_room(), 10);
    bank.action.insert(card::base_game::festival(), 10);
    bank.action.insert(card::base_game::harbinger(), 10);
    bank.action.insert(card::base_game::laboratory(), 10);
    bank.action.insert(card::base_game::moneylender(), 10);
    bank.action.insert(card::base_game::sentry(), 10);
    bank.action.insert(card::base_game::vassal(), 10);
    bank.action.insert(card::base_game::village(), 10);
    bank
}

pub fn slight_of_hand() -> Self {
    let bank = Bank::base_game();
    bank.action.insert(card::base_game::cellar(), 10);
    bank.action.insert(card::base_game::council_room(), 10);
    bank.action.insert(card::base_game::festival(), 10);
    bank.action.insert(card::base_game::gardens(), 10);
    bank.action.insert(card::base_game::library(), 10);
    bank.action.insert(card::base_game::harbinger(), 10);
    bank.action.insert(card::base_game::militia(), 10);
    bank.action.insert(card::base_game::poacher(), 10);
    bank.action.insert(card::base_game::smithy(), 10);
    bank.action.insert(card::base_game::throne_room(), 10);
    bank
}

pub fn improvements() -> Self {
    let bank = Bank::base_game();
    bank.action.insert(card::base_game::artisan(), 10);
    bank.action.insert(card::base_game::cellar(), 10);
    bank.action.insert(card::base_game::market(), 10);
    bank.action.insert(card::base_game::merchant(), 10);
    bank.action.insert(card::base_game::mine(), 10);
    bank.action.insert(card::base_game::moat(), 10);
    bank.action.insert(card::base_game::moneylender(), 10);
    bank.action.insert(card::base_game::poacher(), 10);
    bank.action.insert(card::base_game::remodel(), 10);
    bank.action.insert(card::base_game::witch(), 10);
    bank
}

pub fn silver_and_gold() -> Self {
    let bank = Bank::base_game();
    bank.action.insert(card::base_game::bandit(), 10);
    bank.action.insert(card::base_game::bureaucrat(), 10);
    bank.action.insert(card::base_game::chapel(), 10);
    bank.action.insert(card::base_game::harbinger(), 10);
    bank.action.insert(card::base_game::laboratory(), 10);
    bank.action.insert(card::base_game::merchant(), 10);
    bank.action.insert(card::base_game::mine(), 10);
    bank.action.insert(card::base_game::moneylender(), 10);
    bank.action.insert(card::base_game::throne_room(), 10);
    bank.action.insert(card::base_game::vassal(), 10);
    bank
}


