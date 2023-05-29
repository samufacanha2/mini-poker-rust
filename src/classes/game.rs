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

        let player1 = Player::new(1, deck.deal(3));
        let player2 = Player::new(2, deck.deal(3));
        let player3 = Player::new(3, deck.deal(3));
        let player4 = Player::new(4, deck.deal(3));

        let table_cards = deck.deal(2);

        Game {
            players: vec![player1, player2, player3, player4],
            deck,
            table_cards,
        }
    }

    fn set_scores(&mut self, scores: Vec<i8>) {
        for (i, score) in scores.iter().enumerate() {
            self.players[i].set_score(*score);
        }
    }

    pub fn calculate_score(&mut self) {
        let mut scores = vec![];

        for player in &self.players {
            print!(
                "\n
                ------------------------------------
                \n\n"
            );
            let mut score = 0;
            let mut max_combination_number = 0;
            let mut combination_number = 0;

            let cards = player.cards.clone();

            let mut cards = cards
                .iter()
                .chain(self.table_cards.iter())
                .collect::<Vec<_>>();

            cards.sort_by(|a, b| a.cmp(b));

            for i in 0..3 {
                for j in (i + 1)..4 {
                    for k in (j + 1)..5 {
                        combination_number += 1;
                        let cards = [cards[i], cards[j], cards[k]];

                        print!(
                            "Player {}, combination #{} : {:?}, ",
                            player.id, combination_number, cards
                        );

                        let mut consecutive_cards = 0;

                        for i in 0..cards.len() - 1 {
                            if cards[i].value as i8 + 1 == cards[i + 1].value as i8 {
                                consecutive_cards += 1;
                            }
                            if cards[i].value as i8 == 1 && cards[i + 1].value as i8 == 13 {
                                consecutive_cards += 1;
                            }
                        }

                        print!("Consecutive cards: {}, ", consecutive_cards);

                        let mut same_suit = 0;

                        for i in 0..cards.len() - 1 {
                            if cards[i].suit == cards[i + 1].suit {
                                same_suit += 1;
                            }
                        }

                        println!("Same suit: {}\n", same_suit);

                        let current_score = 2 * consecutive_cards + same_suit;

                        if current_score > score {
                            score = current_score;
                            max_combination_number = combination_number;
                        }
                    }
                }
            }
            scores = scores.iter().chain(vec![score].iter()).cloned().collect();

            println!(
                "Player {} obtained a maximum score of {:?} in combination #{}",
                player.id, score, max_combination_number
            );
        }

        self.set_scores(scores);
    }
}

impl Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Players: {:?} \n\nTable Cards: {:?}\n",
            self.players, self.table_cards
        )
    }
}
