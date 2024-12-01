use std::cmp::{max, min};
use std::fs;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use nom::character::complete::{digit1, line_ending, space1};
use nom::combinator::{all_consuming, map_res};
use nom::{IResult};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use adv_code_2024::*;

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

fn part1(input: &str) -> Result<u64> {
	let parsed = parse(input);
	let (mut left, mut right): (Vec<_>, Vec<_>) = parsed.into_iter().unzip();

	left.sort();
	right.sort();

	Ok(left.into_iter().zip(right).map(|(a, b)| max(a, b) - min(a, b)).sum())

	// Ok(11)
}



// framework junk

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3";

fn main() -> Result<()> {
	start_day(DAY);

	//region Part 1
	println!("=== Part 1 ===");

	assert_eq!(11, part1(TEST)?);

	let input_file = fs::read_to_string(INPUT_FILE)?;
	let result = time_snippet!(part1(input_file.as_str())?);
	println!("Result = {}", result);
	//endregion

	//region Part 2
	// println!("\n=== Part 2 ===");
	//
	// fn part2<R: BufRead>(reader: R) -> Result<usize> {
	//     Ok(0)
	// }
	//
	// assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
	//
	// let input_file = BufReader::new(File::open(INPUT_FILE)?);
	// let result = time_snippet!(part2(input_file)?);
	// println!("Result = {}", result);
	//endregion

	Ok(())
}
