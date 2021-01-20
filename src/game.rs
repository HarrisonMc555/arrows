use array2d::Array2D;
use std::collections::HashSet;
use std::convert::TryFrom;

pub type Board = Array2D<Cell>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Game {
    pub board: Board,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Cell {
    pointer: Pointer,
    number: Option<Number>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Pointer {
    Go(Direction),
    Final,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Direction {
    North,
    Northeast,
    East,
    Southeast,
    South,
    Southwest,
    West,
    Northwest,
}

pub type Number = usize;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Error {
    EmptyBoard,
    MultipleOfNumber(Number),
    NumberTooHigh(Number),
    NoZeroAllowed,
    WrongFinalNumber { actual: Number, expected: Number },
    FinalNumberWithDirection(Number, Direction),
}

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

impl Game {
    #[allow(dead_code)]
    pub fn new(board: Board) -> Result<Self, Error> {
        if board.num_elements() == 0 {
            return Err(Error::EmptyBoard);
        }
        let max_number = board.num_elements();
        let mut seen = HashSet::new();

        for (pointer, number) in board
            .elements_row_major_iter()
            .flat_map(|cell| cell.pointer_number())
        {
            match pointer {
                Pointer::Go(direction) => {
                    if number == max_number {
                        return Err(Error::FinalNumberWithDirection(number, direction));
                    }
                }
                Pointer::Final => {
                    if number != max_number {
                        return Err(Error::WrongFinalNumber {
                            actual: number,
                            expected: max_number,
                        });
                    }
                }
            };

            if seen.contains(&number) {
                return Err(Error::MultipleOfNumber(number));
            }
            if number > max_number {
                return Err(Error::NumberTooHigh(number));
            }

            seen.insert(number);
        }

        Ok(Self { board })
    }

    pub fn example() -> Self {
        Self {
            board: Array2D::from_rows(&vec![
                vec![cell!("e", 1), cell!("e"), cell!("s"), cell!("w", 3)],
                vec![cell!("s"), cell!("s", 12), cell!("w", 5), cell!("w")],
                vec![cell!("se"), cell!("w"), cell!("e"), cell!("n")],
                vec![cell!("e"), cell!("e"), cell!("n"), cell!("*", 16)],
            ])
            .unwrap(),
        }
    }

    pub fn to_strings(&self) -> Vec<String> {
        self.board
            .rows_iter()
            .map(|row| self.row_to_string(row))
            .collect()
    }

    fn row_to_string<'a, T>(&'a self, row_iter: T) -> String
    where
        T: Iterator<Item = &'a Cell>,
    {
        row_iter
            .map(|cell| self.cell_to_string(cell))
            .collect::<Vec<_>>()
            .join("|")
            .into()
    }

    fn cell_to_string(&self, cell: &Cell) -> String {
        let pointer_string = match cell.pointer {
            Pointer::Go(d) => d.to_unicode_arrow(),
            Pointer::Final => "☆",
        };
        let max_num = self.board.num_elements();
        let num_digits = log10(max_num);
        let number_string = match cell.number {
            Some(n) => n.to_string(),
            None => "".to_string(),
        };
        format!(
            "{: >width$} {}",
            number_string,
            pointer_string,
            width = num_digits
        )
    }
}

impl Cell {
    pub fn new(pointer: Pointer, number: Option<Number>) -> Result<Self, Error> {
        if number == Some(0) {
            return Err(Error::NoZeroAllowed);
        }
        Ok(Self { pointer, number })
    }

    pub fn pointer_number(self) -> Option<(Pointer, Number)> {
        self.number.map(|n| (self.pointer, n))
    }
}

impl<'a> TryFrom<&'a str> for Direction {
    type Error = &'a str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Ok(match value {
            "n" => Self::North,
            "ne" => Self::Northeast,
            "e" => Self::East,
            "se" => Self::Southeast,
            "s" => Self::South,
            "sw" => Self::Southwest,
            "w" => Self::West,
            "nw" => Self::Northwest,
            _ => return Err(value),
        })
    }
}

impl Direction {
    fn to_unicode_arrow(self) -> &'static str {
        match self {
            Self::North => "⇑",
            Self::Northeast => "⇗",
            Self::East => "⇒",
            Self::Southeast => "⇘",
            Self::South => "⇓",
            Self::Southwest => "⇙",
            Self::West => "⇐",
            Self::Northwest => "⇖",
        }
    }
}

fn log10(num: usize) -> usize {
    let num = num as f64;
    let exponent = num.log10();
    let exponent_ceil = exponent.ceil();
    exponent_ceil as usize
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn valid_board() {
        let board = Array2D::from_rows(&vec![
            vec![cell!("e", 1), cell!("e"), cell!("s")],
            vec![cell!("se"), cell!("w", 5), cell!("w", 4)],
            vec![cell!("e"), cell!("w"), cell!("*", 9)],
        ])
        .unwrap();
        let result = Game::new(board);
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    fn empty_board() {
        let board = Array2D::from_rows(&vec![]).unwrap();
        let result = Game::new(board);
        println!("{:?}", result);
        assert_eq!(result, Err(Error::EmptyBoard));
    }

    #[test]
    fn multiple_numbers() {
        let board = Array2D::from_rows(&vec![
            vec![cell!("e", 1), cell!("e"), cell!("s")],
            vec![cell!("se"), cell!("w", 5), cell!("w", 4)],
            vec![cell!("e", 5), cell!("w"), cell!("*", 9)],
        ])
        .unwrap();
        let result = Game::new(board);
        println!("{:?}", result);
        assert_eq!(result, Err(Error::MultipleOfNumber(5)));
    }

    #[test]
    fn number_too_high() {
        let board = Array2D::from_rows(&vec![
            vec![cell!("e", 1), cell!("e"), cell!("s")],
            vec![cell!("se"), cell!("w", 5), cell!("w", 4)],
            vec![cell!("e", 10), cell!("w"), cell!("*", 9)],
        ])
        .unwrap();
        let result = Game::new(board);
        println!("{:?}", result);
        assert_eq!(result, Err(Error::NumberTooHigh(10)));
    }

    #[test]
    fn number_zero() {
        assert_eq!(
            Cell::new(Pointer::Go(Direction::East), Some(0)),
            Err(Error::NoZeroAllowed)
        )
    }

    #[test]
    fn wrong_final_number() {
        let board = Array2D::from_rows(&vec![
            vec![cell!("e", 1), cell!("e"), cell!("s")],
            vec![cell!("se"), cell!("w", 5), cell!("w", 4)],
            vec![cell!("e", 8), cell!("w"), cell!("*", 8)],
        ])
        .unwrap();
        let result = Game::new(board);
        assert_eq!(
            result,
            Err(Error::WrongFinalNumber {
                actual: 8,
                expected: 9,
            })
        );
    }

    #[test]
    fn final_number_with_direction() {
        let board = Array2D::from_rows(&vec![
            vec![cell!("e", 1), cell!("e"), cell!("s")],
            vec![cell!("se"), cell!("w", 5), cell!("w", 4)],
            vec![cell!("e", 8), cell!("w"), cell!("e", 9)],
        ])
        .unwrap();
        let result = Game::new(board);
        assert_eq!(
            result,
            Err(Error::FinalNumberWithDirection(9, Direction::East)),
        );
    }
}
