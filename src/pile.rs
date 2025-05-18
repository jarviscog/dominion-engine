use crate::card::Card;

/// A pile of cards. This could be a hand, deck, discard, or supply pile
#[derive(Debug, Clone)]
pub struct Pile {
    cards: Vec<Card>,
}

// TODO Handle split piles
// TODO Hnadle pile rotation
impl Pile {
    pub fn new() -> Pile {
        Pile { cards: Vec::new() }
    }

    pub fn size(&self) -> u32 {
        if self.cards.len() > u32::MAX as usize {
            u32::MAX
        } else {
            self.cards.len() as u32
        }
    }

    /// Creates a new pile from a card and a count of the given card
    pub fn from(count: u8, in_card: Card) -> Pile {
        let mut res_cards: Vec<Card> = Vec::new();
        for i in 0..count {
            res_cards.push(in_card.clone())
        }
        Pile { cards: res_cards }
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
            ],
        }
    }

    /// Pops a card from the top of the pile
    pub fn pop_card(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn pop_all_cards(&mut self) -> Vec<Card> {
        // TODO This could be a performance issue
        let all_cards = self.cards.clone();
        self.cards = Vec::new();
        all_cards
    }

    pub fn push_card(&mut self, in_card: Card) {
        self.cards.push(in_card)
    }

    pub fn top_card_name(&self) -> Option<String> {
        if let Some(c) = self.cards.last() {
            return Some(c.get_name());
        } else {
            None
        }
    }

    pub fn get_all_cards(&self) -> Vec<Card> {
        self.cards.clone()
    }

    /// Returns a vec of cards, ignoring rotation and items on top
    pub fn to_card_vec(&self) -> Vec<Card> {
        return self.cards.clone();
    }

    pub fn print(&self) {
        for card in &self.cards {
            println!("{:?}", card);
        }
    }
}

// This struct will hold the state of the iteration
pub struct PileIter {
    inner: std::vec::IntoIter<Card>,
}

impl IntoIterator for Pile {
    type Item = Card;
    type IntoIter = PileIter;

    fn into_iter(self) -> Self::IntoIter {
        PileIter {
            inner: self.cards.into_iter(),
        }
    }
}

// Implement `Iterator` for your custom iterator
impl Iterator for PileIter {
    type Item = Card;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
