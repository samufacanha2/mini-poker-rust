use std::fmt::Debug;

use super::card::Card;

pub struct Player {
    pub cards: Vec<Card>,
    pub score: i8,
}

impl Player {
    pub fn new(cards: Vec<Card>) -> Player {
        Player { cards, score: 0 }
    }
}

impl Debug for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Player: {:?}", self.cards)
    }
}
