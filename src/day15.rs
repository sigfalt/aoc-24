use anyhow::*;
use grid::Grid;
use nom::branch::alt;
use nom::character::complete::{char, line_ending};
use nom::combinator::{all_consuming, opt, value};
use nom::{Finish, IResult};
use nom::multi::{fold_many1, many1};
use nom::sequence::{separated_pair, terminated};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum MapCell {
	Robot,
	Box,
	Wall,
	Empty
}

fn parse_map_cell(input: &str) -> IResult<&str, MapCell> {
	alt((
		value(MapCell::Robot, char('@')),
		value(MapCell::Box, char('O')),
		value(MapCell::Wall, char('#')),
		value(MapCell::Empty, char('.'))
	))(input)
}

fn parse_map_row(input: &str) -> IResult<&str, Vec<MapCell>> {
	terminated(
		many1(parse_map_cell),
		line_ending
	)(input)
}

fn parse_map(input: &str) -> IResult<&str, Vec<Vec<MapCell>>> {
	many1(parse_map_row)(input)
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
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
	alt((
		value(Direction::North, char('^')),
		value(Direction::East, char('>')),
		value(Direction::South, char('v')),
		value(Direction::West, char('<'))
	))(input)
}

fn parse_direction_line(input: &str) -> IResult<&str, Vec<Direction>> {
	terminated(
		many1(parse_direction),
		opt(line_ending)
	)(input)
}

fn parse_directions(input: &str) -> IResult<&str, Vec<Direction>> {
	fold_many1(parse_direction_line, Vec::new, |mut acc, dirs| {
		acc.extend(dirs);
		acc
	})(input)
}

fn parse(input: &str) -> (Vec<Vec<MapCell>>, Vec<Direction>) {
	let (_, result) = all_consuming(separated_pair(
		parse_map,
		line_ending,
		parse_directions
	))(input).finish().unwrap();
	result
}

pub fn part1(input: &str) -> Result<u64> {
	let (map, directions) = parse(input);
	let mut map: Grid<_> = map.into();

	let (mut robot_row, mut robot_col) = map.indexed_iter().find_map(|(grid_coords, &grid_cell)| {
		if grid_cell == MapCell::Robot {
			Some(grid_coords)
		} else {
			None
		}
	}).unwrap();

	directions.into_iter().for_each(|direction| {
		let mut cell_changes = Vec::new();

		let mut curr_cell_coords = (robot_row as isize, robot_col as isize);
		let mut curr_cell = MapCell::Robot;
		loop {
			let (next_cell_row, next_cell_col) = direction.offset_from(curr_cell_coords).unwrap();
			let next_cell = *map.get(next_cell_row, next_cell_col).unwrap();

			if next_cell == MapCell::Wall {
				cell_changes.clear();
				break;
			} else {
				cell_changes.push(((next_cell_row, next_cell_col), curr_cell));
				if next_cell != MapCell::Box {
					break;
				}
			}

			curr_cell_coords = (next_cell_row, next_cell_col);
			curr_cell = next_cell;
		}
		cell_changes.into_iter().for_each(|((cell_row, cell_col), cell_new_contents)| {
			*map.get_mut(cell_row, cell_col).unwrap() = cell_new_contents;
			if cell_new_contents == MapCell::Robot {
				(robot_row, robot_col) = (cell_row as usize, cell_col as usize);
			}
		});
	});

	let gps_sum = map.indexed_iter().filter_map(|((cell_row, cell_col), &map_cell)| {
		if map_cell == MapCell::Box {
			Some((cell_col + (100 * cell_row)) as u64)
		} else {
			None
		}
	}).sum();

	Ok(gps_sum)
}

pub fn part2(input: &str) -> Result<u64> {
	let _ = input;
	Ok(0)
}

#[cfg(test)]
mod tests {
	use crate::day15::*;

	const SMALL_TEST: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

	const LARGE_TEST: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

	#[test]
	fn test_part_one_small() -> Result<()> {
		assert_eq!(2028, part1(SMALL_TEST)?);
		Ok(())
	}

	#[test]
	fn test_part_one_large() -> Result<()> {
		assert_eq!(10092, part1(LARGE_TEST)?);
		Ok(())
	}

	#[test]
	fn test_part_two() -> Result<()> {
		assert_eq!(0, part2(SMALL_TEST)?);
		Ok(())
	}
}
