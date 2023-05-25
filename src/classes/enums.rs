use std::fmt::Debug;

pub enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

pub enum CardValue {
    Ace = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
}

impl Debug for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Suit::Hearts => write!(f, "Hearts"),
            Suit::Diamonds => write!(f, "Diamonds"),
            Suit::Spades => write!(f, "Spades"),
            Suit::Clubs => write!(f, "Clubs"),
        }
    }
}

impl Debug for CardValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CardValue::Ace => write!(f, "Ace"),
            CardValue::Two => write!(f, "Two"),
            CardValue::Three => write!(f, "Three"),
            CardValue::Four => write!(f, "Four"),
            CardValue::Five => write!(f, "Five"),
            CardValue::Six => write!(f, "Six"),
            CardValue::Seven => write!(f, "Seven"),
            CardValue::Eight => write!(f, "Eight"),
            CardValue::Nine => write!(f, "Nine"),
            CardValue::Ten => write!(f, "Ten"),
            CardValue::Jack => write!(f, "Jack"),
            CardValue::Queen => write!(f, "Queen"),
            CardValue::King => write!(f, "King"),
        }
    }
}

impl Iterator for CardValue {
    type Item = CardValue;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            CardValue::Ace => Some(CardValue::Two),
            CardValue::Two => Some(CardValue::Three),
            CardValue::Three => Some(CardValue::Four),
            CardValue::Four => Some(CardValue::Five),
            CardValue::Five => Some(CardValue::Six),
            CardValue::Six => Some(CardValue::Seven),
            CardValue::Seven => Some(CardValue::Eight),
            CardValue::Eight => Some(CardValue::Nine),
            CardValue::Nine => Some(CardValue::Ten),
            CardValue::Ten => Some(CardValue::Jack),
            CardValue::Jack => Some(CardValue::Queen),
            CardValue::Queen => Some(CardValue::King),
            CardValue::King => None,
        }
    }
}

impl PartialEq for CardValue {
    fn eq(&self, other: &Self) -> bool {
        match self {
            CardValue::Ace => match other {
                CardValue::Ace => true,
                _ => false,
            },
            CardValue::Two => match other {
                CardValue::Two => true,
                _ => false,
            },
            CardValue::Three => match other {
                CardValue::Three => true,
                _ => false,
            },
            CardValue::Four => match other {
                CardValue::Four => true,
                _ => false,
            },
            CardValue::Five => match other {
                CardValue::Five => true,
                _ => false,
            },
            CardValue::Six => match other {
                CardValue::Six => true,
                _ => false,
            },
            CardValue::Seven => match other {
                CardValue::Seven => true,
                _ => false,
            },
            CardValue::Eight => match other {
                CardValue::Eight => true,
                _ => false,
            },
            CardValue::Nine => match other {
                CardValue::Nine => true,
                _ => false,
            },
            CardValue::Ten => match other {
                CardValue::Ten => true,
                _ => false,
            },
            CardValue::Jack => match other {
                CardValue::Jack => true,
                _ => false,
            },
            CardValue::Queen => match other {
                CardValue::Queen => true,
                _ => false,
            },
            CardValue::King => match other {
                CardValue::King => true,
                _ => false,
            },
        }
    }
}

impl PartialOrd for CardValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if *self as i8 > *other as i8 {
            Some(std::cmp::Ordering::Greater)
        } else if (*self as i8) < (*other as i8) {
            Some(std::cmp::Ordering::Less)
        } else {
            Some(std::cmp::Ordering::Equal)
        }
    }
}

impl Eq for CardValue {}

impl Ord for CardValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if *self as i8 > *other as i8 {
            std::cmp::Ordering::Greater
        } else if (*self as i8) < (*other as i8) {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Equal
        }
    }
}

impl Copy for CardValue {}

impl Clone for CardValue {
    fn clone(&self) -> Self {
        *self
    }
}
