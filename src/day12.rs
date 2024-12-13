use anyhow::*;
use itertools::Itertools;

pub fn part1(input: &str) -> Result<u64> {
	// TODO: Solve Part 1 of the puzzle
	let _ = input;
	Ok(0)
}

pub fn part2(input: &str) -> Result<u64> {
	let _ = input;
	Ok(0)
}

#[cfg(test)]
mod tests {
	use crate::day12::*;

	const TEST: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

	#[test]
	fn test_part_one() -> Result<()> {
		assert_eq!(1930, part1(TEST)?);
		Ok(())
	}

	#[test]
	fn test_part_two() -> Result<()> {
		assert_eq!(0, part2(TEST)?);
		Ok(())
	}
}
