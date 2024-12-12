use ahash::AHashSet;
use anyhow::*;
use grid::Grid;
use nom::branch::alt;
use nom::character::complete::{char, line_ending};
use nom::combinator::{all_consuming, opt, value};
use nom::IResult;
use nom::multi::many1;
use nom::sequence::terminated;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MapCell {
	Empty,
	Obstacle,
	Guard(Direction),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
	North,
	East,
	South,
	West,
}
impl Direction {
	pub fn get_offset(&self) -> (isize, isize) {
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

	pub fn rotate(&self) -> Self {
		match self {
			Direction::North => Direction::East,
			Direction::East => Direction::South,
			Direction::South => Direction::West,
			Direction::West => Direction::North,
		}
	}
}

fn parse_cell(input: &str) -> IResult<&str, MapCell> {
	alt((
		value(MapCell::Empty, char('.')),
		value(MapCell::Obstacle, char('#')),
		value(MapCell::Guard(Direction::North), char('^')),
		value(MapCell::Guard(Direction::East), char('>')),
		value(MapCell::Guard(Direction::South), char('v')),
		value(MapCell::Guard(Direction::West), char('<')),
	))(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<MapCell>> {
	terminated(
		many1(parse_cell),
		opt(line_ending)
	)(input)
}

fn parse(input: &str) -> Grid<MapCell> {
	let (_, result) = all_consuming(many1(parse_line))(input).unwrap();
	result.into()
}

pub fn part1(input: &str) -> Result<u64> {
	let map = parse(input);

	let (mut guard_position, mut guard_direction) = map.indexed_iter().find_map(|((x, y), &cell)| {
		if let MapCell::Guard(dir) = cell {
			Some(((isize::try_from(x).unwrap(), isize::try_from(y).unwrap()), dir))
		} else {
			None
		}
	}).unwrap();

	let mut visited_positions = AHashSet::new();
	visited_positions.insert(guard_position);
	let (mut next_row, mut next_col) = guard_direction.offset_from(guard_position).unwrap();

	while let Some(&next_cell) = map.get(next_row, next_col) {
		// don't ever move or clear original guard cell, so only check obstacle vs non-obstacle
		if next_cell == MapCell::Obstacle {
			guard_direction = guard_direction.rotate();
		} else {
			guard_position = (next_row, next_col);
			visited_positions.insert(guard_position);
		}

		(next_row, next_col) = guard_direction.offset_from(guard_position).unwrap();
	}

	Ok(visited_positions.len() as u64)
}

pub fn part2(input: &str) -> Result<u64> {
	let map = parse(input);

	let (mut guard_position, mut guard_direction) = map.indexed_iter().find_map(|((x, y), &cell)| {
		if let MapCell::Guard(dir) = cell {
			Some(((isize::try_from(x).unwrap(), isize::try_from(y).unwrap()), dir))
		} else {
			None
		}
	}).unwrap();

	let mut looping_positions = 0;
	let mut added_obstacle_positions = AHashSet::new();
	fn detect_loop(map: Grid<MapCell>, mut curr_pos: (isize, isize), mut curr_dir: Direction, mut visited_states: AHashSet<((isize, isize), Direction)>) -> bool {
		let (mut next_row, mut next_col) = curr_dir.offset_from(curr_pos).unwrap();
		visited_states.insert((curr_pos, curr_dir));

		while let Some(&next_cell) = map.get(next_row, next_col) {
			// don't ever move or clear original guard cell, so only check obstacle vs non-obstacle
			if next_cell == MapCell::Obstacle {
				curr_dir = curr_dir.rotate();
			} else {
				curr_pos = (next_row, next_col);
			}

			let looping = !visited_states.insert((curr_pos, curr_dir));
			if looping { return true; }

			(next_row, next_col) = curr_dir.offset_from(curr_pos).unwrap();
		}
		false
	}

	let mut visited_states = AHashSet::new();
	visited_states.insert((guard_position, guard_direction));
	let (mut next_row, mut next_col) = guard_direction.offset_from(guard_position).unwrap();
	while let Some(&next_cell) = map.get(next_row, next_col) {
		// don't ever move or clear original guard cell, so only check obstacle vs non-obstacle
		if next_cell == MapCell::Obstacle {
			guard_direction = guard_direction.rotate();
		} else {
			// if the next cell is empty, and we haven't tried putting an obstacle there yet, try that
			if !added_obstacle_positions.contains(&(next_row, next_col)) && next_cell == MapCell::Empty {
				added_obstacle_positions.insert((next_row, next_col));
				let mut modified_map = map.clone();
				*modified_map.get_mut(next_row, next_col).unwrap() = MapCell::Obstacle;
				if detect_loop(modified_map, guard_position, guard_direction, visited_states.clone()) {
					looping_positions += 1;
				}
			}
			guard_position = (next_row, next_col);
		}
		visited_states.insert((guard_position, guard_direction));

		(next_row, next_col) = guard_direction.offset_from(guard_position).unwrap();
	}

	Ok(looping_positions)
}

#[cfg(test)]
mod tests {
	use crate::day06::*;

	const TEST: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

	#[test]
	fn test_part_one() -> Result<()> {
		assert_eq!(41, part1(TEST)?);
		Ok(())
	}

	#[test]
	fn test_part_two() -> Result<()> {
		assert_eq!(6, part2(TEST)?);
		Ok(())
	}
}
