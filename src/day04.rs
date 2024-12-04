use anyhow::*;
use grid::Grid;
use nom::branch::alt;
use nom::character::complete::{char, line_ending};
use nom::combinator::{all_consuming, opt, value};
use nom::{Finish, IResult};
use nom::multi::many1;
use nom::sequence::terminated;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Letter {
    X,
    M,
    A,
    S,
}

fn parse_letter(input: &str) -> IResult<&str, Letter> {
    alt((
        value(Letter::X, char('X')),
        value(Letter::M, char('M')),
        value(Letter::A, char('A')),
        value(Letter::S, char('S')),
    ))(input)
}

fn parse_row(input: &str) -> IResult<&str, Vec<Letter>> {
    terminated(
        many1(parse_letter),
        opt(line_ending)
    )(input)
}

fn parse(input: &str) -> Result<Vec<Vec<Letter>>> {
    let (_, result) = all_consuming(many1(parse_row))(input).finish().unwrap();
    Ok(result)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    North,
    Northeast,
    East,
    Southeast,
    South,
    Southwest,
    West,
    Northwest
}
impl Direction {
    pub fn get_offset(&self) -> (isize, isize) {
        match self {
            Direction::North => (0, -1),
            Direction::Northeast => (1, -1),
            Direction::East => (1, 0),
            Direction::Southeast => (1, 1),
            Direction::South => (0, 1),
            Direction::Southwest => (-1, 1),
            Direction::West => (-1, 0),
            Direction::Northwest => (-1, -1),
        }
    }
    pub fn iter() -> impl Iterator<Item = Direction> {
        [
            Direction::North,
            Direction::Northeast,
            Direction::East,
            Direction::Southeast,
            Direction::South,
            Direction::Southwest,
            Direction::West,
            Direction::Northwest
        ].into_iter()
    }
}

pub fn part1(input: &str) -> Result<u64> {
    let wordsearch: Grid<_> = parse(input)?.into();

    const TARGET: &[Letter; 4] = &[Letter::X, Letter::M, Letter::A, Letter::S];
    const TARGET_TUPLE: (&Letter, &[Letter]) = TARGET.split_first().unwrap();
    const TARGET_START: &Letter = TARGET_TUPLE.0;
    const TARGET_REST: &[Letter] = TARGET_TUPLE.1;

    let result = wordsearch.indexed_iter().map(|((x, y), letter)| {
        let (x, y) = (isize::try_from(x).unwrap(), isize::try_from(y).unwrap());
        if letter == TARGET_START {
            Direction::iter().map(|dir| {
                let (x_offset, y_offset) = dir.get_offset();
                let found_target = TARGET_REST.into_iter().enumerate().all(|(ix, target_letter)| {
                    let ix = isize::try_from(ix).unwrap();
                    let (next_x, next_y) = (x + (x_offset * (ix + 1)), y + (y_offset * (ix + 1)));
                    wordsearch.get(next_x, next_y).is_some_and(|grid_letter| grid_letter == target_letter)
                });

                if found_target { 1 } else { 0 }
            }).sum()
        } else {
            0
        }
    }).sum();

    Ok(result)
}

pub fn part2(input: &str) -> Result<u64> {
    let _ = input;
    Ok(0)
}

#[cfg(test)]
mod tests {
    use crate::day04::*;

    const TEST: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(18, part1(TEST)?);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        assert_eq!(0, part2(TEST)?);
        Ok(())
    }
}
