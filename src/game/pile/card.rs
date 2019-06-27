#[derive(Clone)]
pub enum Suit {
    Spade,
    Heart,
    Club,
    Diamond,
}

#[derive(PartialEq)]
#[derive(Clone)]
pub enum Color {
    Red,
    Black
}

#[derive(Clone)]
pub struct Card {
    value: u8,
    suit: Suit,
    color: Color,
    face_up: bool,
}

impl Card {
    pub fn new(val: u8, s: Suit) -> Self {
        let col = match s {
            Suit::Spade | Suit::Club => Color::Black,
            Suit::Heart | Suit::Diamond => Color::Red,
        };

        Card {
            value: val,
            suit: s,
            color: col,
            face_up: false
        }
    }

    pub fn flip_card_up(&mut self) {
        self.face_up = true;
    }

    pub fn flip_card_down(&mut self) {
        self.face_up = false;
    }

    pub fn is_flipped_up(&self) -> bool {
        self.face_up
    }

    pub fn get_value(&self) -> u8 {
        self.value
    }

    pub fn get_suit(&self) -> Suit {
        self.suit.clone()
    }

    pub fn get_color(&self) -> Color {
        self.color.clone()
    }

    pub fn get_card_string(&self) -> String {
        let suit = match &self.suit {
            Suit::Spade => 'S',
            Suit::Heart => 'H',
            Suit::Club => 'C',
            Suit::Diamond => 'D',
        };

        let value = match &self.value {
            1 => "A",
            2 => "2",
            3 => "3",
            4 => "4",
            5 => "5",
            6 => "6",
            7 => "7",
            8 => "8",
            9 => "9",
            10 => "10",
            11 => "J",
            12 => "Q",
            13 => "K",
            _ => "", // This will never occur
        };

        value.to_string() + &suit.to_string()
    }

    pub fn print(&self) {
        println!("{}", self.get_card_string());
    }
}
