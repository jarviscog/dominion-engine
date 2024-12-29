#![allow(unused)]

use event::Event;
use event_condition::EventCondition;


mod expansion {
    #[derive(Debug)]
    pub enum Expansion {
        BaseGame,
        Intrigue,
        Seaside
    }
}

mod event {
    #[derive(Debug, Clone)]
    pub enum Event {
        PlusCoin(u8),
        PlusAction(u8),
        PlusBuy(u8),
    }
}

mod event_condition {
    use crate::event;
    #[derive(Debug, Clone)]
    pub enum EventCondition {
        Event(event::Event),
        And(Box<EventCondition>, Box<EventCondition>),
        Or(Box<EventCondition>, Box<EventCondition>),
    }
}

mod card_type {
    pub enum CardType {
        Action,
        Treasure,
        Victory,
        Curse,
        Attack,
        Reaction,
    }
}

mod card {
    use crate::{event_condition::EventCondition, expansion::Expansion, card_type::CardType};

    pub struct Card {
        name: String,
        expansion: Expansion,
        events: Vec<EventCondition>,
        card_type: Vec<CardType>,
    }
}


fn main() {

    println!("Hello world");
    println!("{:?}", event::Event::PlusBuy(1));
    println!("{:?}", expansion::Expansion::Seaside);

    let plus_coin = EventCondition::Event(Event::PlusCoin(1));
    let plus_action = EventCondition::Event(Event::PlusAction(1));
    println!("{:?}", EventCondition::Or(Box::new(plus_coin), Box::new(plus_action)));


}












