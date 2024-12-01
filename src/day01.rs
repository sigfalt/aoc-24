use std::cmp::{max, min};
use ahash::AHashMap;
use anyhow::*;
use nom::character::complete::{digit1, line_ending, space1};
use nom::combinator::{all_consuming, map_res};
use nom::{IResult};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;

fn parse_u64(input: &str) -> IResult<&str, u64> {
	map_res(digit1, |num: &str| num.parse())(input)
}

fn parse_line(input: &str) -> IResult<&str, (u64, u64)> {
	separated_pair(parse_u64, space1, parse_u64)(input)
}

fn parse(input: &str) -> Vec<(u64, u64)> {
	all_consuming(separated_list1(
		line_ending,
		parse_line
	))(input).unwrap().1
}

pub fn part1(input: &str) -> Result<u64> {
	let parsed = parse(input);
	let (mut left, mut right): (Vec<_>, Vec<_>) = parsed.into_iter().unzip();

	left.sort();
	right.sort();

	Ok(left.into_iter().zip(right).map(|(a, b)| max(a, b) - min(a, b)).sum())
}

pub fn part2(input: &str) -> Result<u64> {
	let parsed = parse(input);
	let (left, right): (Vec<_>, Vec<_>) = parsed.into_iter().unzip();

	let mut count_map = AHashMap::new();
	right.into_iter().for_each(|val| {
		count_map.entry(val).and_modify(|cnt| *cnt += 1).or_insert(1u64);
	});
	Ok(left.into_iter().map(|val| val * count_map.get(&val).unwrap_or(&0)).sum())
}

#[cfg(test)]
mod tests {
	use crate::day01::*;

	const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3";

	#[test]
	fn test_part_one() -> Result<()> {
		assert_eq!(11, part1(TEST)?);

		Ok(())
	}

	#[test]
	fn test_part_two() -> Result<()> {
		assert_eq!(31, part2(TEST)?);

		Ok(())
	}
}
