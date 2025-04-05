use rand::Rng;

#[derive(Debug, PartialEq)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug, PartialEq)]
pub enum Rank {
    Ace,
    Number(u8),
    Jack,
    Queen,
    King,
}

impl Suit {
    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
            _ => panic!("Invalid suit value"),
        }
    }

    pub fn random() -> Suit {
        let mut rng = rand::thread_rng();
        let val = rng.gen_range(1..=4); // 1 to 4
        Suit::translate(val)
    }
}

impl Rank {
    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            2..=10 => Rank::Number(value),
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => panic!("Invalid rank value"),
        }
    }

    pub fn random() -> Rank {
        let mut rng = rand::thread_rng();
        let val = rng.gen_range(1..=13);
        Rank::translate(val)
    }
}

#[derive(Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: Card) -> bool {
    card.suit == Suit::Spade && card.rank == Rank::Ace
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_winner_card() {
        let card = Card {
            suit: Suit::Spade,
            rank: Rank::Ace,
        };
        assert!(winner_card(card));

        let card2 = Card {
            suit: Suit::Heart,
            rank: Rank::King,
        };
        assert!(!winner_card(card2));
    }
}
