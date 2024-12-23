use std::iter::successors;
use ahash::AHashMap;
use anyhow::*;
use itertools::Itertools;
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
	let seeds = parse(input);

	let all_buyer_secrets = seeds.into_iter().map(|secret|
		successors(Some(secret), |&secret| Some(generate_next_secret(secret))).take(2000)
	);
	let sequence_profit_map = all_buyer_secrets.fold(AHashMap::new(), |sequence_profit_map, secrets| {
		let prices = secrets.map(|secret| (secret % 10) as i8);
		let deltas = prices.clone().tuple_windows().map(|(first, second)| second - first);

		// sequences are defined as an ordered collection of four price changes
		let sequences = deltas.tuple_windows::<(_, _, _, _)>();

		// we will never sell on the first four prices provided by a buyer as there will not yet be
		// enough information to specify a sequence of four price changes on which to sell
		let sequence_price_mappings = sequences.zip(prices.skip(4));

		let sequence_price_map = sequence_price_mappings.fold(AHashMap::new(), |mut sequence_price_map, (sequence, price)| {
			sequence_price_map.entry(sequence).or_insert(price);
			sequence_price_map
		});

		sequence_price_map.into_iter().fold(sequence_profit_map, |mut profit_map, (sequence, price)| {
			profit_map.entry(sequence)
				.and_modify(|profit| { *profit += price as u64; })
				.or_insert(price as u64);
			profit_map
		})
	});

	Ok(sequence_profit_map.into_values().max().unwrap())
}

#[cfg(test)]
mod tests {
	use crate::day22::*;

	#[test]
	fn test_part_one() -> Result<()> {
		assert_eq!(37327623, part1("1
10
100
2024")?);
		Ok(())
	}

	#[test]
	fn test_part_two() -> Result<()> {
		assert_eq!(23, part2("1
2
3
2024")?);
		Ok(())
	}
}
