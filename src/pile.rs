use crate::card::Card;


/// A pile of cards. This could be a hand, deck, discard, or supply pile
#[derive(Debug)]
pub struct Pile {
    cards: Vec<Card>
}

// TODO Handle split piles
// TODO Hnadle pile rotation
impl Pile {

    pub fn new() -> Pile {
        Pile {
            cards: Vec::new()
        }
    }

    pub fn size(&self) -> usize {
        self.cards.len()
    }

    /// Creates a new pile from a card and a count of the given card
    pub fn from(count: u8, in_card: Card) -> Pile {
        let mut res_cards: Vec<Card> = Vec::new();
        for i in 0..count {
            res_cards.push(in_card.clone())
        }
        Pile {
            cards: res_cards
        }
    }

    pub fn starter_deck() -> Pile {
        // TODO make this cleaner
        Pile {
            cards: vec![
                Card::copper(),
                Card::copper(),
                Card::copper(),
                Card::copper(),
                Card::copper(),
                Card::copper(),
                Card::copper(),
                Card::estate(),
                Card::estate(),
                Card::estate(),
            ]
        }

    }


}
