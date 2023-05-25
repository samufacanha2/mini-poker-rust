pub mod classes;

fn main() {
    let mut deck = classes::deck::Deck::new();

    deck.shuffle();

    let player1_cards = deck.deal(3);
    let player2_cards = deck.deal(3);
    let player3_cards = deck.deal(3);
    let player4_cards = deck.deal(3);

    let table_cards = deck.deal(2);

    println!("player one cards: {:?}", player1_cards);
    println!("player two cards: {:?}", player2_cards);
    println!("player three cards: {:?}", player3_cards);
    println!("player four cards: {:?}", player4_cards);

    println!("table cards: {:?}", table_cards);
}
