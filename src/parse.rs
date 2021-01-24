#![allow(unused_imports, unreachable_code, dead_code, unused_variables)]
use crate::game;
use crate::game::{Board, Cell, Direction, Game, Pointer};
use array2d::Array2D;
use nom;
use nom::bytes::complete::tag;
use nom::error::ErrorKind;
use nom::Finish;
use nom::Parser;

type I<'a> = &'a str;

pub fn parse_board<'a, E>(text: &'a str) -> Result<Board, E>
where
    E: nom::error::ParseError<&'a str>,
{
    let (_, rows) = rows(text).finish()?;
    // for (i, row) in rows.iter().enumerate() {
    //     println!("Row {} has {} elements", i, row.len());
    // }
    Ok(Array2D::from_rows(&rows).expect("Parser returned but invalid board"))
}

fn rows<'a, E>(text: &'a str) -> nom::IResult<&'a str, Vec<Vec<Cell>>, E>
where
    E: nom::error::ParseError<&'a str>,
{
    nom::multi::separated_list1(nom::character::complete::line_ending, row)(text)
}

fn row<'a, E>(text: &'a str) -> nom::IResult<&'a str, Vec<Cell>, E>
where
    E: nom::error::ParseError<&'a str>,
{
    nom::multi::separated_list1(comma, cell)(text)
}

fn comma<'a, E>(text: &'a str) -> nom::IResult<&'a str, (), E>
where
    E: nom::error::ParseError<&'a str>,
{
    nom::combinator::map(
        nom::sequence::tuple((
            nom::character::complete::space0,
            tag(","),
            nom::character::complete::space0,
        )),
        |_| (),
    )(text)
}

fn cell<'a, E>(text: &'a str) -> nom::IResult<&'a str, Cell, E>
where
    E: nom::error::ParseError<&'a str>,
{
    let (remaining, (pointer, number)) = nom::sequence::tuple((
        pointer,
        nom::combinator::opt(nom::character::complete::digit1),
    ))(text)?;
    let number = match number {
        Some(s) => Some(
            s.parse()
                .map_err(|_| nom::Err::Error(E::from_error_kind(text, ErrorKind::Digit)))?,
        ),
        None => None,
    };
    let cell = Cell::new(pointer, number)
        .map_err(|_| nom::Err::Error(E::from_error_kind(text, ErrorKind::Digit)))?;
    Ok((remaining, cell))
}

fn pointer<'a, E>(text: &'a str) -> nom::IResult<&'a str, Pointer, E>
where
    E: nom::error::ParseError<&'a str>,
{
    nom::branch::alt((
        nom::combinator::map(tag("*"), |_| Pointer::Final),
        nom::combinator::map(dir, |d| Pointer::Go(d)),
    ))(text)
}

fn dir<'a, E>(text: &'a str) -> nom::IResult<&'a str, Direction, E>
where
    E: nom::error::ParseError<&'a str>,
{
    map_tags(vec![
        ("ne", Direction::Northeast),
        ("se", Direction::Southeast),
        ("sw", Direction::Southwest),
        ("nw", Direction::Northwest),
        ("n", Direction::North),
        ("e", Direction::East),
        ("s", Direction::South),
        ("w", Direction::West),
    ])(text)
    // nom::branch::alt((
    //     map_to(tag("ne"), Direction::Northeast),
    //     map_to(tag("se"), Direction::Southeast),
    //     map_to(tag("sw"), Direction::Southwest),
    //     map_to(tag("nw"), Direction::Northwest),
    //     map_to(tag("n"), Direction::North),
    //     map_to(tag("e"), Direction::East),
    //     map_to(tag("s"), Direction::South),
    //     map_to(tag("w"), Direction::West),
    // ))(text)
}

fn map_tags<T, I, O, E>(pairs: Vec<(T, O)>) -> impl FnMut(I) -> nom::IResult<I, O, E>
where
    T: nom::InputLength + Clone,
    I: nom::InputTake + nom::InputIter + nom::Compare<T> + Clone,
    O: Clone,
    E: nom::error::ParseError<I>,
{
    move |input: I| {
        for (tag_text, out) in pairs.clone() {
            let result = tag(tag_text)(input.clone());
            if result.is_ok() {
                return result.map(|(remaining, _)| (remaining, out.clone()));
            }
        }
        Err(nom::Err::Error(E::from_error_kind(input, ErrorKind::Tag)))
    }
}

fn map_to<I, O1, O2, E, F>(parser: F, value: O2) -> impl FnMut(I) -> nom::IResult<I, O2, E>
where
    F: Parser<I, O1, E>,
    O2: Clone,
{
    nom::combinator::map(parser, move |_| value.clone())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_board() {
        let c = |d| Cell {
            pointer: Pointer::Go(d),
            number: None,
        };
        let cn = |d, n| Cell {
            pointer: Pointer::Go(d),
            number: Some(n),
        };

        let actual = parse_board::<(&str, ErrorKind)>("e1,e,s,w3\ns,s12,w5,w\nse,w,e,n\ne,e,n,*16");
        let expected = Ok(Game::example().board);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_row() {
        let c = |d| Cell {
            pointer: Pointer::Go(d),
            number: None,
        };
        let cn = |d, n| Cell {
            pointer: Pointer::Go(d),
            number: Some(n),
        };
        let mut parser = row;

        assert_eq!(
            parser.parse("n,w"),
            Ok(("", vec![c(Direction::North), c(Direction::West)]))
        );
        assert_eq!(parser.parse(""), err("", ErrorKind::Tag));
    }

    #[test]
    fn test_cell() {
        let c = |d| Cell {
            pointer: Pointer::Go(d),
            number: None,
        };
        let cn = |d, n| Cell {
            pointer: Pointer::Go(d),
            number: Some(n),
        };
        let mut parser = cell;
        assert_eq!(parser.parse("n"), Ok(("", c(Direction::North))));
        assert_eq!(parser.parse("ne"), Ok(("", c(Direction::Northeast))));
        assert_eq!(parser.parse("n4"), Ok(("", cn(Direction::North, 4))));
        assert_eq!(parser.parse("sw5"), Ok(("", cn(Direction::Southwest, 5))));
        assert_eq!(parser.parse("w15"), Ok(("", cn(Direction::West, 15))));
        assert_eq!(
            parser.parse("w15, n7"),
            Ok((", n7", cn(Direction::West, 15)))
        );
        assert_eq!(parser.parse(""), err("", ErrorKind::Tag));
    }

    #[test]
    fn test_pointer() {
        let mut parser = pointer;
        assert_eq!(parser.parse("n"), Ok(("", Pointer::Go(Direction::North))));
        assert_eq!(
            parser.parse("ne"),
            Ok(("", Pointer::Go(Direction::Northeast)))
        );
        assert_eq!(
            parser.parse("ne3"),
            Ok(("3", Pointer::Go(Direction::Northeast)))
        );
        assert_eq!(
            parser.parse("sw3"),
            Ok(("3", Pointer::Go(Direction::Southwest)))
        );
        assert_eq!(
            parser.parse("ss3"),
            Ok(("s3", Pointer::Go(Direction::South)))
        );
        assert_eq!(parser.parse("*"), Ok(("", Pointer::Final)));
        assert_eq!(parser.parse(" n"), err(" n", ErrorKind::Tag));
        assert_eq!(parser.parse(" *"), err(" *", ErrorKind::Tag));
    }

    #[test]
    fn test_dir() {
        let mut parser = dir;
        assert_eq!(parser.parse("n"), Ok(("", Direction::North)));
        assert_eq!(parser.parse("ne"), Ok(("", Direction::Northeast)));
        assert_eq!(parser.parse("ne3"), Ok(("3", Direction::Northeast)));
        assert_eq!(parser.parse("sw3"), Ok(("3", Direction::Southwest)));
        assert_eq!(parser.parse("ss3"), Ok(("s3", Direction::South)));
        assert_eq!(parser.parse(" n"), err(" n", ErrorKind::Tag));
    }

    #[test]
    fn test_map_tags() {
        let mut parser = map_tags(vec![("foo", 1), ("bar", 2), ("baz", 42)]);

        assert_eq!(parser.parse("foo"), Ok(("", 1)));
        assert_eq!(parser.parse("bar123"), Ok(("123", 2)));
        assert_eq!(parser.parse("bazasdf"), Ok(("asdf", 42)));
        assert_eq!(parser.parse(" foo"), err(" foo", ErrorKind::Tag));
        assert_eq!(parser.parse("bafoo"), err("bafoo", ErrorKind::Tag));
    }

    #[test]
    fn test_map_to() {
        let mut parser = map_to(tag("x"), 42);
        assert_eq!(parser.parse("x"), Ok(("", 42)));
        assert_eq!(parser.parse("xyz"), Ok(("yz", 42)));
        assert_eq!(parser.parse(" x"), err(" x", ErrorKind::Tag));
        assert_eq!(parser.parse(" xyz"), err(" xyz", ErrorKind::Tag));
    }

    fn err<O>(
        remaining: I,
        kind: nom::error::ErrorKind,
    ) -> Result<O, nom::Err<(I, nom::error::ErrorKind)>> {
        Err(nom::Err::Error((remaining, kind)))
    }
}
