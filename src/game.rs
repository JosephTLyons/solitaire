mod deck;
mod pile;

use deck::*;
use pile::*;
use std::io;

enum Actions {
    NextThreeCards,
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
    deck: Deck,
    temp_deck: Pile,
    deposit_piles: Vec<Pile>,
    flip_piles: Vec<Pile>,
}

impl Game {
    pub fn new() -> Self {
        // Set up deck
        let mut dec: Deck = Deck::new();
        dec.shuffle();

        // Set up flip piles
        let mut flip_p: Vec<Pile> = vec![Pile::new(); 7];

        for i in 0..flip_p.len() {
            for pile in flip_p.iter_mut().skip(i) {
                // In this case, I know that there will always be enough cards to remove from the
                // dec, so it can be unwrapped
                pile.add_to_top(dec.remove_from_top().unwrap());
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

        Game {
            deck: dec,
            temp_deck: temp,
            deposit_piles: vec![Pile::new(); 4],
            flip_piles: flip_p,
        }
    }

    pub fn take_turn(&mut self) {
        self.print_action_menu();

        loop {
            let input = self.get_integer_input();

            match input {
                1 => self.next_three_cards(),
                2 => self.move_card(),
                3 => {}
                4 => {}
                _ => println!("Input needs to be a single integer 1 - 4"),
            }

            if 0 < input && input <= 4 {
                break;
            }
        }
    }

    fn print_action_menu(&self) {
        println!("Action Menu");
        println!("===========");
        println!("1: Next Three Cards");
        println!("2: Move Card");
        println!("3: New Game");
        println!("4: Quit");
    }

    // Returns 0 if input is bad
    fn get_integer_input(&self) -> u8 {
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                input.remove(input.len() - 1);
                match input.parse::<u8>() {
                    Ok(val) => val,
                    Err(_) => 0,
                }
            }
            Err(_) => 0,
        }
    }

    fn next_three_cards(&mut self) {
        let mut card;

        // Put temp deck back
        for _ in 0..self.temp_deck.get_pile_size() {
            card = self.temp_deck.remove_from_bottom().unwrap();
            card.flip_card_down();
            self.deck.add_to_bottom(card);
        }

        // Put 3 more cards into temp deck
        for _ in 0..3 {
            card = self.deck.remove_from_top().unwrap();
            card.flip_card_up();
            self.temp_deck.add_to_top(card);
        }
    }

    fn move_card(&self) {
        self.print_move_card_menu();

        let mut from: Option<&Pile>;

        loop {
            from = self.get_pile_reference();

            if from.is_some() {
                break;
            } else {
                println!("Input needs to be a single integer 1 - 12")
            }
        }

        let mut to: Option<&Pile>;

        loop {
            to = self.get_pile_reference();

            if to.is_some() {
                break;
            } else {
                println!("Input needs to be a single integer 1 - 12")
            }
        }

        if self.move_is_valid(from.unwrap(), to.unwrap()) {
            // move_card_from_to(from, to);
        }
    }

    fn get_pile_reference(&self) -> Option<&Pile> {
        match self.get_integer_input() {
            1 => Some(&self.deck),
            2 => Some(&self.flip_piles[0]),
            3 => Some(&self.flip_piles[1]),
            4 => Some(&self.flip_piles[2]),
            5 => Some(&self.flip_piles[3]),
            6 => Some(&self.flip_piles[4]),
            7 => Some(&self.flip_piles[5]),
            8 => Some(&self.flip_piles[6]),
            9 => Some(&self.deposit_piles[0]),
            10 => Some(&self.deposit_piles[1]),
            11 => Some(&self.deposit_piles[2]),
            12 => Some(&self.deposit_piles[3]),
            _ => None,
        }
    }

    fn print_move_card_menu(&self) {
        println!("Move card from:");
        println!("===============");
        println!("1: Deck");
        println!("2: Flip Pile 1");
        println!("3: Flip Pile 2");
        println!("4: Flip Pile 3");
        println!("5: Flip Pile 4");
        println!("6: Flip Pile 5");
        println!("7: Flip Pile 6");
        println!("8: Flip Pile 7");
        println!("9: Deposit Pile 1");
        println!("10: Deposit Pile 2");
        println!("11: Deposit Pile 3");
        println!("12: Deposit Pile 4");
    }

    fn move_is_valid(&self, from_pile: &Pile, to_pile: &Pile) -> bool {
        // if from_pile == to_pile {
        //     return true;
        // }
        //
        // // Case of moving to deposit pile, (same suit, ascending)
        // else if true {
        //
        // }
        //
        // // Case of moving to flip pile, (alternating colors, descending)
        // else if card_on_top.get_value() == card_on_bottom.get_value() - 1 {
        //     if card_on_top.get_color() != card_on_bottom.get_color() {
        //         return true;
        //     }
        // }
        //
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

        for i in (0..flip_pile_print_count).rev() {
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
