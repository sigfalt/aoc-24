use std::ops::{ControlFlow, Div, Mul, Rem, Sub};
use anyhow::*;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, line_ending, one_of};
use nom::combinator::{all_consuming, map, map_res, opt};
use nom::{Finish, IResult};
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair, terminated, tuple};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ButtonDelta {
	x: u64,
	y: u64,
}
impl Mul<u64> for ButtonDelta {
	type Output = ButtonDelta;

	fn mul(self, rhs: u64) -> Self::Output {
		Self::Output {
			x: self.x * rhs,
			y: self.y * rhs
		}
	}
}
impl Mul<&u64> for ButtonDelta {
	type Output = ButtonDelta;

	fn mul(self, rhs: &u64) -> Self::Output {
		Self::Output {
			x: self.x * rhs,
			y: self.y * rhs
		}
	}
}
impl Mul<u64> for &ButtonDelta {
	type Output = ButtonDelta;

	fn mul(self, rhs: u64) -> Self::Output {
		Self::Output {
			x: self.x * rhs,
			y: self.y * rhs
		}
	}
}
impl Mul<&u64> for &ButtonDelta {
	type Output = ButtonDelta;

	fn mul(self, rhs: &u64) -> Self::Output {
		Self::Output {
			x: self.x * rhs,
			y: self.y * rhs
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct PrizeLocation {
	x: u64,
	y: u64,
}
impl Div<ButtonDelta> for PrizeLocation {
	type Output = (u64, u64);

	fn div(self, rhs: ButtonDelta) -> Self::Output {
		(self.x / rhs.x, self.y / rhs.y)
	}
}
impl Rem<ButtonDelta> for PrizeLocation {
	type Output = (u64, u64);

	fn rem(self, rhs: ButtonDelta) -> Self::Output {
		(self.x % rhs.x, self.y % rhs.y)
	}
}
impl Sub<ButtonDelta> for PrizeLocation {
	type Output = PrizeLocation;

	fn sub(self, rhs: ButtonDelta) -> Self::Output {
		Self::Output {
			x: self.x - rhs.x,
			y: self.y - rhs.y
		}
	}
}
impl PrizeLocation {
	pub fn checked_sub(self, rhs: ButtonDelta) -> Option<PrizeLocation> {
		Some(PrizeLocation {
			x: self.x.checked_sub(rhs.x)?,
			y: self.y.checked_sub(rhs.y)?
		})
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ClawMachine {
	a: ButtonDelta,
	b: ButtonDelta,
	prize: PrizeLocation
}

fn parse_u64(input: &str) -> IResult<&str, u64> {
	map_res(digit1, |num: &str| num.parse())(input)
}

fn parse_button(input: &str) -> IResult<&str, ButtonDelta> {
	map(separated_pair(
		preceded(tuple((tag("Button "), one_of("AB"), tag(": X+"))), parse_u64),
		tag(", Y+"),
		terminated(parse_u64, line_ending)
	), |(x, y)| ButtonDelta { x, y })(input)
}

fn parse_prize(input: &str) -> IResult<&str, PrizeLocation> {
	map(separated_pair(
		preceded(tag("Prize: X="), parse_u64),
		tag(", Y="),
		terminated(parse_u64, opt(line_ending))
	), |(x, y)| PrizeLocation { x, y })(input)
}

fn parse_claw_machine(input: &str) -> IResult<&str, ClawMachine> {
	map(tuple((
		parse_button,
		parse_button,
		parse_prize
	)), |(a, b, prize)| ClawMachine { a, b, prize })(input)
}

fn parse(input: &str) -> Vec<ClawMachine> {
	let (_, result) = all_consuming(separated_list1(line_ending, parse_claw_machine))(input).finish().unwrap();
	result
}

pub fn part1(input: &str) -> Result<u64> {
	let claw_machines = parse(input);

	const A_COST: u64 = 3;
	const B_COST: u64 = 1;
	const MAX_PRESSES: u64 = 100;

	let spent_tokens = claw_machines.into_iter().filter_map(|claw_machine| {
		if let ControlFlow::Break(Some(token_cost)) = (0..=MAX_PRESSES).try_for_each(|a_presses| {
			let a_distance = claw_machine.a * a_presses;
			if let Some(remaining_distance) = claw_machine.prize.checked_sub(a_distance) {
				if let Some(b_presses) = match (remaining_distance % claw_machine.b, remaining_distance / claw_machine.b) {
					((x_rem, y_rem), (x_mult, y_mult))
					if x_rem == 0 && y_rem == 0 && x_mult == y_mult && x_mult <= MAX_PRESSES => {
						Some(x_mult)
					},
					_ => None
				} {
					ControlFlow::Break(Some((a_presses * A_COST) + (b_presses * B_COST)))
				} else {
					ControlFlow::Continue(())
				}
			} else {
				ControlFlow::Break(None)
			}
		}) {
			Some(token_cost)
		} else {
			None
		}
	}).sum();

	Ok(spent_tokens)
}

pub fn part2(input: &str) -> Result<u64> {
	let _ = input;
	Ok(0)
}

#[cfg(test)]
mod tests {
	use crate::day13::*;

	const TEST: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

	#[test]
	fn test_part_one() -> Result<()> {
		assert_eq!(480, part1(TEST)?);
		Ok(())
	}

	#[test]
	fn test_part_two() -> Result<()> {
		assert_eq!(0, part2(TEST)?);
		Ok(())
	}
}
