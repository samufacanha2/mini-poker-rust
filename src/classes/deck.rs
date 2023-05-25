use super::{card::Card, enums::{Suit, CardValue}};

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards = Vec::new();
        for suit in 0..4 {
            for value in 0..13 {
                cards.push(Card {
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
                });
            }
        }
        Deck { cards }
    }


    pub fn deal(&mut self, number: i8) -> Vec<Card> {
        let mut cards = Vec::new();

        for _ in 0..number {
            cards.push(self.cards.pop().unwrap());
        }

        cards
    }

    pub fn shuffle(&mut self) {
        use rand::seq::SliceRandom;
        self.cards.shuffle(&mut rand::thread_rng());
    }

}
