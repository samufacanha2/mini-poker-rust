pub mod classes;

fn main() {
    let mut game = classes::game::Game::new();
    println!("{:?}", game);
    game.calculate_score();
}
