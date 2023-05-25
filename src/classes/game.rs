use std::fmt::{self, Debug};

use super::{card::Card, deck::Deck, player::Player};

pub struct Game {
    pub players: Vec<Player>,
    pub deck: Deck,
    pub table_cards: Vec<Card>,
}

impl Game {
    pub fn new() -> Game {
        let mut deck = Deck::new();
        deck.shuffle();

        let player1 = Player::new(deck.deal(3));
        let player2 = Player::new(deck.deal(3));
        let player3 = Player::new(deck.deal(3));
        let player4 = Player::new(deck.deal(3));

        let table_cards = deck.deal(2);

        Game {
            players: vec![player1, player2, player3, player4],
            deck,
            table_cards,
        }
    }

    pub fn calculate_score(&self) {
        let mut score = 0;

        for player in &self.players {
            //  TODO: order cards by value
            //  TODO: get points from consecutive cards
            //  TODO: get points from same suit

            for card in &player.cards {
                score += card.value as i8
            }

            println!("{:?} has a score of {:?}", player, score);
        }
    }
}

impl Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Players: {:?}\nTable Cards: {:?}",
            self.players, self.table_cards
        )
    }
}
