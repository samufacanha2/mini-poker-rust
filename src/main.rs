use std::io::{stdin, Read};

pub mod classes;

fn main() {
    let mut game = classes::game::Game::new();
    println!("{:?}", game);
    game.calculate_score();

    println!("\npress enter/return to exit.");

    stdin().read(&mut [0]).unwrap();
}
