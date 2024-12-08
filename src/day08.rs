use anyhow::*;

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
    use crate::day08::*;

    const TEST: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(14, part1(TEST)?);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        assert_eq!(0, part2(TEST)?);
        Ok(())
    }
}
