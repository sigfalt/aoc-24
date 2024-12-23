use std::cell::LazyCell;
use std::iter::{once, repeat_n};
use std::ops::ControlFlow;
use ahash::{AHashMap, AHashSet};
use anyhow::*;
use grid::{grid, Grid};
use itertools::Itertools;
use nom::character::complete::{line_ending, one_of};
use nom::combinator::{all_consuming, map_res};
use nom::{Finish, IResult};
use nom::multi::{many1, separated_list1};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum NumericKeypad {
	Zero,
	One,
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
	Activate,
}
impl NumericKeypad {
	// numeric keypad diagram:
	// 7 8 9
	// 4 5 6
	// 1 2 3
	//   0 A
	const LAYOUT: LazyCell<Grid<Option<NumericKeypad>>> = LazyCell::new(|| grid![
		[Some(NumericKeypad::Seven), Some(NumericKeypad::Eight),     Some(NumericKeypad::Nine)]
		[ Some(NumericKeypad::Four),  Some(NumericKeypad::Five),      Some(NumericKeypad::Six)]
		[  Some(NumericKeypad::One),   Some(NumericKeypad::Two),    Some(NumericKeypad::Three)]
		[                      None,  Some(NumericKeypad::Zero), Some(NumericKeypad::Activate)]
	]);

	pub const fn get_pos(&self) -> (isize, isize) {
		match self {
			NumericKeypad::Seven => (0, 0),
			NumericKeypad::Eight => (0, 1),
			NumericKeypad::Nine => (0, 2),
			NumericKeypad::Four => (1, 0),
			NumericKeypad::Five => (1, 1),
			NumericKeypad::Six => (1, 2),
			NumericKeypad::One => (2, 0),
			NumericKeypad::Two => (2, 1),
			NumericKeypad::Three => (2, 2),
			NumericKeypad::Zero => (3, 1),
			NumericKeypad::Activate => (3, 2),
		}
	}

	pub fn move_to(&self, target: &NumericKeypad) -> Vec<Vec<MovementDirection>> {
		let (start_row, start_col) = self.get_pos();
		let (end_row, end_col) = target.get_pos();

		let row_delta = start_row.abs_diff(end_row);
		let row_steps = if start_row < end_row {
			repeat_n(MovementDirection::Down, row_delta)
		} else {
			repeat_n(MovementDirection::Up, row_delta)
		};

		let col_delta = start_col.abs_diff(end_col);
		let col_steps = if start_col < end_col {
			repeat_n(MovementDirection::Right, col_delta)
		} else {
			repeat_n(MovementDirection::Left, col_delta)
		};

		let move_permutation = row_steps.chain(col_steps).collect_vec();
		let permutation_len = move_permutation.len();
		move_permutation.into_iter().permutations(permutation_len)
			.fold(AHashSet::new(), |mut set, permutation| {
				set.insert(permutation);
				set
		}).into_iter().filter(|moveset| {
			let all_valid_positions = moveset.into_iter().try_fold((start_row, start_col), |prev_pos, movement| {
				if let Some((next_row, next_col)) = movement.offset_from(prev_pos) {
					if Self::LAYOUT.get(next_row, next_col).copied().flatten().is_some() {
						return ControlFlow::Continue((next_row, next_col));
					}
				}
				ControlFlow::Break(())
			});
			!all_valid_positions.is_break()
		}).collect_vec()
	}
}
impl TryFrom<NumericKeypad> for u64 {
	type Error = Error;
	fn try_from(value: NumericKeypad) -> std::result::Result<Self, Self::Error> {
		match value {
			NumericKeypad::Zero => Ok(0),
			NumericKeypad::One => Ok(1),
			NumericKeypad::Two => Ok(2),
			NumericKeypad::Three => Ok(3),
			NumericKeypad::Four => Ok(4),
			NumericKeypad::Five => Ok(5),
			NumericKeypad::Six => Ok(6),
			NumericKeypad::Seven => Ok(7),
			NumericKeypad::Eight => Ok(8),
			NumericKeypad::Nine => Ok(9),
			NumericKeypad::Activate => bail!("Activate button cannot be mapped to a digit"),
		}
	}
}
impl Default for NumericKeypad {
	fn default() -> Self {
		Self::Activate
	}
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum DirectionalKeypad {
	Up,
	Left,
	Down,
	Right,
	Activate
}
impl DirectionalKeypad {
	// directional keypad diagram:
	//   ^ A
	// < v >
	const LAYOUT: LazyCell<Grid<Option<DirectionalKeypad>>> = LazyCell::new(|| grid![
		[                         None,   Some(DirectionalKeypad::Up), Some(DirectionalKeypad::Activate)]
		[Some(DirectionalKeypad::Left), Some(DirectionalKeypad::Down),    Some(DirectionalKeypad::Right)]
	]);

	pub const fn get_pos(&self) -> (isize, isize) {
		match self {
			DirectionalKeypad::Up => (0, 1),
			DirectionalKeypad::Activate => (0, 2),
			DirectionalKeypad::Left => (1, 0),
			DirectionalKeypad::Down => (1, 1),
			DirectionalKeypad::Right => (1, 2),
		}
	}

	pub fn move_to(&self, target: &DirectionalKeypad) -> Vec<Vec<MovementDirection>> {
		let (start_row, start_col) = self.get_pos();
		let (end_row, end_col) = target.get_pos();

		let row_delta = start_row.abs_diff(end_row);
		let row_steps = if start_row < end_row {
			repeat_n(MovementDirection::Down, row_delta)
		} else {
			repeat_n(MovementDirection::Up, row_delta)
		};

		let col_delta = start_col.abs_diff(end_col);
		let col_steps = if start_col < end_col {
			repeat_n(MovementDirection::Right, col_delta)
		} else {
			repeat_n(MovementDirection::Left, col_delta)
		};

		let move_permutation = row_steps.chain(col_steps).collect_vec();
		let permutation_len = move_permutation.len();
		move_permutation.into_iter().permutations(permutation_len)
			.fold(AHashSet::new(), |mut set, permutation| {
				set.insert(permutation);
				set
			}).into_iter().filter(|moveset| {
			let all_valid_positions = moveset.into_iter().try_fold((start_row, start_col), |prev_pos, movement| {
				if let Some((next_row, next_col)) = movement.offset_from(prev_pos) {
					if Self::LAYOUT.get(next_row, next_col).copied().flatten().is_some() {
						return ControlFlow::Continue((next_row, next_col));
					}
				}
				ControlFlow::Break(())
			});
			!all_valid_positions.is_break()
		}).collect_vec()
	}
}
impl From<MovementDirection> for DirectionalKeypad {
	fn from(value: MovementDirection) -> Self {
		match value {
			MovementDirection::Up => DirectionalKeypad::Up,
			MovementDirection::Right => DirectionalKeypad::Right,
			MovementDirection::Down => DirectionalKeypad::Down,
			MovementDirection::Left => DirectionalKeypad::Left,
		}
	}
}
impl Default for DirectionalKeypad {
	fn default() -> Self {
		Self::Activate
	}
}

fn parse_keypress(input: &str) -> IResult<&str, NumericKeypad> {
	map_res(one_of("0123456789A"), |chr| match chr {
		'0' => Ok(NumericKeypad::Zero),
		'1' => Ok(NumericKeypad::One),
		'2' => Ok(NumericKeypad::Two),
		'3' => Ok(NumericKeypad::Three),
		'4' => Ok(NumericKeypad::Four),
		'5' => Ok(NumericKeypad::Five),
		'6' => Ok(NumericKeypad::Six),
		'7' => Ok(NumericKeypad::Seven),
		'8' => Ok(NumericKeypad::Eight),
		'9' => Ok(NumericKeypad::Nine),
		'A' => Ok(NumericKeypad::Activate),
		_ => bail!("")
	})(input)
}

fn parse_keycode(input: &str) -> IResult<&str, Vec<NumericKeypad>> {
	many1(parse_keypress)(input)
}

fn parse(input: &str) -> Vec<Vec<NumericKeypad>> {
	let (_, result) = all_consuming(separated_list1(line_ending, parse_keycode))(input).finish().unwrap();
	result
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum MovementDirection {
	Up,
	Right,
	Down,
	Left,
}
impl MovementDirection {
	pub const fn get_offset(&self) -> (isize, isize) {
		match self {
			MovementDirection::Up => (-1, 0),
			MovementDirection::Right => (0, 1),
			MovementDirection::Down => (1, 0),
			MovementDirection::Left => (0, -1),
		}
	}

	pub fn offset_from(&self, (row, col): (impl TryInto<isize>, impl TryInto<isize>)) -> Option<(isize, isize)> {
		let row_isize = row.try_into().ok()?;
		let col_isize = col.try_into().ok()?;
		let (row_offset, col_offset) = self.get_offset();
		Some((row_isize + row_offset, col_isize + col_offset))
	}
}

pub fn part1(input: &str) -> Result<u64> {
	solve(input, 3)
}

fn solve(input: &str, robot_chain_len: u64) -> Result<u64> {
	let input_codes = parse(input);

	let mut cost_cache: AHashMap<(DirectionalKeypad, DirectionalKeypad, u64), u64> = AHashMap::new();

	let mut get_cost = |prev_button: &DirectionalKeypad, next_button: &DirectionalKeypad, depth: u64| -> u64 {
		fn rec(prev_button: &DirectionalKeypad, next_button: &DirectionalKeypad, depth: u64, cache: &mut AHashMap<(DirectionalKeypad, DirectionalKeypad, u64), u64>) -> u64 {
			if let Some(&val) = cache.get(&(*prev_button, *next_button, depth)) {
				return val;
			}

			let cost = if depth == 0 {
				// cost of the button is just pressing the button
				1
			} else {
				let possible_movesets = prev_button.move_to(next_button);
				let moveset_cost = possible_movesets.into_iter().map(|moveset| {
					let (cost, _) = moveset.into_iter().map(DirectionalKeypad::from).chain(once(DirectionalKeypad::Activate))
						.fold((0, DirectionalKeypad::default()), |(sum, prev_btn), next_btn| {
							// the cost of a set of moves is the cost of each individual move, summed together
							(sum + rec(&prev_btn, &next_btn, depth - 1, cache), next_btn)
						});
					cost
				});
				// find the moveset with the lowest cost
				let min_cost = moveset_cost.min().unwrap();
				min_cost
			};
			cache.insert((*prev_button, *next_button, depth), cost);

			cost
		}
		rec(prev_button, next_button, depth, &mut cost_cache)
	};

	let complexities = input_codes.into_iter().map(|input_code| {
		// each input code is a sequence of numeric keypad presses we need to input, but indirectly
		//
		// we enter key presses on a directional keypad, which go through a variable number of levels
		// of directional keypad indirection, and finally to the numeric keypad
		//
		// put in reverse, the numeric key presses need to be used to determine whatever number of
		// levels of indirect directional key presses
		let numeric_code = input_code.iter().fold(0, |accum, &digit| {
			if let Result::Ok(digit) = u64::try_from(digit) {
				(accum * 10) + digit
			} else {
				accum
			}
		});

		let (input_min_cost, _) = input_code.into_iter().fold((0, NumericKeypad::default()), |(cost, prev_button), next_button| {
			let possible_movesets = prev_button.move_to(&next_button);
			let moveset_cost = possible_movesets.into_iter().map(|moveset| {
				let (cost, _) = moveset.into_iter().map(DirectionalKeypad::from).chain(once(DirectionalKeypad::Activate))
					.fold((0, DirectionalKeypad::default()), |(sum, prev_btn), next_btn| {
						(sum + get_cost(&prev_btn, &next_btn, robot_chain_len - 1), next_btn)
					});
				cost
			});
			let min_cost = moveset_cost.min().unwrap();

			(cost + min_cost, next_button)
		});

		input_min_cost * numeric_code
	});

	Ok(complexities.sum())
}

pub fn part2(input: &str) -> Result<u64> {
	solve(input, 26)
}

#[cfg(test)]
mod tests {
	use crate::day21::*;

	const TEST: &str = "029A
980A
179A
456A
379A";

	#[test]
	fn test() -> Result<()> {
		assert_eq!(126384, part1(TEST)?);
		Ok(())
	}
}
