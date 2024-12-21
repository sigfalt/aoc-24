use anyhow::*;
use aoc_24::day20::*;
use aoc_24::start_day;

const DAY: &str = "20";

pub fn main() -> Result<()> {
	let input_file = start_day(DAY)?;
	let input = input_file.as_str();

	println!("=== Part 1 ===");
	let result = part1(input)?;
	println!("Result = {}", result);

	println!("\n=== Part 2 ===");
	let result = part2(input)?;
	println!("Result = {}", result);

	Ok(())
}
