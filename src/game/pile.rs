extern crate rand;
pub mod card;

use crate::game::pile::rand::Rng;
pub use card::*;
use std::collections::VecDeque;

#[derive(Clone)]
pub enum PileType {
    Deposit,
    Flip,
    Deck,
}

#[derive(Clone)]
pub struct Pile {
    pile: VecDeque<Card>,
    pile_type: PileType,
}

impl Pile {
    pub fn new(p_type: PileType) -> Self {
        Pile {
            pile: VecDeque::new(),
            pile_type: p_type,
        }
    }

    pub fn index(&mut self, index: usize) -> &mut Card {
        &mut self.pile[index]
    }

    // pub fn get_internal_vec(&mut self) -> &mut VecDeque<Card> {
    //     &mut self.pile;
    // }

    fn is_card_flipped_up(&self, pos: usize) -> bool {
        self.pile[pos].is_flipped_up()
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        let mut rand_num;

        for i in 0..self.pile.len() {
            rand_num = rng.gen_range(i, self.pile.len());
            self.pile.swap(i, rand_num);
        }
    }

    pub fn add_to_top(&mut self, card: Card) {
        self.pile.push_front(card);
    }

    pub fn add_to_bottom(&mut self, card: Card) {
        self.pile.push_back(card);
    }

    pub fn remove_from_top(&mut self) -> Option<Card> {
        self.pile.pop_front()
    }

    pub fn remove_from_bottom(&mut self) -> Option<Card> {
        self.pile.pop_back()
    }

    pub fn get_card_string(&self, pos: usize) -> String {
        if pos < self.pile.len() {
            if self.is_card_flipped_up(pos) {
                return self.pile[pos].get_card_string();
            }

            return "#".to_string();
        }

        String::from("-")
    }

    pub fn get_pile_size(&self) -> usize {
        self.pile.len()
    }

    pub fn get_pile_type(&self) -> &PileType {
        &self.pile_type
    }

    pub fn print(&self) {
        for card in &self.pile {
            card.print();
        }
    }
}
