use std::collections::VecDeque;
use anyhow::*;
use itertools::{repeat_n, Itertools};
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, line_ending};
use nom::combinator::{all_consuming, map, map_res, opt};
use nom::{Finish, IResult};
use nom::multi::{many1, separated_list1};
use nom::sequence::{separated_pair, terminated};

#[derive(Clone, Debug, PartialEq, Eq)]
struct Equation {
	test_val: u64,
	numbers: Vec<u64>
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Operator {
	Addition,
	Multiplication
}
impl Operator {
	pub const fn values() -> [Operator; 2] {
		[
			Operator::Addition,
			Operator::Multiplication
		]
	}
}

fn parse_u64(input: &str) -> IResult<&str, u64> {
	map_res(digit1, |num: &str| num.parse())(input)
}

fn parse_equation(input: &str) -> IResult<&str, Equation> {
	map(separated_pair(
		parse_u64,
		tag(": "),
		terminated(separated_list1(char(' '), parse_u64), opt(line_ending))
	), |(test_val, numbers)| Equation { test_val, numbers })(input)
}

fn parse(input: &str) -> Vec<Equation> {
	let (_, result) = all_consuming(many1(parse_equation))(input).finish().unwrap();
	result
}

pub fn part1(input: &str) -> Result<u64> {
	let equations = parse(input);

	let result = equations.into_iter().filter_map(|Equation { test_val, numbers }| {
		let mut numbers = VecDeque::from_iter(numbers.clone());
		let starting_value = numbers.pop_front().unwrap();
		let numbers = numbers;
		let found_good_operations = repeat_n(Operator::values().into_iter(), numbers.len()).multi_cartesian_product().into_iter().find(|operators| {
			let total_value = operators.into_iter().zip_eq(numbers.clone()).fold(starting_value, |total, (op, next_val)| {
				match op {
					Operator::Addition => total + next_val,
					Operator::Multiplication => total * next_val
				}
			});
			total_value == test_val
		});

		if let Some(_) = found_good_operations {
			Some(test_val)
		} else {
			None
		}
	}).sum();

	Ok(result)
}

pub fn part2(input: &str) -> Result<u64> {
	let equations = parse(input);

	fn process_operation_recursive(value: u64, numbers: &[u64], target: u64) -> bool {
		if let Some((next_number, numbers)) = numbers.split_first() {
			process_operation_recursive(value + next_number, numbers, target)
				|| process_operation_recursive(value * next_number, numbers, target)
				|| process_operation_recursive(value * 10u64.pow(next_number.ilog10() + 1) + next_number, numbers, target)
		} else {
			value == target
		}
	}

	let result = equations.into_iter().filter_map(|Equation { test_val, numbers }| {
		let (&starting_value, numbers) = numbers.as_slice().split_first().unwrap();
		let found_good_operations = process_operation_recursive(starting_value, numbers, test_val);

		if found_good_operations {
			Some(test_val)
		} else {
			None
		}
	}).sum();

	Ok(result)
}

#[cfg(test)]
mod tests {
	use crate::day07::*;

	const TEST: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

	#[test]
	fn test_part_one() -> Result<()> {
		assert_eq!(3749, part1(TEST)?);
		Ok(())
	}

	#[test]
	fn test_part_two() -> Result<()> {
		assert_eq!(11387, part2(TEST)?);
		Ok(())
	}
}
