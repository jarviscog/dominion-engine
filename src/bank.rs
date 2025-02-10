
use crate::pile::Pile;
use crate::card::Card;

pub struct Bank {

    copper: Pile,
    silver: Pile,
    gold: Pile,

    estate: Pile,
    duchy: Pile,
    province: Pile,

    trash: Pile,

    supply_piles: Vec<Pile>
}

impl Bank {
    
    pub fn new() -> Bank {
        Bank {
            copper: Pile::new(),
            silver: Pile::new(),
            gold: Pile::new(),
            estate: Pile::new(),
            duchy: Pile::new(),
            province: Pile::new(),
            trash: Pile::new(),
            supply_piles: Vec::new(),
        }
    }

    // Pushes a supply pile. No more than 10 supply piles can exist at a time
    pub fn push_supply_pile(&mut self, card: Card) {
        if self.supply_piles.len() <= 10 {
            self.supply_piles.push(Pile::from(10, card))
        } else {
            println!("WARNING: All of the supply piles have already been filled")
        }
    }

    /// Sets up the bank for a basic game of dominion
    /// Does not set the supply piles. Set this with either the name of the recommended kingdom, or
    /// use `set_supply_piles()`
    pub fn dominion() -> Bank {
        // TODO set_supply_piles()
        Bank {
            copper: Pile::from(60, Card::copper()),
            silver: Pile::from(40, Card::silver()),
            gold: Pile::from(30, Card::gold()),

            estate: Pile::from(24, Card::estate()),
            duchy: Pile::from(12, Card::duchy()),
            province: Pile::from(12, Card::province()),

            trash: Pile::new(),
            supply_piles: Vec::new()
        }
    }

    pub fn first_game() -> Bank {
        let mut ret_bank = Bank::dominion();
         //ret_bank.push_supply_pile(Card::cellar());
         ret_bank.push_supply_pile(Card::market());
         //ret_bank.push_supply_pile(Card::merchant());
         //ret_bank.push_supply_pile(Card::militia());
         //ret_bank.push_supply_pile(Card::mine());
         //ret_bank.push_supply_pile(Card::moat());
         //ret_bank.push_supply_pile(Card::remodel());
         //ret_bank.push_supply_pile(Card::smithy());
         ret_bank.push_supply_pile(Card::village());
         //ret_bank.push_supply_pile(Card::workshop());
        ret_bank
    }

    pub fn size_distortion() -> Bank {
        let mut ret_bank = Bank::dominion();
         //ret_bank.push_supply_pile(Card::artisan());
         //ret_bank.push_supply_pile(Card::bandit());
         //ret_bank.push_supply_pile(Card::bureaucrat());
         //ret_bank.push_supply_pile(Card::chapel());
         ret_bank.push_supply_pile(Card::festival());
         //ret_bank.push_supply_pile(Card::gardens());
         //ret_bank.push_supply_pile(Card::sentry());
         //ret_bank.push_supply_pile(Card::throne_room());
         //ret_bank.push_supply_pile(Card::witch());
         //ret_bank.push_supply_pile(Card::workshop());
        ret_bank
    }
}








