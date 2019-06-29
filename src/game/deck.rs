use super::pile::*;

pub struct Deck {
    pub pile: Pile
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
