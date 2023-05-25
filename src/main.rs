pub mod classes;

fn main() {
    let game = classes::game::Game::new();
    println!("{:?}", game);
    game.calculate_score();
}
