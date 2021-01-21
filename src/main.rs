mod game;
mod solver;

use array2d::Array2D;
use game::{Cell, Direction, Game, Pointer};
use solver::Solver;

macro_rules! cell {
    ($direction:tt) => {
        Cell::new(dir!($direction), None).unwrap();
    };
    ($_:tt, 0) => {
        compile_error!("Cell numbers must be non-zero");
    };
    ($direction:tt, $number:tt) => {
        Cell::new(dir!($direction), Some($number)).unwrap();
    };
}

macro_rules! dir {
    ("n") => {
        Pointer::Go(Direction::North);
    };
    ("n") => {
        Pointer::Go(Direction::North);
    };
    ("ne") => {
        Pointer::Go(Direction::Northeast);
    };
    ("e") => {
        Pointer::Go(Direction::East);
    };
    ("se") => {
        Pointer::Go(Direction::Southeast);
    };
    ("s") => {
        Pointer::Go(Direction::South);
    };
    ("sw") => {
        Pointer::Go(Direction::Southwest);
    };
    ("w") => {
        Pointer::Go(Direction::West);
    };
    ("nw") => {
        Pointer::Go(Direction::Northwest);
    };
    ("*") => {
        Pointer::Final;
    };
}

fn main() {
    let game = Game::example();
    for row in game.to_strings() {
        println!("{}", row);
    }
    println!();

    let solved =
        Game::new(Solver::solve(game.board).expect("No solution")).expect("Invalid solution board");
    for row in solved.to_strings() {
        println!("{}", row);
    }
    println!();

    let board = Array2D::from_rows(&vec![
        vec![cell!("e", 1), cell!("s"), cell!("w", 5), cell!("sw")],
        vec![cell!("se"), cell!("se"), cell!("s"), cell!("sw")],
        vec![cell!("ne"), cell!("e"), cell!("w"), cell!("n")],
        vec![cell!("n"), cell!("w"), cell!("n"), cell!("*", 16)],
    ])
    .unwrap();
    let solved =
        Game::new(Solver::solve(board).expect("No solution")).expect("Invalid solution board");
    for row in solved.to_strings() {
        println!("{}", row);
    }
}
