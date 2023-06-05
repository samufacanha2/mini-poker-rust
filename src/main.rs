use std::{
    io::{stdin, Read},
    process::Command,
};

pub mod classes;

fn main() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .output()
            .expect("failed to execute process");
    }

    let mut game = classes::game::Game::new();
    println!("{:?}", game);
    game.calculate_score();

    println!("\npress enter/return to exit.");

    stdin().read(&mut [0]).unwrap();
}
