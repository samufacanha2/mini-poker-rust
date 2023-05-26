use std::fmt::Debug;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
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
