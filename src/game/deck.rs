use core::ops::Deref;
use core::ops::DerefMut;
use super::pile::*;

pub struct Deck {
    pile: Pile
}

impl Deck {
    pub fn new() -> Self {
        let mut p = Pile::new();

        for i in 1..14 {
            p.add_to_top(Card::new(i, Suit::Spade));
        }

        for i in 1..14 {
            p.add_to_top(Card::new(i, Suit::Heart));
        }

        for i in 1..14 {
            p.add_to_top(Card::new(i, Suit::Club));
        }

        for i in 1..14 {
            p.add_to_top(Card::new(i, Suit::Diamond));
        }

        Deck {
            pile: p
        }
    }
}

impl Deref for Deck {
    type Target = Pile;
    fn deref(&self) -> &Pile {
        &self.pile
    }
}

impl DerefMut for Deck {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pile
    }
}
