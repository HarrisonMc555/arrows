#![allow(dead_code, unused_variables, unreachable_patterns)]

use crate::game::Direction::*;
use crate::game::*;
use std::cmp::Ordering::*;
use std::collections::HashMap;

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

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Error {
    ImpossibleBoard,
    Internal(String),
}

impl Solver {
    fn new(board: Board) -> Self {
        let num_to_index = Self::create_num_to_index(&board);
        Solver {
            board,
            num_to_index,
        }
    }

    pub fn solve(board: Board) -> Result<Board, Error> {
        let mut solver = Solver::new(board);
        solver.solve_internal(1)?;
        Ok(solver.board)
    }

    fn solve_internal(&mut self, number: Number) -> Result<(), Error> {
        if number >= self.board.num_elements() {
            return Ok(());
        }
        if self.num_to_index.contains_key(&number) {
            return self.solve_internal(number + 1);
        }

        let prev_number = number - 1;
        let possible_indices = self.get_possible_indices_from_prev(prev_number);
        unimplemented!()
    }

    fn get_possible_indices_from_prev(&self, prev_number: Number) -> Result<Vec<Index>, Error> {
        let prev_index = match self.num_to_index.get(&prev_number) {
            Some(prev_index) => prev_index,
            // None => return (1..=self.max_number()).filter(|n|)
            None => return Ok(self.get_empty_indices()),
        };

        let prev_pointer = self.board[prev_index.row_column()].pointer;
        let direction = match prev_pointer {
            Pointer::Go(direction) => direction,
            Pointer::Final => {
                return Err(Error::Internal(format!(
                    "Previous index {:?} was final",
                    prev_index
                )))
            }
        };

        Ok(self.get_empty_indices_in_direction(*prev_index, direction))
    }

    fn get_empty_indices_in_direction(&self, index: Index, direction: Direction) -> Vec<Index> {
        let mut index = index;
        let mut indices = Vec::new();
        loop {
            index = match index.step(direction) {
                Some(index) => index,
                None => return indices,
            };
            if index.row >= self.board.num_rows() || index.column >= self.board.num_columns() {
                return indices;
            }
            if self.board[index.row_column()].number.is_none() {
                indices.push(index);
            }
        }
    }

    fn get_empty_indices(&self) -> Vec<Index> {
        self.board
            .enumerate_row_major()
            .filter_map(|((row, column), cell)| {
                if cell.number.is_some() {
                    None
                } else {
                    Some(Index::new(row, column))
                }
            })
            .collect()
    }

    fn max_number(&self) -> Number {
        self.board.num_elements()
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

    pub fn row_column(self) -> (usize, usize) {
        (self.row, self.column)
    }

    pub fn step(self, direction: Direction) -> Option<Self> {
        let row = self.row;
        let column = self.column;
        let new_row = match direction {
            Northwest | North | Northeast => row.checked_sub(1)?,
            Southwest | South | Southeast => row + 1,
            _ => row,
        };
        let new_column = match direction {
            Northwest | West | Southwest => column.checked_sub(1)?,
            Northeast | East | Southeast => column + 1,
            _ => column,
        };
        Some(Self {
            row: new_row,
            column: new_column,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use array2d::Array2D;

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
        let solver = Solver::new(board);

        let num_to_index = vec![(1, (0, 0)), (5, (1, 1)), (4, (1, 2)), (9, (2, 2))]
            .into_iter()
            .map(|(num, (row, column))| (num, Index::new(row, column)))
            .collect::<HashMap<_, _>>();

        assert_eq!(solver.num_to_index, num_to_index);
    }

    #[test]
    fn test_step() {
        let index = |row, column| Some(Index::new(row, column));

        let middle = Index::new(10, 10);
        assert_eq!(middle.step(North), index(9, 10));
        assert_eq!(middle.step(Northeast), index(9, 11));
        assert_eq!(middle.step(East), index(10, 11));
        assert_eq!(middle.step(Southeast), index(11, 11));
        assert_eq!(middle.step(South), index(11, 10));
        assert_eq!(middle.step(Southwest), index(11, 9));
        assert_eq!(middle.step(West), index(10, 9));
        assert_eq!(middle.step(Northwest), index(9, 9));

        let corner = Index::new(0, 0);
        assert_eq!(corner.step(North), None);
        assert_eq!(corner.step(Northeast), None);
        assert_eq!(corner.step(East), index(0, 1));
        assert_eq!(corner.step(Southeast), index(1, 1));
        assert_eq!(corner.step(South), index(1, 0));
        assert_eq!(corner.step(Southwest), None);
        assert_eq!(corner.step(West), None);
        assert_eq!(corner.step(Northwest), None);

        let edge_north = Index::new(0, 10);
        assert_eq!(edge_north.step(North), None);
        assert_eq!(edge_north.step(Northeast), None);
        assert_eq!(edge_north.step(East), index(0, 11));
        assert_eq!(edge_north.step(Southeast), index(1, 11));
        assert_eq!(edge_north.step(South), index(1, 10));
        assert_eq!(edge_north.step(Southwest), index(1, 9));
        assert_eq!(edge_north.step(West), index(0, 9));
        assert_eq!(edge_north.step(Northwest), None);

        let edge_west = Index::new(10, 0);
        assert_eq!(edge_west.step(North), index(9, 0));
        assert_eq!(edge_west.step(Northeast), index(9, 1));
        assert_eq!(edge_west.step(East), index(10, 1));
        assert_eq!(edge_west.step(Southeast), index(11, 1));
        assert_eq!(edge_west.step(South), index(11, 0));
        assert_eq!(edge_west.step(Southwest), None);
        assert_eq!(edge_west.step(West), None);
        assert_eq!(edge_west.step(Northwest), None);
    }

    #[test]
    fn test_empty_indices() {
        let board = Array2D::from_rows(&vec![
            vec![cell!("e", 1), cell!("e"), cell!("s")],
            vec![cell!("se"), cell!("w", 5), cell!("w", 4)],
            vec![cell!("e"), cell!("w"), cell!("*", 9)],
        ])
        .unwrap();
        let solver = Solver::new(board);

        let actual = solver.get_empty_indices();
        let expected = vec![(0, 1), (0, 2), (1, 0), (2, 0), (2, 1)]
            .into_iter()
            .map(|(row, column)| Index::new(row, column))
            .collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_indices_in_direction() {
        let board = Array2D::from_rows(&vec![
            vec![cell!("e", 1), cell!("e"), cell!("s")],
            vec![cell!("se"), cell!("w", 5), cell!("w", 4)],
            vec![cell!("e"), cell!("w"), cell!("*", 9)],
        ])
        .unwrap();
        let solver = Solver::new(board);

        let indices = |tuples: Vec<(usize, usize)>| {
            tuples
                .into_iter()
                .map(|(row, column)| Index::new(row, column))
                .collect::<Vec<_>>()
        };

        let actual = solver.get_empty_indices_in_direction(Index::new(0, 0), East);
        let expected = indices(vec![(0, 1), (0, 2)]);
        assert_eq!(actual, expected);

        let actual = solver.get_empty_indices_in_direction(Index::new(0, 0), South);
        let expected = indices(vec![(1, 0), (2, 0)]);
        assert_eq!(actual, expected);

        let actual = solver.get_empty_indices_in_direction(Index::new(0, 0), Southeast);
        let expected = vec![];
        assert_eq!(actual, expected);

        let actual = solver.get_empty_indices_in_direction(Index::new(0, 2), Southwest);
        let expected = indices(vec![(2, 0)]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_possible_indices_from_prev() -> Result<(), super::Error> {
        let board = Array2D::from_rows(&vec![
            vec![cell!("e", 1), cell!("e"), cell!("s")],
            vec![cell!("se"), cell!("w", 5), cell!("w", 4)],
            vec![cell!("e"), cell!("w"), cell!("*", 9)],
        ])
        .unwrap();
        let solver = Solver::new(board);

        let indices = |tuples: Vec<(usize, usize)>| {
            tuples
                .into_iter()
                .map(|(row, column)| Index::new(row, column))
                .collect::<Vec<_>>()
        };

        let actual = solver.get_possible_indices_from_prev(1)?;
        let expected = indices(vec![(0, 1), (0, 2)]);
        assert_eq!(actual, expected);

        let actual = solver.get_possible_indices_from_prev(2)?;
        let expected = indices(vec![(0, 1), (0, 2), (1, 0), (2, 0), (2, 1)]);
        assert_eq!(actual, expected);

        let actual = solver.get_possible_indices_from_prev(5)?;
        let expected = indices(vec![(1, 0)]);
        assert_eq!(actual, expected);

        Ok(())
    }
}
