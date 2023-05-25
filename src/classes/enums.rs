use std::fmt::Debug;

pub enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs
}

pub enum CardValue {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack, 
    Queen,
    King
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