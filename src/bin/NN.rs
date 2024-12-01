use std::fs;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

fn part1(input: &str) -> Result<usize> {
    // TODO: Solve Part 1 of the puzzle
    let _ = input;
    Ok(0)
}

fn part2(input: &str) -> Result<usize> {
    let _ = input;
    Ok(0)
}

const DAY: &str = "NN"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "<TEST-INPUT>"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Part 1 ===");
    let input_file = fs::read_to_string(INPUT_FILE)?;
    let result = time_snippet!(part1(input_file.as_str())?);
    println!("Result = {}", result);

    // println!("\n=== Part 2 ===");
    // let result = time_snippet!(part2(input_file.as_str())?);
    // println!("Result = {}", result);

    Ok(())
}

#[test]
fn test_part_one() -> Result<()> {
    assert_eq!(0, part1(TEST)?);

    Ok(())
}

#[test]
fn test_part_two() -> Result<()> {
    assert_eq!(0, part2(TEST)?);

    Ok(())
}
