use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::hash_map::Entry;
use std::fmt::{Display, Formatter};
use std::ops::ControlFlow;
use ahash::AHashMap;
use anyhow::*;
use grid::Grid;
use nom::character::complete::{char, digit1, line_ending};
use nom::combinator::{all_consuming, map, map_res};
use nom::{Finish, IResult};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BytePos {
	row: usize,
	col: usize
}
impl Display for BytePos {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{},{}", self.row, self.col)
	}
}
impl From<(usize, usize)> for BytePos {
	fn from((row, col): (usize, usize)) -> Self {
		Self { row, col }
	}
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
	map_res(digit1, |num: &str| num.parse())(input)
}

fn parse_pos(input: &str) -> IResult<&str, BytePos> {
	map(separated_pair(
		parse_usize,
		char(','),
		parse_usize
	), |(row, col)| BytePos { row, col })(input)
}

fn parse(input: &str) -> Vec<BytePos> {
	let (_, result) = all_consuming(separated_list1(line_ending, parse_pos))(input).finish().unwrap();
	result
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum MapCell {
	Empty,
	Corrupted,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct SearchNode {
	est_cost: u64,
	real_cost: u64,

	pos: (isize, isize),
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
	let bytes = parse(input);

	part1_steps_req((71, 71), bytes.into_iter().take(1024))
}

fn part1_steps_req((grid_rows, grid_cols): (usize, usize), bytes: impl IntoIterator<Item = BytePos>) -> Result<u64> {
	let mut grid = Grid::init(grid_rows, grid_cols, MapCell::Empty);
	bytes.into_iter().for_each(|byte_pos| {
		let BytePos { row: byte_row, col: byte_col } = byte_pos;
		if let Some(grid_cell) = grid.get_mut(byte_row, byte_col) {
			*grid_cell = MapCell::Corrupted;
		}
	});

	let (end_row, end_col) = (grid_rows as isize - 1, grid_cols as isize - 1);
	let taxicab_distance = |(curr_row, curr_col): (isize, isize)| {
		(curr_row.abs_diff(end_row) + curr_col.abs_diff(end_col)) as u64
	};

	let mut queue = BinaryHeap::new();
	let mut weights = AHashMap::new();
	weights.insert((0, 0), 0);
	queue.push(SearchNode {
		est_cost: taxicab_distance((0, 0)),
		real_cost: 0,
		pos: (0, 0),
	});

	while let Some(SearchNode { real_cost, pos: curr_pos, .. }) = queue.pop() {
		if curr_pos == (end_row, end_col) {
			return Ok(real_cost);
		}
		// may insert same node multiple times, skip if already seen with lower cost
		if let Some(&prev_cost) = weights.get(&curr_pos) {
			if real_cost > prev_cost {
				continue;
			}
		}
		let successors = {
			// can move in any direction that is not occupied by a corrupted byte
			Direction::values().into_iter().filter_map(|dir| {
				let (next_row, next_col) = dir.offset_from(curr_pos).unwrap();
				if let Some(&next_cell) = grid.get(next_row, next_col) {
					if next_cell != MapCell::Corrupted {
						Some(((next_row, next_col), real_cost + 1))
					} else {
						None
					}
				} else {
					None
				}
			})
		};
		let _ = successors.into_iter().for_each(|(next_pos, next_cost)| {
			match weights.entry(next_pos) {
				Entry::Vacant(e) => {
					e.insert(next_cost);
					queue.push(SearchNode {
						est_cost: next_cost + taxicab_distance(next_pos),
						real_cost: next_cost,
						pos: next_pos,
					});
				},
				Entry::Occupied(mut e) => {
					if next_cost < *e.get() {
						e.insert(next_cost);
						queue.push(SearchNode {
							est_cost: next_cost + taxicab_distance(next_pos),
							real_cost: next_cost,
							pos: next_pos,
						});
					}
				}
			};
		});
	}

	bail!("No path found?!")
}

pub fn part2(input: &str) -> Result<BytePos> {
	let bytes = parse(input);

	part2_blocking_byte((71, 71), bytes)
}

fn part2_blocking_byte((grid_rows, grid_cols): (usize, usize), bytes: impl IntoIterator<Item = BytePos>) -> Result<BytePos> {
	let mut grid = Grid::init(grid_rows, grid_cols, MapCell::Empty);

	let (end_row, end_col) = (grid_rows as isize - 1, grid_cols as isize - 1);
	let taxicab_distance = |(curr_row, curr_col): (isize, isize)| {
		(curr_row.abs_diff(end_row) + curr_col.abs_diff(end_col)) as u64
	};

	let first_blocking_byte = bytes.into_iter().try_for_each(|next_byte| {
		let BytePos { row: byte_row, col: byte_col } = next_byte;
		if let Some(grid_cell) = grid.get_mut(byte_row, byte_col) {
			*grid_cell = MapCell::Corrupted;
		}

		let mut queue = BinaryHeap::new();
		let mut weights = AHashMap::new();
		weights.insert((0, 0), 0);
		queue.push(SearchNode {
			est_cost: taxicab_distance((0, 0)),
			real_cost: 0,
			pos: (0, 0),
		});

		let mut path_found = false;
		while let Some(SearchNode { real_cost, pos: curr_pos, .. }) = queue.pop() {
			if curr_pos == (end_row, end_col) {
				path_found = true;
				break;
			}
			// may insert same node multiple times, skip if already seen with lower cost
			if let Some(&prev_cost) = weights.get(&curr_pos) {
				if real_cost > prev_cost {
					continue;
				}
			}
			let successors = {
				// can move in any direction that is not occupied by a corrupted byte
				Direction::values().into_iter().filter_map(|dir| {
					let (next_row, next_col) = dir.offset_from(curr_pos).unwrap();
					if let Some(&next_cell) = grid.get(next_row, next_col) {
						if next_cell != MapCell::Corrupted {
							Some(((next_row, next_col), real_cost + 1))
						} else {
							None
						}
					} else {
						None
					}
				})
			};
			let _ = successors.into_iter().for_each(|(next_pos, next_cost)| {
				match weights.entry(next_pos) {
					Entry::Vacant(e) => {
						e.insert(next_cost);
						queue.push(SearchNode {
							est_cost: next_cost + taxicab_distance(next_pos),
							real_cost: next_cost,
							pos: next_pos,
						});
					},
					Entry::Occupied(mut e) => {
						if next_cost < *e.get() {
							e.insert(next_cost);
							queue.push(SearchNode {
								est_cost: next_cost + taxicab_distance(next_pos),
								real_cost: next_cost,
								pos: next_pos,
							});
						}
					}
				};
			});
		}

		if path_found {
			ControlFlow::Continue(())
		} else {
			ControlFlow::Break(next_byte)
		}
	});

	Ok(first_blocking_byte.break_value().unwrap())
}

#[cfg(test)]
mod tests {
	use crate::day18::*;

	const TEST: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

	#[test]
	fn test_part_one() -> Result<()> {
		assert_eq!(22, part1_steps_req((7, 7), parse(TEST).into_iter().take(12))?);
		Ok(())
	}

	#[test]
	fn test_part_two() -> Result<()> {
		assert_eq!(BytePos::from((6, 1)), part2_blocking_byte((7, 7), parse(TEST).into_iter())?);
		Ok(())
	}
}
