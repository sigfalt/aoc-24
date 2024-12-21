use anyhow::*;
use grid::Grid;
use nom::character::complete::{line_ending, one_of};
use nom::combinator::{all_consuming, map_res, opt};
use nom::{Finish, IResult};
use nom::multi::many1;
use nom::sequence::terminated;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MapCell {
	Start,
	End,
	Wall,
	Empty,
}

fn parse_cell(input: &str) -> IResult<&str, MapCell> {
	map_res(one_of("SE#."), |chr| {match chr {
		'S' => Ok(MapCell::Start),
		'E' => Ok(MapCell::End),
		'#' => Ok(MapCell::Wall),
		'.' => Ok(MapCell::Empty),
		_ => bail!("parse_cell consumed unexpected character")
	}})(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<MapCell>> {
	terminated(
		many1(parse_cell),
		opt(line_ending)
	)(input)
}

fn parse(input: &str) -> Vec<Vec<MapCell>> {
	let (_, result) = all_consuming(many1(parse_line))(input).finish().unwrap();
	result
}

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

	pub const fn opposite(&self) -> Direction {
		match self {
			Direction::North => Direction::South,
			Direction::East => Direction::West,
			Direction::South => Direction::North,
			Direction::West => Direction::East,
		}
	}
}

pub fn part1(input: &str) -> Result<usize> {
	part1_with_savings(input, 100)
}

pub fn part1_with_savings(input: &str, min_savings: usize) -> Result<usize> {
	let map: Grid<_> = parse(input).into();
	let start_pos = map.indexed_iter().find_map(|((row, col), &cell)| {
		if cell == MapCell::Start {
			Some((row as isize, col as isize))
		} else {
			None
		}
	}).unwrap();

	// map is supposed to represent a racecourse
	// assume map only has one path from start to finish, with no branches/dead ends
	// starting from the start and simply not proceeding backwards should find the path from start to end
	let mut course = Vec::from([start_pos]);
	let mut backwards = None;
	let mut curr_pos = start_pos;
	while map.get(curr_pos.0, curr_pos.1).unwrap() != &MapCell::End {
		let mut to_check = Direction::values().into_iter().filter_map(|dir| {
			if let Some(backwards) = backwards {
				if backwards != dir {
					Some((dir, dir.offset_from(curr_pos).unwrap()))
				} else {
					None
				}
			} else {
				Some((dir, dir.offset_from(curr_pos).unwrap()))
			}
		});
		let (next_dir, next_pos) = to_check.find(|&(_, (row_to_check, col_to_check))|
			map.get(row_to_check, col_to_check).unwrap() != &MapCell::Wall
		).unwrap();

		course.push(next_pos);
		backwards = Some(next_dir.opposite());
		curr_pos = next_pos;
	}

	let mut time_map = Grid::init(map.rows(), map.cols(), None);
	course.iter().enumerate().for_each(|(time, &(row, col))| {
		*time_map.get_mut(row, col).unwrap() = Some(time);
	});

	let all_skip_lens = course.iter().flat_map(|&(row, col)| {
		let start_time = time_map.get(row, col).cloned().flatten().unwrap();

		// for each point along the course, check two cells in each direction to see if there is
		// a later point in the race we can reach
		let possible_skips_to = Direction::values().into_iter().filter_map(move |dir|
			dir.offset_from((row, col)).and_then(|(once_offset_row, once_offset_col)|
				dir.offset_from((once_offset_row, once_offset_col)))
		).filter_map(|(offset_row, offset_col)|
			time_map.get(offset_row, offset_col).cloned().flatten()
		);

		// check that we do save time (we can be trying to skip backwards), but assuming we do,
		// the time saved by the skip is the difference in the start and end points,
		// minus the 2 picoseconds spent traversing the skip
		possible_skips_to.filter_map(move |skip_end_time| {
			if skip_end_time > start_time + 2 {
				Some(skip_end_time - start_time - 2)
			} else {
				None
			}
		})
	});

	Ok(all_skip_lens.filter(|&skip_savings| skip_savings >= min_savings).count())
}

pub fn part2(input: &str) -> Result<u64> {
	let _ = input;
	Ok(0)
}

#[cfg(test)]
mod tests {
	use crate::day20::*;

	const TEST: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

	#[test]
	fn test_part_one() -> Result<()> {
		assert_eq!(5, part1_with_savings(TEST, 20)?);
		Ok(())
	}

	#[test]
	fn test_part_two() -> Result<()> {
		assert_eq!(0, part2(TEST)?);
		Ok(())
	}
}
