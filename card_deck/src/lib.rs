use rand::Rng;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

impl Suit {
    pub fn random() -> Suit {
        match rand::thread_rng().gen_range(1..=4) {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            _ => Suit::Club,
  
        }
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
            _ => panic!("Invalid suit value!"),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

impl Rank {
    pub fn random() -> Self {
        match rand::thread_rng().gen_range(1..=13) {
            1 => Rank::Ace,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            n => Rank::Number(n),
        }
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            2..=10 => Rank::Number(value),
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => panic!("Invalid rank value!"),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: Card) -> bool {
    card == Card {
        suit: Suit::Spade,
        rank: Rank::Ace,
    }
}