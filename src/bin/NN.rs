use std::fs;
use anyhow::*;
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

fn main() -> Result<()> {
    start_day(DAY);

    let input_file = fs::read_to_string(INPUT_FILE)?;
    let input = input_file.as_str();

    println!("=== Part 1 ===");
    let result = part1(input)?;
    println!("Result = {}", result);

    // println!("\n=== Part 2 ===");
    // let result = part2(input)?;
    // println!("Result = {}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TEST: &str = "<TEST-INPUT>"; // TODO: Add the test input

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
}
