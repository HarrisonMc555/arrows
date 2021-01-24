mod game;
mod parse;
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
    println!();

    solve(&vec![
        "s1,se,s21,se,s,e,sw,sw39",
        "e,e55,nw,s,sw56,w42,w41,sw8",
        "e,se,e,s,n,nw,n,s",
        "e,e48,e,w,s,se28,w,s",
        "s,s,se,se60,sw,n,se,w",
        "n6,n58,n,nw,e,n,w12,sw",
        "n,ne,e,ne,n,nw,s,nw24",
        "e,n,n19,n,w33,n37,w,*64",
    ]);

    solve(&vec![
        "s1,se,se,e46,w48,s14,s,w",
        "ne5,se59,s,w,se39,sw,sw,s",
        "se,se,se61,se,sw,s55,w,w25",
        "s,n,w57,sw,nw7,n,s52,w",
        "se,s22,se,w21,s,n13,sw,s",
        "n,ne,nw,n,w,n,s27,n",
        "e,e,e,n,n,nw56,nw,w41",
        "e,ne,ne12,n34,e,nw37,n,*64",
    ]);
}

fn solve(rows: &[&str]) {
    let text = rows.join("\n");
    let board =
        parse::parse_board::<(&str, nom::error::ErrorKind)>(&text).expect("Invalid board format");
    let solved_board = Solver::solve(board).expect("No solution");
    let game = Game::new(solved_board).expect("Invalid solution board");
    for row in game.to_strings() {
        println!("{}", row);
    }
    println!()
}
