//use colored::Colorize;
use super::*;
use crate::cost::Cost;
use crate::pile::Pile;

#[derive(Debug)]
enum PlayerType {
    Bot,
    Terminal,
}

#[derive(Debug)]
pub struct Player {
    name: String,
    player_type: PlayerType,

    // Locations
    hand: Pile,
    deck: Pile,
    discard: Pile,
    in_play: Pile,

    internal_buffer: Pile,
    // Statevalues; Should be reset at end of turn
    actions: u32,
    buys: u32,
    debt: u32,
    coins: u32,
}

impl Player {
    pub fn bot(name: String) -> Player {
        Player {
            name,
            player_type: PlayerType::Bot,
            hand: Pile::new(),
            deck: Pile::starter_deck(),
            discard: Pile::new(),
            in_play: Pile::new(),
            internal_buffer: Pile::new(),
            actions: 1,
            buys: 1,
            coins: 0,
            debt: 0,
        }
    }

    pub fn terminal(name: String) -> Player {
        Player {
            name,
            player_type: PlayerType::Terminal,
            hand: Pile::new(),
            deck: Pile::starter_deck(),
            discard: Pile::new(),
            in_play: Pile::new(),
            internal_buffer: Pile::new(),
            actions: 1,
            buys: 1,
            coins: 0,
            debt: 0,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_actions(&self) -> u32 {
        self.actions
    }

    pub fn get_buys(&self) -> u32 {
        self.buys
    }

    pub fn get_coins(&self) -> u32 {
        self.coins
    }

    pub fn get_debt(&self) -> u32 {
        self.debt
    }

    pub fn add_actions(&mut self, val: i32) {
        if let Some(actions) = i32::try_from(self.actions).ok() {
            let result = actions + val;
            if result > 0 {
                self.actions = result as u32
            } else {
                println!("Cannot set negative buys")
            }
        }
    }

    pub fn add_buys(&mut self, val: i32) {
        if let Some(buys) = i32::try_from(self.buys).ok() {
            let result = buys + val;
            if result > 0 {
                self.buys = result as u32
            } else {
                println!("Cannot set negative buys")
            }
        }
    }

    pub fn add_coins(&mut self, val: i32) {
        if let Some(coins) = i32::try_from(self.coins).ok() {
            let result = coins + val;
            if result > 0 {
                self.coins = result as u32
            } else {
                println!("Cannot set negative coins")
            }
        }
    }

    pub fn add_debt(&mut self, val: i32) {
        if let Some(debt) = i32::try_from(self.debt).ok() {
            let result = debt + val;
            if result > 0 {
                self.debt = result as u32
            } else {
                println!("Cannot set negative debt")
            }
        }
    }

    pub fn draw_cards(&mut self, number_of_cards: u32) {
        println!("Drawing {} cards", number_of_cards);
        let mut number_of_cards_drawn = 0;
        while number_of_cards_drawn < number_of_cards {
            if let Some(card) = self.deck.pop_card() {
                self.hand.push_card(card);
            } else {
                // We need to shuffle the discards to draw another card
                self.shuffle_discard_into_deck();
                if let Some(card) = self.deck.pop_card() {
                    self.hand.push_card(card);
                } else {
                    println!("INFO: No more cards to shuffle. All cards must be in your hand");
                    return;
                }
            }
            number_of_cards_drawn += 1;
        }
    }

    pub fn remove_card(&mut self, location: Location, filters: Vec<CardFilter>) -> Option<Card> {
        // TODO Pop Card(s) from a location that meet some filters
        todo!()
    }

    pub fn shuffle_discard_into_deck(&mut self) {
        let mut cards = self.discard.pop_all_cards();
        // TODO actually do the shuffle
        for card in cards {
            self.deck.push_card(card);
        }
    }

    fn get_all_cards(&self) -> Vec<Card> {
        let mut all_cards: Vec<Card> = Vec::new();
        all_cards.append(&mut self.hand.get_all_cards());
        all_cards.append(&mut self.deck.get_all_cards());
        all_cards.append(&mut self.discard.get_all_cards());
        all_cards.append(&mut self.in_play.get_all_cards());
        all_cards
    }

    /// Get the total cards the player has, including deck, discard, hand
    /// Used to calculate VPs for Gardens
    pub fn total_cards(&self) -> u32 {
        self.hand.size().saturating_add(
            self.deck
                .size()
                .saturating_add(self.discard.size().saturating_add(self.in_play.size())),
        )
    }

    pub fn get_deck(&self) -> Vec<Card> {
        self.deck.to_card_vec()
    }
    pub fn get_hand(&self) -> Vec<Card> {
        self.hand.to_card_vec()
    }
    pub fn get_discard(&self) -> Vec<Card> {
        self.discard.to_card_vec()
    }
    pub fn get_in_play(&self) -> Vec<Card> {
        self.in_play.to_card_vec()
    }
    pub fn get_internal_buffer(&self) -> Vec<Card> {
        self.internal_buffer.to_card_vec()
    }

    pub fn print_state(&self) {
        println!("");
        println!("{:=^58}", self.get_name());
        println!(
            "Deck: {:<8} Hand: {:<8} Discard: {:<8} In Play: {:<8}",
            self.deck.size(),
            self.hand.size(),
            self.discard.size(),
            self.in_play.size(),
        );
        println!(
            "Actions: {:<8} Buys: {:<8} Coins: {:<8} Debt: {:<8}",
            self.actions, self.buys, self.coins, self.debt
        );
        print!("Cards in hand: ");
        for card in self.hand.clone().into_iter() {
            print!("{} ", card.get_name())
        }
        println!("");
        println!("");
    }
}
