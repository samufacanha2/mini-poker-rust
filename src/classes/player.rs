use std::fmt::Debug;

use super::card::Card;

#[derive(Clone)]
pub struct Player {
    pub id: i8,
    pub cards: Vec<Card>,
    pub score: i8,
}

impl Player {
    pub fn new(id: i8, cards: Vec<Card>) -> Player {
        Player {
            id,
            cards,
            score: 0,
        }
    }

    pub fn set_score(&mut self, score: i8) {
        self.score = score;
    }
}

impl Debug for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "\nPlayer {}: {:?}", self.id, self.cards)
    }
}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Player) -> Option<std::cmp::Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl Ord for Player {
    fn cmp(&self, other: &Player) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Player) -> bool {
        self.score == other.score
    }
}

impl Eq for Player {}
