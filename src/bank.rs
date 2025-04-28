
use crate::pile::Pile;
use crate::card::Card;
use crate::player;

pub struct Bank {

    copper: Pile,
    silver: Pile,
    gold: Pile,

    estate: Pile,
    duchy: Pile,
    province: Pile,

    curses: Pile,

    trash: Pile,

    //https://wiki.dominionstrategy.com/index.php/Supply#Non-Supply
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
            curses: Pile::new(),
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

            curses: Pile::new(),
            trash: Pile::new(),
            supply_piles: Vec::new()
        }
    }

    /// Attempt to take a card from the supply. 
    /// If the card is removed, will return the Card
    /// If not, will return None
    pub fn take_card(&mut self, card: &Card) -> Option<Card> {

        // Check the basic supply piles
        if let Some(card) = match card.get_name().as_ref() {
            "Copper" => self.copper.pop_card(),
            "Silver" => self.silver.pop_card(),
            "Gold" => self.gold.pop_card(),

            "Estate" => self.estate.pop_card(),
            "Duchy" => self.duchy.pop_card(),
            "Province" => self.province.pop_card(),
            _ => None
        } { return Some(card) } 

        for pile in self.supply_piles.iter_mut() {
            if let Some(pile_name) = pile.top_card_name() {
                if pile_name == card.get_name() {
                    pile.pop_card();
                    return pile.pop_card()
                }
            }
        }
        None
        
    }

    /// Check to see if one of the top cards of a supply pile is a given card
    /// Will compare using the name of the card
    pub fn supply_contains_card(&self, in_card: Card) -> bool {
        for pile in &self.supply_piles {
            if let Some(card_name) = pile.top_card_name() {
                if card_name == in_card.get_name() {
                    return true;
                }
            }
        }
        return false;
    }

    /// Finish populating the bank based off of the number of players, and the type of game being
    /// played (add the key if playing Renaissance, etc.)
    pub fn finish_population(&mut self, player_count: usize) {
        // TODO check if there are any reasons that other info might need to get passed into this
        // function other than player count
        
        // https://wiki.dominionstrategy.com/index.php/Setup

        // Victory piles
        if player_count <= 2 {
            self.province = Pile::from(8, Card::province());
            self.duchy = Pile::from(8, Card::duchy());
            self.estate = Pile::from(8, Card::estate());
        }
        
        // Set up curses
        self.curses = Pile::from(((player_count-1) * 10) as u8, Card::curse());
        
        // Gardens
        if self.supply_contains_card(Card::gardens()) {

        }

        // TODO
        // Gardens should contain 12 cards, not 10
        // Number of Farms in the game
    }

    // https://wiki.dominionstrategy.com/index.php/Recommended_Kingdoms/Dominion#Dominion_alone
    pub fn first_game() -> Bank {
        let mut ret_bank = Bank::dominion();
        ret_bank.push_supply_pile(Card::cellar());
        ret_bank.push_supply_pile(Card::market());
        //ret_bank.push_supply_pile(Card::merchant());
        //ret_bank.push_supply_pile(Card::militia());
        ret_bank.push_supply_pile(Card::mine());
        ret_bank.push_supply_pile(Card::moat());
        ret_bank.push_supply_pile(Card::remodel());
        ret_bank.push_supply_pile(Card::smithy());
        ret_bank.push_supply_pile(Card::village());
        ret_bank.push_supply_pile(Card::workshop());
        ret_bank
    }

    pub fn size_distortion() -> Bank {
        let mut ret_bank = Bank::dominion();
        ret_bank.push_supply_pile(Card::artisan());
        //ret_bank.push_supply_pile(Card::bandit());
        ret_bank.push_supply_pile(Card::bureaucrat());
        ret_bank.push_supply_pile(Card::chapel());
        ret_bank.push_supply_pile(Card::festival());
        ret_bank.push_supply_pile(Card::gardens());
        ret_bank.push_supply_pile(Card::sentry());
        ret_bank.push_supply_pile(Card::throne_room());
        ret_bank.push_supply_pile(Card::witch());
        ret_bank.push_supply_pile(Card::workshop());
        ret_bank
    }

    pub fn deck_top() -> Bank {
        let mut ret_bank = Bank::dominion();
        ret_bank.push_supply_pile(Card::artisan());
        ret_bank.push_supply_pile(Card::bureaucrat());
        ret_bank.push_supply_pile(Card::council_room());
        ret_bank.push_supply_pile(Card::festival());
        ret_bank.push_supply_pile(Card::harbinger());
        ret_bank.push_supply_pile(Card::laboratory());
        ret_bank.push_supply_pile(Card::moneylender());
        ret_bank.push_supply_pile(Card::sentry());
        ret_bank.push_supply_pile(Card::vassal());
        ret_bank.push_supply_pile(Card::village());
        ret_bank
    }

    pub fn sleight_of_hand() -> Bank {
        let mut ret_bank = Bank::dominion();
        ret_bank.push_supply_pile(Card::cellar());
        ret_bank.push_supply_pile(Card::council_room());
        ret_bank.push_supply_pile(Card::festival());
        ret_bank.push_supply_pile(Card::gardens());
        ret_bank.push_supply_pile(Card::harbinger());
        //ret_bank.push_supply_pile(Card::library());
        //ret_bank.push_supply_pile(Card::militia());
        ret_bank.push_supply_pile(Card::poacher());
        ret_bank.push_supply_pile(Card::smithy());
        ret_bank.push_supply_pile(Card::throne_room());
        ret_bank
    }

    pub fn improvements() -> Bank {
        let mut ret_bank = Bank::dominion();
        ret_bank.push_supply_pile(Card::artisan());
        ret_bank.push_supply_pile(Card::cellar());
        ret_bank.push_supply_pile(Card::market());
        //ret_bank.push_supply_pile(Card::merchant());
        ret_bank.push_supply_pile(Card::mine());
        ret_bank.push_supply_pile(Card::moat());
        ret_bank.push_supply_pile(Card::moneylender());
        ret_bank.push_supply_pile(Card::poacher());
        ret_bank.push_supply_pile(Card::remodel());
        ret_bank.push_supply_pile(Card::witch());
        ret_bank
    }

    pub fn silver_and_gold() -> Bank {
        let mut ret_bank = Bank::dominion();
        ret_bank.push_supply_pile(Card::bandit());
        ret_bank.push_supply_pile(Card::bureaucrat());
        ret_bank.push_supply_pile(Card::chapel());
        ret_bank.push_supply_pile(Card::harbinger());
        ret_bank.push_supply_pile(Card::laboratory());
        //ret_bank.push_supply_pile(Card::merchant());
        ret_bank.push_supply_pile(Card::mine());
        ret_bank.push_supply_pile(Card::moneylender());
        ret_bank.push_supply_pile(Card::throne_room());
        ret_bank.push_supply_pile(Card::vassal());
        ret_bank
    }
}








