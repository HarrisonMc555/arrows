use array2d::Array2D;
use std::convert::TryFrom;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Game {
    board: Array2D<Cell>,
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

pub enum Error {
    MultipleOfNumber(Number),
    NumberTooHigh(Number),
    NoZeroAllowed,
}

macro_rules! cell {
    ($direction:tt) => {
        Cell::new(dir!($direction), None);
    };
    ($direction:tt, $number:tt) => {
        Cell::new(dir!($direction), Some($number));
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
    pub fn new(board: Array2D<Cell>) -> Result<Self, Error> {
        let numbers = board
            .elements_row_major_iter()
            .flat_map(|cell| cell.number)
            .collect::<Vec<_>>();
        Ok(Self { board })
    }

    pub fn example() -> Self {
        Self {
            board: Array2D::from_rows(&vec![
                vec![cell!("e", 1), cell!("e"), cell!("s"), cell!("w", 3)],
                vec![cell!("s"), cell!("s", 12), cell!("w", 5), cell!("w")],
                vec![cell!("se"), cell!("w"), cell!("e"), cell!("n")],
                vec![cell!("e"), cell!("e"), cell!("n"), cell!("*", 16)],
            ]),
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
    pub fn new(pointer: Pointer, number: Option<Number>) -> Self {
        Self { pointer, number }
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
