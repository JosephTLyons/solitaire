mod pile;

use pile::*;
use std::io;

enum Actions {
    CycleCards,
    MoveCard,
    NewGame,
    Quit,
}

enum From {
    TempDeck,
    DepositPile1,
    DepositPile2,
    DepositPile3,
    DepositPile4,
    FlipPile1,
    FlipPile2,
    FlipPile3,
    FlipPile4,
    FlipPile5,
    FlipPile6,
    FlipPile7,
}

enum To {
    TempDeck,
    DepositPile1,
    DepositPile2,
    DepositPile3,
    DepositPile4,
    FlipPile1,
    FlipPile2,
    FlipPile3,
    FlipPile4,
    FlipPile5,
    FlipPile6,
    FlipPile7,
}

pub struct Game {
    deck: Pile,
    temp_deck: Pile,
    deposit_piles: Vec<Pile>,
    flip_piles: Vec<Pile>,
}

impl Game {
    pub fn new() -> Self {
        // Set up deck
        let mut dec: Pile = Pile::new();
        dec.make_pile_the_main_deck();
        dec.shuffle();

        // Set up deposit piles
        let mut deposit_p: Vec<Pile> = Vec::new();

        for _ in 0..4 {
            deposit_p.push(Pile::new());
        }

        // Set up flip piles
        let mut flip_p: Vec<Pile> = Vec::new();

        for _ in 0..7 {
            flip_p.push(Pile::new());
        }

        for i in 0..flip_p.len() {
            for j in i..flip_p.len() {
                // In this case, I know that there will always be enough cards to remove from the
                // dec, so it can be unwrapped
                flip_p[j].add_to_top(dec.remove_from_top().unwrap());
            }
        }

        // Flip top cards over in deposit piles
        for pile in &mut flip_p {
            pile.index(0).flip_card_up();
        }

        // Add 3 cards to temp deck and flip over
        let mut temp: Pile = Pile::new();
        temp.add_to_top(dec.remove_from_top().unwrap());
        temp.add_to_top(dec.remove_from_top().unwrap());
        temp.add_to_top(dec.remove_from_top().unwrap());

        for i in 0..temp.get_pile_size() {
            temp.index(i).flip_card_up()
        }

        // Wish I could use the vec! macro here instead of pushing onto the Vecs above
        Game {
            deck: dec,
            temp_deck: temp,
            deposit_piles: deposit_p,
            flip_piles: flip_p,
        }
    }

    pub fn take_turn(&mut self) {
        let mut should_continue = true;

        while should_continue {
            self.print_action_menu();
            let input = self.get_integer_input();

            match input {
                Some(n) => {
                    match n {
                        1 => self.cycle_cards(),
                        2 => self.move_card(),
                        3 => {}
                        4 => {}
                        _ => {} // This will never occur
                    }

                    should_continue = false;
                }

                None => {
                    println!("Input needs to be a single integer");
                    should_continue = true;
                }
            }
        }
    }

    fn print_action_menu(&self) {
        println!("Action Menu");
        println!("===========");
        println!("1: Cycle Cards");
        println!("2: Move Card");
        println!("3: New Game");
        println!("4: Quit");
    }

    // Returns 0 if input is bad
    fn get_integer_input(&self) -> Option<u8> {
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                input.remove(input.len() - 1);
                match input.parse::<u8>() {
                    Ok(val) => Some(val),
                    Err(_) => None,
                }
            }
            Err(_) => None,
        }
    }

    fn cycle_cards(&mut self) {
        for _ in 0..self.temp_deck.get_pile_size() {
            let mut card = self.temp_deck.remove_from_bottom().unwrap();
            card.flip_card_down();
            self.deck.add_to_bottom(card);
            card = self.deck.remove_from_top().unwrap();
            card.flip_card_up();
            self.temp_deck.add_to_top(card);
        }
    }

    fn move_card(&self) {}

    fn move_to_flip_pile_is_ok(&self, card_on_top: &Card, card_on_bottom: &Card) -> bool {
        if card_on_top.get_value() == card_on_bottom.get_value() - 1 {
            if card_on_top.get_color() != card_on_bottom.get_color() {
                return true;
            }
        }

        false
    }

    fn move_to_deposit_pil_is_ok(&self, card_on_top: &Card, card_on_bottom: &Card) -> bool {
        if card_on_top.get_value() == card_on_bottom.get_value() + 1 {
            if card_on_top.get_color() != card_on_bottom.get_color() {
                return true;
            }
        }

        false
    }

    pub fn print(&self) {
        print!("{}", String::from("\n").repeat(5));

        println!(
            "{:2}: {:3} {:3} {:3}               {:3} {:3} {:3} {:3}",
            self.deck.get_pile_size(),
            self.temp_deck.get_card_string(0),
            self.temp_deck.get_card_string(1),
            self.temp_deck.get_card_string(2),
            self.deposit_piles[0].get_card_string(0),
            self.deposit_piles[1].get_card_string(0),
            self.deposit_piles[2].get_card_string(0),
            self.deposit_piles[3].get_card_string(0),
        );

        let mut flip_pile_print_count = 1;

        for i in 0..self.flip_piles.len() {
            if flip_pile_print_count < self.flip_piles[i].get_pile_size() {
                flip_pile_print_count = self.flip_piles[i].get_pile_size();
            }
        }

        // Print pointer to useable card from deck
        println!("            ^");

        for i in 0..flip_pile_print_count {
            println!(
                "                  {:3} {:3} {:3} {:3} {:3} {:3} {:3}",
                self.flip_piles[0].get_card_string(i),
                self.flip_piles[1].get_card_string(i),
                self.flip_piles[2].get_card_string(i),
                self.flip_piles[3].get_card_string(i),
                self.flip_piles[4].get_card_string(i),
                self.flip_piles[5].get_card_string(i),
                self.flip_piles[6].get_card_string(i),
            );
        }

        print!("{}", String::from("\n").repeat(2));
    }
}
