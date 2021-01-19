mod game;
mod solver;

use game::Game;

fn main() {
    let game = Game::example();
    for row in game.to_strings() {
        println!("{}", row);
    }
}
