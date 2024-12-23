use std::iter::successors;
use anyhow::*;
use nom::character::complete::{digit1, line_ending};
use nom::combinator::{all_consuming, map_res};
use nom::{Finish, IResult};
use nom::multi::separated_list1;

fn parse_u64(input: &str) -> IResult<&str, u64> {
	map_res(digit1, |num: &str| num.parse())(input)
}

fn parse(input: &str) -> Vec<u64> {
	let (_, result) = all_consuming(separated_list1(line_ending, parse_u64))(input).finish().unwrap();
	result
}

fn mix(a: u64, b: u64) -> u64 {
	a ^ b
}

fn prune(a: u64) -> u64 {
	a % 16777216
}

fn generate_next_secret(secret: u64) -> u64 {
	let phase1 = prune(mix(secret, secret * 64));
	let phase2 = prune(mix(phase1, phase1 / 32));
	let phase3 = prune(mix(phase2, phase2 * 2048));

	phase3
}

pub fn part1(input: &str) -> Result<u64> {
	let seeds = parse(input);

	let new_secrets = seeds.into_iter().map(|seed| {
		successors(Some(seed), |&secret| Some(generate_next_secret(secret)))
	}).map(|mut seed_sequence| seed_sequence.nth(2000).unwrap());

	Ok(new_secrets.sum())
}

pub fn part2(input: &str) -> Result<u64> {
	let _ = input;
	Ok(0)
}

#[cfg(test)]
mod tests {
	use crate::day22::*;

	const TEST: &str = "1
10
100
2024";

	#[test]
	fn test_part_one() -> Result<()> {
		assert_eq!(37327623, part1(TEST)?);
		Ok(())
	}

	#[test]
	fn test_part_two() -> Result<()> {
		assert_eq!(0, part2(TEST)?);
		Ok(())
	}
}
