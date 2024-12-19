use std::fmt::{Display, Formatter};
use ahash::AHashMap;
use anyhow::*;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, one_of};
use nom::combinator::{all_consuming, map, map_res};
use nom::{Finish, IResult};
use nom::multi::{many1, separated_list1};
use nom::sequence::separated_pair;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Stripe {
	White,
	Blue,
	Black,
	Red,
	Green
}
impl Stripe {
	pub const fn to_char(&self) -> char {
		match self {
			Stripe::White => 'w',
			Stripe::Blue => 'u',
			Stripe::Black => 'b',
			Stripe::Red => 'r',
			Stripe::Green => 'g'
		}
	}
}

fn parse_stripe(input: &str) -> IResult<&str, Stripe> {
	map_res(one_of("wubrg"), |chr| match chr {
		'w' => Ok(Stripe::White),
		'u' => Ok(Stripe::Blue),
		'b' => Ok(Stripe::Black),
		'r' => Ok(Stripe::Red),
		'g' => Ok(Stripe::Green),
		_ => bail!("unrecognized stripe in input")
	})(input)
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Towel {
	stripes: Vec<Stripe>
}
impl Display for Towel {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.stripes.iter().map(|stripe| {
			write!(f, "{}", stripe.to_char())
		}).collect()
	}
}

fn parse_towel(input: &str) -> IResult<&str, Towel> {
	map(many1(parse_stripe), |stripes| Towel { stripes })(input)
}

fn parse_available_patterns(input: &str) -> IResult<&str, Vec<Towel>> {
	separated_list1(tag(", "), parse_towel)(input)
}

fn parse_goal_patterns(input: &str) -> IResult<&str, Vec<Towel>> {
	separated_list1(line_ending, parse_towel)(input)
}

fn parse(input: &str) -> (Vec<Towel>, Vec<Towel>) {
	let (_, result) = all_consuming(separated_pair(parse_available_patterns, many1(line_ending), parse_goal_patterns))(input).finish().unwrap();
	result
}

pub fn part1(input: &str) -> Result<u64> {
	let (available_patterns, goal_patterns) = parse(input);

	let mut solved_patterns = AHashMap::from_iter(available_patterns.into_iter().map(|towel| (towel, true)));

	let mut calculate_if_possible = |towel: Towel| -> bool {
		fn rec(towel: Towel, solved_patterns: &mut AHashMap<Towel, bool>) -> bool {
			if !solved_patterns.contains_key(&towel) {
				// a towel is possible if we can split it anywhere and construct both sub-towels
				let solvable = (1..towel.stripes.len()).into_iter().any(|split_ix| {
					// recursively check (calculating if required) if both sub-towels are possible
					let (left, right) = towel.stripes.split_at(split_ix);
					rec(Towel { stripes: left.to_vec() }, solved_patterns)
						&& rec(Towel { stripes: right.to_vec() }, solved_patterns)
				});
				solved_patterns.insert(towel.clone(), solvable);
			}

			solved_patterns.get(&towel).cloned().unwrap_or_default()
		}

		rec(towel, &mut solved_patterns)
	};

	let found_goals = goal_patterns.into_iter().fold(0, |sum, goal_towel| {
		let result = calculate_if_possible(goal_towel);

		if result { sum + 1 } else { sum }
	});

	Ok(found_goals)
}

pub fn part2(input: &str) -> Result<u64> {
	let _ = input;
	Ok(0)
}

#[cfg(test)]
mod tests {
	use crate::day19::*;

	const TEST: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

	#[test]
	fn test_part_one() -> Result<()> {
		assert_eq!(6, part1(TEST)?);
		Ok(())
	}

	#[test]
	fn test_part_two() -> Result<()> {
		assert_eq!(0, part2(TEST)?);
		Ok(())
	}
}
