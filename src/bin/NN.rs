use std::fs;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

fn part1(input: &str) -> Result<usize> {
    // TODO: Solve Part 1 of the puzzle
    Ok(0)
}

fn part2(input: &str) -> Result<usize> {
    // TODO: Solve Part 1 of the puzzle
    Ok(0)
}

const DAY: &str = "NN"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "<TEST-INPUT>"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");
    let input_file = fs::read_to_string(INPUT_FILE)?;

    // TODO: Set the expected answer for the test input
    assert_eq!(0, part1(TEST)?);

    let result = time_snippet!(part1(input_file.as_str())?);
    println!("Result = {}", result);

    // println!("\n=== Part 2 ===");
    //
    // assert_eq!(0, part2(TEST)?);
    //
    // let result = time_snippet!(part2(input_file.as_str())?);
    // println!("Result = {}", result);

    Ok(())
}
