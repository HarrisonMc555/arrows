#![allow(dead_code, unused_variables, unreachable_patterns)]

use crate::game::Direction::*;
use crate::game::*;
use array2d::Array2D;
use std::cmp::Ordering::*;
use std::collections::HashMap;
use std::num::NonZeroUsize;

#[derive(Debug)]
pub struct Solver {
    board: Board,
    num_to_index: HashMap<Number, Index>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Index {
    row: usize,
    column: usize,
}

impl Solver {
    pub fn new(game: Game) -> Self {
        let board = game.board;
        let num_to_index = Self::create_num_to_index(&board);
        Solver {
            board,
            num_to_index,
        }
    }

    fn create_num_to_index(board: &Board) -> HashMap<Number, Index> {
        board
            .enumerate_row_major()
            .flat_map(|((row, column), cell)| {
                let (_, number) = cell.pointer_number()?;
                let index = Index::new(row, column);
                Some((number, index))
            })
            .collect()
    }
}

fn get_direction(index1: Index, index2: Index) -> Option<Direction> {
    let Index {
        row: row1,
        column: column1,
    } = index1;
    let Index {
        row: row2,
        column: column2,
    } = index2;

    let row_diff = abs_difference(row1, row2);
    let column_diff = abs_difference(column1, column2);

    if row_diff != 0 && column_diff != 0 && row_diff != column_diff {
        // If both are non-zero, then it should be a straight diagonal line
        return None;
    }

    let north_south_cmp = row2.cmp(&row1);
    let east_west_cmp = column2.cmp(&column1);

    Some(match (north_south_cmp, east_west_cmp) {
        (Less, Less) => Northwest,
        (Less, Equal) => North,
        (Less, Greater) => Northeast,
        (Equal, Less) => West,
        (Equal, Equal) => return None,
        (Equal, Greater) => East,
        (Greater, Less) => Southwest,
        (Greater, Equal) => South,
        (Greater, Greater) => Southeast,
    })
}

fn abs_difference<T: std::ops::Sub<Output = T> + Ord>(x: T, y: T) -> T {
    if x < y {
        y - x
    } else {
        x - y
    }
}

impl Index {
    pub fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! cell {
        ($direction:tt) => {
            Cell::new(dir!($direction), None);
        };
        ($_:tt, 0) => {
            compile_error!("Cell numbers must be non-zero");
        };
        ($direction:tt, $number:tt) => {
            Cell::new(
                dir!($direction),
                Some(NonZeroUsize::new($number).expect("Cell numbers must be non-zero")),
            );
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

    #[test]
    fn test_get_direction() {
        let start = Index::new(10, 20);
        let dir = |row, column| get_direction(start, Index::new(row, column));

        assert_eq!(dir(10, 20), None);

        assert_eq!(dir(9, 20), Some(North));
        assert_eq!(dir(8, 20), Some(North));
        assert_eq!(dir(0, 20), Some(North));

        assert_eq!(dir(11, 20), Some(South));
        assert_eq!(dir(12, 20), Some(South));
        assert_eq!(dir(20, 20), Some(South));

        assert_eq!(dir(10, 21), Some(East));
        assert_eq!(dir(10, 22), Some(East));
        assert_eq!(dir(10, 30), Some(East));

        assert_eq!(dir(10, 19), Some(West));
        assert_eq!(dir(10, 18), Some(West));
        assert_eq!(dir(10, 10), Some(West));
        assert_eq!(dir(10, 0), Some(West));

        assert_eq!(dir(9, 19), Some(Northwest));
        assert_eq!(dir(8, 18), Some(Northwest));
        assert_eq!(dir(0, 10), Some(Northwest));

        assert_eq!(dir(9, 21), Some(Northeast));
        assert_eq!(dir(8, 22), Some(Northeast));
        assert_eq!(dir(0, 30), Some(Northeast));

        assert_eq!(dir(11, 19), Some(Southwest));
        assert_eq!(dir(12, 18), Some(Southwest));
        assert_eq!(dir(20, 10), Some(Southwest));

        assert_eq!(dir(11, 21), Some(Southeast));
        assert_eq!(dir(12, 22), Some(Southeast));
        assert_eq!(dir(20, 30), Some(Southeast));

        assert_eq!(dir(11, 22), None);
        assert_eq!(dir(11, 23), None);
        assert_eq!(dir(11, 18), None);
        assert_eq!(dir(11, 0), None);
        assert_eq!(dir(9, 22), None);
        assert_eq!(dir(9, 23), None);
        assert_eq!(dir(9, 18), None);
        assert_eq!(dir(9, 0), None);
        assert_eq!(dir(5, 55), None);
    }

    #[test]
    fn test_solver_new() {
        let board = Array2D::from_rows(&vec![
            vec![cell!("e", 1), cell!("e"), cell!("s")],
            vec![cell!("se"), cell!("w", 5), cell!("w", 4)],
            vec![cell!("e"), cell!("w"), cell!("*", 9)],
        ])
        .unwrap();
        let game = Game::new(board).unwrap();
        let solver = Solver::new(game);

        let num_to_index = vec![(1, (0, 0)), (5, (1, 1)), (4, (1, 2)), (9, (2, 2))]
            .into_iter()
            .map(|(num, (row, column))| (NonZeroUsize::new(num).unwrap(), Index::new(row, column)))
            .collect::<HashMap<_, _>>();

        assert_eq!(solver.num_to_index, num_to_index);
    }
}
