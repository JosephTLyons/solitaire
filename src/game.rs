mod deck;
mod pile;

use deck::*;
use pile::*;
use std::io;
use std::io::stdout;
use std::io::Write;

enum Actions {
    NextThreeCards,
    MoveCard,
    NewGame,
    Quit,
}

enum ChosenDeck {
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
    visible_deck_cards: Pile,
    deposit_piles: Vec<Pile>,
    flip_piles: Vec<Pile>,
    temp_deck: Pile,
    should_quit: bool,
    won: bool,
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
            visible_deck_cards: temp,
            deposit_piles: vec![Pile::new(); 4],
            flip_piles: flip_p,
            temp_deck: Pile::new(),
            should_quit: false,
            won: false,
        }
    }

    pub fn play(&mut self) {
        while !(self.should_quit || self.won) {
            self.print();
            self.take_turn()
        }

        if self.won {
            println!("You've won!");
        }
    }

    fn take_turn(&mut self) {
        self.print_action_menu();
        println!();
        print!("Choice: ");
        stdout().flush().unwrap();

        loop {
            let input = self.get_integer_input();

            match input {
                1 => self.next_three_cards(),
                2 => self.move_card(),
                3 => {}
                4 => self.should_quit = true,
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
        for _ in 0..self.visible_deck_cards.get_pile_size() {
            card = self.visible_deck_cards.remove_from_bottom().unwrap();
            card.flip_card_down();
            self.deck.add_to_bottom(card);
        }

        // Put 3 more cards into temp deck
        for _ in 0..3 {
            card = self.deck.remove_from_top().unwrap();
            card.flip_card_up();
            self.visible_deck_cards.add_to_top(card);
        }
    }

    fn move_card(&mut self) {
        self.print_move_card_menu();

        let mut from: Option<&Pile>;

        loop {
            print!("From: ");
            stdout().flush().unwrap();

            from = self.get_pile_reference();

            if from.is_some() {
                break;
            } else {
                println!("Input needs to be a single integer 1 - 12")
            }
        }

        let mut to: Option<&Pile>;

        loop {
            print!("To: ");
            stdout().flush().unwrap();

            to = self.get_pile_reference();

            if to.is_some() {
                break;
            } else {
                println!("Input needs to be a single integer 1 - 12")
            }
        }

        // Options:
        //     Deck to Flip    (Deck to temp to Flip)
        //     Flip to Flip    (Flip to temp to Flip)
        //     Deposit to Flip (Deposit to temp to Flip)
        //     Flip to Deposit (Flip to temp to Deposit)

        if self.move_to_flip_pile_ok(
            &from.unwrap().clone().index(0),
            &to.unwrap().clone().index(0),
        ) {
            to.unwrap().to_owned().add_to_top(from.unwrap().to_owned().remove_from_top().unwrap());
        }
    }

    fn get_pile_reference(&self) -> Option<&Pile> {
        match self.get_integer_input() {
            1 => Some(&self.visible_deck_cards),
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
        println!("Move card:");
        println!("=========");
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

    fn move_to_deposit_pile_ok(&self, from: &Card, to: &Card) -> bool {
        // Case of moving to deposit pile, (same suit, ascending)
        if (from.get_value() == to.get_value() + 1) && (from.get_suit() == to.get_suit()) {
            return true;
        }

        false
    }

    fn move_to_flip_pile_ok(&self, from: &Card, to: &Card) -> bool {
        // Case of moving to flip pile, (alternating colors, descending)
        if (from.get_value() == to.get_value() - 1) && (from.get_color() != to.get_color()) {
            return true;
        }

        false
    }

    fn print(&self) {
        print!("{}", String::from("\n").repeat(5));

        println!(
            "{:2}: {:3} {:3} {:3}               {:3} {:3} {:3} {:3}",
            self.deck.get_pile_size(),
            self.visible_deck_cards.get_card_string(2),
            self.visible_deck_cards.get_card_string(1),
            self.visible_deck_cards.get_card_string(0),
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

        // Print dynamic pointer to next card in temp pile that we can access
        println!(
            "{}{}",
            String::from("    ").repeat(self.visible_deck_cards.get_pile_size()),
            String::from("^"),
        );

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

        println!();
    }
}
