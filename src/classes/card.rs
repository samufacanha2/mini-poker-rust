use std::fmt::{self, Debug};

use rand::Rng;

use super::{card_suit::Suit, card_value::CardValue};

pub struct Card {
    pub suit: Suit,
    pub value: CardValue,
}

impl Card {
    pub fn random() -> Card {
        let mut rng = rand::thread_rng();

        let suit = rng.gen_range(0..=3);
        let value = rng.gen_range(0..=12);
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

impl Clone for Card {
    fn clone(&self) -> Card {
        Card {
            suit: self.suit.clone(),
            value: self.value.clone(),
        }
    }
}

impl Copy for Card {}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.suit == other.suit
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.value == other.value {
            Some(self.suit.cmp(other.suit))
        } else {
            Some(self.value.cmp(other.value))
        }
    }
}

impl Eq for Card {}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.value == other.value {
            self.suit.cmp(other.suit)
        } else {
            self.value.cmp(other.value)
        }
    }
}
