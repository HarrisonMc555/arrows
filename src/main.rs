mod game;
mod solver;

use game::Game;
use solver::Solver;

fn main() {
    let game = Game::example();
    for row in game.to_strings() {
        println!("{}", row);
    }

    println!();
    let solved =
        Game::new(Solver::solve(game.board).expect("Invalid solution board")).expect("No solution");
    for row in solved.to_strings() {
        println!("{}", row);
    }
}
