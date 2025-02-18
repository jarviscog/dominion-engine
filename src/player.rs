
//use colored::Colorize;
use crate::pile::Pile;
use crate::cost::Cost;
use super::*;

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

    pub fn sub_coins(&mut self, val: u32) -> Result<(), String> {
        if val > self.coins {
            return Err("Not enough coins".to_owned())
        } 
        self.coins -= val;
        Ok(())
    }

    pub fn get_debt(&self) -> u32 {
        self.debt
    }

    pub fn add_actions(&mut self, val: u32) {
        self.actions += val
    }

    pub fn sub_actions(&mut self, val: u32) -> Result<(), String> {
        if val > self.actions {
            return Err("Not enough actions".to_owned())
        } 
        self.actions -= val;
        Ok(())
    }

    pub fn add_buys(&mut self, val: u32) {
        self.buys += val
    }

    pub fn sub_buys(&mut self, val: u32) -> Result<(), String> {
        if val > self.buys {
            return Err("Not enough buys".to_owned())
        } 
        self.buys -= val;
        Ok(())
    }

    pub fn add_coins(&mut self, val: u32) {
        self.coins += val
    }

    pub fn add_debt(&mut self, val: u32) {
        self.debt += val
    }

    pub fn get_hand(&self) -> Vec<Card> {
        self.hand.to_card_vec()
    }

    pub fn print_state(&self) {
        println!("");
        println!("{:=^58}", self.get_name());
        println!("Deck: {:<8} Hand: {:<8} Discard: {:<8} In Play: {:<8}", 
            self.deck.size(), 
            self.hand.size(),
            self.discard.size(),
            self.in_play.size(),
        );
        println!("Actions: {:<8} Buys: {:<8} Coins: {:<8} Debt: {:<8}", 
            self.actions, 
            self.buys, 
            self.coins, 
            self.debt);
        println!("");
    }

}
