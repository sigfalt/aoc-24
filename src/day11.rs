use anyhow::*;
use itertools::Itertools;
use nom::character::complete::{char, digit1};
use nom::combinator::{all_consuming, map_res};
use nom::{Finish, IResult};
use nom::multi::separated_list1;

fn parse_u64(input: &str) -> IResult<&str, u64> {
    map_res(digit1, |num: &str| num.parse())(input)
}

fn parse(input: &str) -> Vec<u64> {
    let (_, result) = all_consuming(separated_list1(char(' '), parse_u64))(input).finish().unwrap();
    result
}

fn blink(rocks: Vec<u64>) -> impl Iterator<Item = u64> {
    rocks.into_iter().flat_map(|rock| {
        if rock == 0 {
            vec![1]
        } else if (rock.ilog10() + 1) % 2 == 0 {
            let num_digits = rock.ilog10() + 1;
            let base = 10u64.pow(num_digits / 2);
            vec![rock / base, rock % base]
        } else {
            vec![rock * 2024]
        }
    })
}

pub fn part1(input: &str) -> Result<u64> {
    let rocks = parse(input);

    let rocks = (0..25).fold(rocks, |rocks , _| blink(rocks).collect_vec());

    Ok(rocks.len() as u64)
}

pub fn part2(input: &str) -> Result<u64> {
    let _ = input;
    Ok(0)
}

#[cfg(test)]
mod tests {
    use crate::day11::*;

    const TEST: &str = "125 17";

    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(55312, part1(TEST)?);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        assert_eq!(0, part2(TEST)?);
        Ok(())
    }
}
