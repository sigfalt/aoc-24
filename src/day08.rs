use ahash::{AHashMap, AHashSet};
use anyhow::*;
use grid::Grid;
use itertools::Itertools;
use nom::character::complete::{anychar, line_ending};
use nom::combinator::{all_consuming, map_res, opt};
use nom::{Finish, IResult};
use nom::multi::many1;
use nom::sequence::terminated;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MapCell {
    Empty,
    Antenna { frequency: char },
}
impl TryFrom<char> for MapCell {
    type Error = &'static str;
    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        if value.is_ascii_alphanumeric() {
            Result::Ok(MapCell::Antenna { frequency: value })
        } else if value == '.' {
            Result::Ok(MapCell::Empty)
        } else {
            Err("")
        }
    }
}

fn parse_cell(input: &str) -> IResult<&str, MapCell> {
    map_res(
        anychar,
        |byte: char| { MapCell::try_from(byte) }
    )(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<MapCell>> {
    terminated(many1(parse_cell), opt(line_ending))(input)
}

fn parse(input: &str) -> Vec<Vec<MapCell>> {
    let (_, result) = all_consuming(many1(parse_line))(input).finish().unwrap();
    result
}

pub fn part1(input: &str) -> Result<u64> {
    let map: Grid<_> = parse(input).into();

    let mut frequency_coordinate_map = AHashMap::new();
    map.indexed_iter().for_each(|(coords, &cell)| {
        if let MapCell::Antenna { frequency } = cell {
            frequency_coordinate_map.entry(frequency)
                .or_insert(Vec::new())
                .push(coords);
        }
    });
    let frequency_coordinate_map = frequency_coordinate_map;

    let mut antinode_coords = AHashSet::new();
    frequency_coordinate_map.iter().for_each(|(_frequency, coordinate_list)| {
        coordinate_list.iter().combinations(2).for_each(|x| {
            let (&&(first_row, first_col), _) = x.split_first().unwrap();
            let (&&(second_row, second_col), _) = x.split_last().unwrap();
            let (first_row, first_col) = (isize::try_from(first_row).unwrap(), isize::try_from(first_col).unwrap());
            let (second_row, second_col) = (isize::try_from(second_row).unwrap(), isize::try_from(second_col).unwrap());
            let (first_antinode_row, first_antinode_col) = ((first_row - second_row) + first_row, (first_col - second_col) + first_col);
            let (second_antinode_row, second_antinode_col) = ((second_row - first_row) + second_row, (second_col - first_col) + second_col);
            if let Some(_) = map.get(first_antinode_row, first_antinode_col) {
                antinode_coords.insert((first_antinode_row, first_antinode_col));
            }
            if let Some(_) = map.get(second_antinode_row, second_antinode_col) {
                antinode_coords.insert((second_antinode_row, second_antinode_col));
            }
        })
    });

    Ok(antinode_coords.len() as u64)
}

pub fn part2(input: &str) -> Result<u64> {
    let _ = input;
    Ok(0)
}

#[cfg(test)]
mod tests {
    use crate::day08::*;

    const TEST: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(14, part1(TEST)?);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        assert_eq!(0, part2(TEST)?);
        Ok(())
    }
}
