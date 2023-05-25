use std::fmt::{self, Debug};

use rand::Rng;

use super::enums::{CardValue, Suit};

pub struct Card {
    pub suit: Suit,
    pub value: CardValue,
}

impl Card {
    pub fn random() -> Card {
        let mut rng = rand::thread_rng();

        let suit = rng.gen_range(0..=4);
        let value = rng.gen_range(0..=13);
        Card {
            suit: match suit {
                0 => Suit::Hearts,
                1 => Suit::Diamonds,
                2 => Suit::Spades,
                3 => Suit::Clubs,
                _ => panic!("Invalid suit"),
            },
            value: match value {
                0 => CardValue::Ace,
                1 => CardValue::Two,
                2 => CardValue::Three,
                3 => CardValue::Four,
                4 => CardValue::Five,
                5 => CardValue::Six,
                6 => CardValue::Seven,
                7 => CardValue::Eight,
                8 => CardValue::Nine,
                9 => CardValue::Ten,
                10 => CardValue::Jack,
                11 => CardValue::Queen,
                12 => CardValue::King,
                _ => panic!("Invalid value"),
            },
        }
    }
}

impl Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} of {:?}", self.value, self.suit)
    }
}
