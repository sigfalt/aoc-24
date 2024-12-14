use std::ops::{Add, ControlFlow, Mul};
use anyhow::*;
use grid::Grid;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, line_ending};
use nom::combinator::{all_consuming, map, map_res, opt, recognize};
use nom::{Finish, IResult};
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair};

fn parse_u64(input: &str) -> IResult<&str, u64> {
	map_res(digit1, |num: &str| num.parse())(input)
}

fn parse_i64(input: &str) -> IResult<&str, i64> {
	map_res(recognize(preceded(opt(char('-')), digit1)), |num: &str| num.parse())(input)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RobotInfo {
	pos: NormalizedPosition,
	vel: Velocity,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct NormalizedPosition {
	x: u64,
	y: u64,
}
impl Add<Self> for NormalizedPosition {
	type Output = Position;

	fn add(self, rhs: Self) -> Self::Output {
		Position::from(self) + rhs.into()
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Position {
	x: i64,
	y: i64,
}
impl From<NormalizedPosition> for Position {
	fn from(value: NormalizedPosition) -> Self {
		Self {
			x: value.x as i64,
			y: value.y as i64,
		}
	}
}
impl Add<Self> for Position {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Self::Output {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Velocity {
	delta_x: i64,
	delta_y: i64,
}
impl Mul<u64> for Velocity {
	type Output = Position;

	fn mul(self, rhs: u64) -> Self::Output {
		self * rhs as i64
	}
}
impl Mul<i64> for Velocity {
	type Output = Position;

	fn mul(self, rhs: i64) -> Self::Output {
		Self::Output {
			x: self.delta_x * rhs,
			y: self.delta_y * rhs
		}
	}
}

fn parse_position(input: &str) -> IResult<&str, NormalizedPosition> {
	map(separated_pair(
		preceded(tag("p="), parse_u64),
		char(','),
		parse_u64
	), |(x, y)| NormalizedPosition { x, y })(input)
}

fn parse_velocity(input: &str) -> IResult<&str, Velocity> {
	map(separated_pair(
		preceded(tag("v="), parse_i64),
		char(','),
		parse_i64
	), |(delta_x, delta_y)| Velocity { delta_x, delta_y })(input)
}

fn parse_robot(input: &str) -> IResult<&str, RobotInfo> {
	map(separated_pair(
		parse_position,
		char(' '),
		parse_velocity
	), |(pos, vel)| RobotInfo { pos, vel })(input)
}

fn parse(input: &str) -> Vec<RobotInfo> {
	let (_, result) = all_consuming(separated_list1(line_ending, parse_robot))(input).finish().unwrap();
	result
}

pub fn part1(input: &str) -> Result<u64> {
	const GRID_ROW_LEN: u64 = 101;
	const GRID_COL_LEN: u64 = 103;

	part1_with_grid_size(input, GRID_ROW_LEN, GRID_COL_LEN)
}

fn part1_with_grid_size(input: &str, grid_row_len: u64, grid_col_len: u64) -> Result<u64> {
	let robots = parse(input);

	const MAX_TIME: u64 = 100;

	let quadrants_product = robots.into_iter().filter_map(|robot| {
		let distance_travelled = robot.vel * MAX_TIME;
		let new_position = distance_travelled + robot.pos.into();
		let normalized_position = NormalizedPosition {
			x: new_position.x.rem_euclid(grid_row_len as i64) as u64,
			y: new_position.y.rem_euclid(grid_col_len as i64) as u64,
		};

		if normalized_position.x < grid_row_len / 2 && normalized_position.y < grid_col_len / 2 {
			Some(0)
		} else if normalized_position.x > grid_row_len / 2 && normalized_position.y < grid_col_len / 2 {
			Some(1)
		} else if normalized_position.x < grid_row_len / 2 && normalized_position.y > grid_col_len / 2 {
			Some(2)
		} else if normalized_position.x > grid_row_len / 2 && normalized_position.y > grid_col_len / 2 {
			Some(3)
		} else {
			None
		}
	}).fold([0, 0, 0, 0], |[quad0, quad1, quad2, quad3], b| {
		match b {
			0 => [quad0 + 1, quad1, quad2, quad3],
			1 => [quad0, quad1 + 1, quad2, quad3],
			2 => [quad0, quad1, quad2 + 1, quad3],
			3 => [quad0, quad1, quad2, quad3 + 1],
			_ => [quad0, quad1, quad2, quad3]
		}
	}).into_iter().product();

	Ok(quadrants_product)
}

pub fn part2(input: &str) -> Result<u64> {
	let robots = parse(input);
	const GRID_ROW_LEN: u64 = 101;
	const GRID_COL_LEN: u64 = 103;

	const RUN_LEN_REQUIRED: u64 = 10;

	let tree_time = (0..).try_for_each(|time: u64| {
		let mut grid: Grid<_> = Grid::init(GRID_COL_LEN as usize, GRID_ROW_LEN as usize, false);

		robots.iter().for_each(|robot| {
			let distance_travelled = robot.vel * time;
			let new_position = distance_travelled + robot.pos.into();
			let normalized_position = NormalizedPosition {
				x: new_position.x.rem_euclid(GRID_ROW_LEN as i64) as u64,
				y: new_position.y.rem_euclid(GRID_COL_LEN as i64) as u64,
			};
			*grid.get_mut(normalized_position.y, normalized_position.x).unwrap() = true;
		});

		
		let long_run_found = grid.iter_rows().map(|row| {
			let mut longest_run = 0;
			let mut curr_run = 0;
			row.for_each(|&cell| {
				if cell {
					curr_run += 1;
					if curr_run > longest_run {
						longest_run = curr_run;
					}
				} else {
					curr_run = 0;
				}
			});
			longest_run
		}).any(|run| run > RUN_LEN_REQUIRED);
		if long_run_found {
			// println!("time t={:?}", time);
			// grid.iter_rows().for_each(|row| {
			// 	println!("{}", row.map(|&cell| if cell { '█' } else { ' ' }).join(" "));
			// });
			// println!();

			ControlFlow::Break(time)
		} else {
			ControlFlow::Continue(())
		}
	});

	Ok(tree_time.break_value().unwrap())
}

#[cfg(test)]
mod tests {
	use crate::day14::*;

	const TEST: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

	const TEST_GRID_ROW_LEN: u64 = 7;
	const TEST_GRID_COL_LEN: u64 = 11;

	#[test]
	fn test_small_grid() -> Result<()> {
		assert_eq!(12, part1_with_grid_size(TEST, TEST_GRID_ROW_LEN, TEST_GRID_COL_LEN)?);
		Ok(())
	}
}
