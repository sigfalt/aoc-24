use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::hash_map::Entry;
use ahash::{AHashMap, AHashSet};
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

	pub const fn perpendicular(&self) -> [Direction; 2] {
		match self {
			Direction::North | Direction::South => [
				Direction::East, Direction::West
			],
			Direction::East | Direction::West => [
				Direction::North, Direction::South
			]
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct SearchNode {
	est_cost: u64,
	real_cost: u64,

	pos: (isize, isize),
	facing: Direction
}
impl PartialOrd for SearchNode {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		// inverted, lower weight is better
		Some(self.cmp(other))
	}
}
impl Ord for SearchNode {
	fn cmp(&self, other: &Self) -> Ordering {
		// inverted, lower weight is better
		other.est_cost.cmp(&self.est_cost)
	}
}

pub fn part1(input: &str) -> Result<u64> {
	let map: Grid<_> = parse(input).into();
	let (start_row, start_col) = map.indexed_iter().find_map(|((row, col), &cell)| {
		if cell == MapCell::Start {
			Some((row as isize, col as isize))
		} else {
			None
		}
	}).unwrap();
	let (end_row, end_col) = map.indexed_iter().find_map(|((row, col), &cell)| {
		if cell == MapCell::End {
			Some((row as isize, col as isize))
		} else {
			None
		}
	}).unwrap();

	fn taxicab_distance((curr_row, curr_col): (isize, isize), (goal_row, goal_col): (isize, isize)) -> u64 {
		(curr_row.abs_diff(goal_row) + curr_col.abs_diff(goal_col)) as u64
	}

	let mut queue = BinaryHeap::new();
	let mut weights = AHashMap::new();
	weights.insert(((start_row, start_col), Direction::East), 0);
	queue.push(SearchNode {
		est_cost: taxicab_distance((start_row, start_col), (end_row, end_col)),
		real_cost: 0,
		pos: (start_row, start_col),
		facing: Direction::East,
	});

	while let Some(SearchNode { real_cost, pos: curr_pos, facing: curr_dir, .. }) = queue.pop() {
		if curr_pos == (end_row, end_col) {
			return Ok(real_cost);
		}
		// may insert same node multiple times, skip if already seen with lower cost
		if let Some(&prev_cost) = weights.get(&(curr_pos, curr_dir)) {
			if real_cost > prev_cost {
				continue;
			}
		}
		let successors = {
			// can either step forward or turn each direction
			let (forward_row, forward_col) = curr_dir.offset_from(curr_pos).unwrap();
			let forward_move = (map.get(forward_row, forward_col).unwrap() != &MapCell::Wall)
				.then_some([(((forward_row, forward_col), curr_dir), real_cost + 1)]).into_iter().flatten();

			let turn_moves = curr_dir.perpendicular().into_iter().map(|new_dir|
				((curr_pos, new_dir), real_cost + 1000));
			forward_move.chain(turn_moves)
		};
		let _ = successors.into_iter().for_each(|((next_pos, next_dir), next_cost)| {
			match weights.entry((next_pos, next_dir)) {
				Entry::Vacant(e) => {
					e.insert(next_cost);
					queue.push(SearchNode {
						est_cost: next_cost + taxicab_distance(next_pos, (end_row, end_col)),
						real_cost: next_cost,
						pos: next_pos,
						facing: next_dir,
					});
				},
				Entry::Occupied(mut e) => {
					if next_cost < *e.get() {
						e.insert(next_cost);
						queue.push(SearchNode {
							est_cost: next_cost + taxicab_distance(next_pos, (end_row, end_col)),
							real_cost: next_cost,
							pos: next_pos,
							facing: next_dir,
						});
					}
				}
			};
		});
	}

	bail!("No path found?!")
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct SearchNodePartTwo {
	est_cost: u64,
	real_cost: u64,
	path: AHashSet<(isize, isize)>,
	pos: (isize, isize),
	facing: Direction
}
impl PartialOrd for SearchNodePartTwo {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		// inverted, lower weight is better
		Some(self.cmp(other))
	}
}
impl Ord for SearchNodePartTwo {
	fn cmp(&self, other: &Self) -> Ordering {
		// inverted, lower weight is better
		other.est_cost.cmp(&self.est_cost)
	}
}

pub fn part2(input: &str) -> Result<u64> {
	let map: Grid<_> = parse(input).into();
	let start_pos = map.indexed_iter().find_map(|((row, col), &cell)| {
		if cell == MapCell::Start {
			Some((row as isize, col as isize))
		} else {
			None
		}
	}).unwrap();
	let (end_row, end_col) = map.indexed_iter().find_map(|((row, col), &cell)| {
		if cell == MapCell::End {
			Some((row as isize, col as isize))
		} else {
			None
		}
	}).unwrap();

	let mut min_cost = None;
	let mut found_paths = Vec::new();
	let taxicab_distance = |(curr_row, curr_col): (isize, isize)| {
		(curr_row.abs_diff(end_row) + curr_col.abs_diff(end_col)) as u64
	};

	let mut queue = BinaryHeap::new();
	let mut weights = AHashMap::new();
	weights.insert((start_pos, Direction::East), 0);
	queue.push(SearchNodePartTwo {
		est_cost: taxicab_distance(start_pos),
		real_cost: 0,
		path: AHashSet::from([start_pos]),
		pos: start_pos,
		facing: Direction::East,
	});

	while let Some(SearchNodePartTwo { real_cost,  pos: curr_pos, facing: curr_dir, path: curr_path, .. }) = queue.pop() {
		if let Some(min_cost) = min_cost {
			if min_cost < real_cost {
				break;
			}
		}
		if curr_pos == (end_row, end_col) {
			min_cost = Some(real_cost);
			found_paths.push(curr_path.clone());
		}
		// may insert same node multiple times, skip if already seen with lower cost
		if let Some(&prev_cost) = weights.get(&(curr_pos, curr_dir)) {
			if real_cost > prev_cost {
				continue;
			}
		}
		let successors = {
			// can either step forward or turn each direction
			let (forward_row, forward_col) = curr_dir.offset_from(curr_pos).unwrap();
			let forward_move = (map.get(forward_row, forward_col).unwrap() != &MapCell::Wall)
				.then_some([(((forward_row, forward_col), curr_dir), real_cost + 1)]).into_iter().flatten();

			let turn_moves = curr_dir.perpendicular().into_iter().map(|new_dir|
				((curr_pos, new_dir), real_cost + 1000));
			forward_move.chain(turn_moves)
		};
		let _ = successors.into_iter().for_each(|((next_pos, next_dir), next_cost)| {
			match weights.entry((next_pos, next_dir)) {
				Entry::Vacant(e) => {
					e.insert(next_cost);
					let mut path = curr_path.clone();
					path.insert(next_pos);
					queue.push(SearchNodePartTwo {
						est_cost: next_cost + taxicab_distance(next_pos),
						real_cost: next_cost,
						path,
						pos: next_pos,
						facing: next_dir,
					});
				},
				Entry::Occupied(mut e) => {
					if next_cost <= *e.get() {
						e.insert(next_cost);
						let mut path = curr_path.clone();
						path.insert(next_pos);
						queue.push(SearchNodePartTwo {
							est_cost: next_cost + taxicab_distance(next_pos),
							real_cost: next_cost,
							path,
							pos: next_pos,
							facing: next_dir,
						});
					}
				}
			};
		});
	}

	let unique_cells = found_paths.into_iter().fold(AHashSet::new(), |unique_cells, path_tiles| {
		path_tiles.into_iter().fold(unique_cells, |mut unique_cells, pos| {
			unique_cells.insert(pos);
			unique_cells
		})
	});
	Ok(unique_cells.len() as u64)
}

#[cfg(test)]
mod tests {
	use crate::day16::*;

	const TEST_ONE: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

	const TEST_TWO: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

	#[test]
	fn test_one_part_one() -> Result<()> {
		assert_eq!(7036, part1(TEST_ONE)?);
		Ok(())
	}

	#[test]
	fn test_two_part_one() -> Result<()> {
		assert_eq!(11048, part1(TEST_TWO)?);
		Ok(())
	}

	#[test]
	fn test_one_part_two() -> Result<()> {
		assert_eq!(45, part2(TEST_ONE)?);
		Ok(())
	}

	#[test]
	fn test_two_part_two() -> Result<()> {
		assert_eq!(64, part2(TEST_TWO)?);
		Ok(())
	}
}
