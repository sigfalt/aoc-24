use anyhow::*;

pub fn part1(input: &str) -> Result<usize> {
    // TODO: Solve Part 1 of the puzzle
    let _ = input;
    Ok(0)
}

pub fn part2(input: &str) -> Result<usize> {
    let _ = input;
    Ok(0)
}

#[cfg(test)]
mod tests {
    use crate::dayNN::*;

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
