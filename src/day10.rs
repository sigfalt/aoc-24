use std::collections::VecDeque;
use ahash::AHashSet;
use anyhow::*;
use grid::Grid;
use nom::bytes::complete::take;
use nom::character::complete::line_ending;
use nom::combinator::{all_consuming, map_res, opt};
use nom::{Finish, IResult};
use nom::multi::many1;
use nom::sequence::terminated;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    pub const fn get_offset(&self) -> (isize, isize) {
        match self {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
        }
    }

    pub fn offset_from(&self, (row, col): (impl TryInto<isize>, impl TryInto<isize>)) -> Option<(isize, isize)> {
        let row_isize = row.try_into().ok()?;
        let col_isize = col.try_into().ok()?;
        let (row_offset, col_offset) = self.get_offset();
        Some((row_isize + row_offset, col_isize + col_offset))
    }

    pub const fn values() -> [Direction; 4] {
        [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West
        ]
    }
}

fn parse_digit(input: &str) -> IResult<&str, u64> {
    map_res(take(1u8), |dgt| u64::from_str_radix(dgt, 10))(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<u64>> {
    terminated(many1(parse_digit), opt(line_ending))(input)
}

fn parse(input: &str) -> Vec<Vec<u64>> {
    let (_, result) = all_consuming(many1(parse_line))(input).finish().unwrap();
    result
}

pub fn part1(input: &str) -> Result<u64> {
    let topographical_map: Grid<_> = parse(input).into();

    let mut score_map: Grid<u64> = {
        let (grid_rows, grid_cols) = topographical_map.size();
        Grid::init(grid_rows, grid_cols, 0)
    };
    topographical_map.indexed_iter().filter_map(|(coords, &height)| {
        if height == 9 { Some(coords) } else { None }
    }).for_each(|(row, col)| {
        let mut nodes_to_process = VecDeque::new();
        nodes_to_process.push_back((row as isize, col as isize));
        let mut can_reach_this_peak = AHashSet::new();

        while let Some((curr_row, curr_col)) = nodes_to_process.pop_front() {
            can_reach_this_peak.insert((curr_row, curr_col));

            if let Some(&height) = topographical_map.get(curr_row, curr_col) {
                if height > 0 {
                    let neighbor_coords = Direction::values().into_iter()
                        .flat_map(|dir| dir.offset_from((curr_row, curr_col)));
                    let filtered_neighbors = neighbor_coords.filter(|&(nbr_row, nbr_col)| {
                        topographical_map.get(nbr_row, nbr_col).is_some_and(|&nbr_height| nbr_height == height - 1)
                    });
                    nodes_to_process.extend(filtered_neighbors);
                }
            }
        }

        can_reach_this_peak.iter().for_each(|&(row_to_update, col_to_update)| {
            if let Some(curr_score) = score_map.get_mut(row_to_update, col_to_update) {
                *curr_score += 1;
            }
        })
    });

    let score_sum = topographical_map.indexed_iter().filter_map(|(coords, &height)| {
        if height == 0 { Some(coords) } else { None }
    }).flat_map(|(row, col)| score_map.get(row, col)).sum();

    Ok(score_sum)
}

pub fn part2(input: &str) -> Result<u64> {
    let topographical_map: Grid<_> = parse(input).into();

    let mut rating_map: Grid<Option<u64>> = {
        let (grid_rows, grid_cols) = topographical_map.size();
        Grid::init(grid_rows, grid_cols, None)
    };

    let calculate_rating = |(row, col): (usize, usize)| -> Option<u64> {
        fn rec((row, col): (isize, isize), topographical_map: &Grid<u64>, rating_map: &mut Grid<Option<u64>>) -> Option<u64> {
            if let Some(rating) = rating_map.get(row, col).copied().flatten() {
                return Some(rating);
            }

            let calculated_rating = if let Some(&height) = topographical_map.get(row, col) {
                if height == 9 {
                    Some(1)
                } else {
                    let neighbor_coords = Direction::values().into_iter()
                        .flat_map(|dir| dir.offset_from((row, col)));
                    let filtered_neighbors = neighbor_coords.filter(|&(nbr_row, nbr_col)| {
                        topographical_map.get(nbr_row, nbr_col).is_some_and(|&nbr_height| nbr_height == height + 1)
                    });

                    let rating = filtered_neighbors.flat_map(|nbr_coords| rec(nbr_coords, topographical_map, rating_map)).sum();
                    Some(rating)
                }
            } else {
                None
            };

            if calculated_rating.is_some() {
                *rating_map.get_mut(row, col).unwrap() = calculated_rating;
            }
            calculated_rating
        }
        rec((row.try_into().ok()?, col.try_into().ok()?), &topographical_map, &mut rating_map)
    };

    let rating_sum = topographical_map.indexed_iter().filter_map(|(coords, &height)| {
        if height == 0 { Some(coords) } else { None }
    }).flat_map(calculate_rating).sum();
    Ok(rating_sum)
}

#[cfg(test)]
mod tests {
    use crate::day10::*;

    const TEST: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(36, part1(TEST)?);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        assert_eq!(81, part2(TEST)?);
        Ok(())
    }
}
