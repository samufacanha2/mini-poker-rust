#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Suit {
    Hearts = 0,
    Diamonds = 1,
    Spades = 2,
    Clubs = 3,
}

impl Iterator for Suit {
    type Item = Suit;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Suit::Hearts => Some(Suit::Diamonds),
            Suit::Diamonds => Some(Suit::Spades),
            Suit::Spades => Some(Suit::Clubs),
            Suit::Clubs => None,
        }
    }
}
