use ahash::AHashMap;
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
    let rocks = parse(input);
    const MAX_AGE: u64 = 75;

    let mut rock_age_count_cache = AHashMap::new();
    let mut count_rocks = |rock: u64, depth: u64| -> u64 {
        fn rec(rock: u64, depth: u64, rock_age_count_cache: &mut AHashMap<(u64, u64), u64>) -> u64 {
            if let Some(&count) = rock_age_count_cache.get(&(rock, depth)) {
                return count;
            }

            let count = if depth == 0 {
                1
            } else {
                if rock == 0 {
                    rec(1, depth - 1, rock_age_count_cache)
                } else if (rock.ilog10() + 1) % 2 == 0 {
                    let num_digits = rock.ilog10() + 1;
                    let base = 10u64.pow(num_digits / 2);
                    rec(rock / base, depth - 1, rock_age_count_cache)
                        + rec(rock % base, depth - 1, rock_age_count_cache)
                } else {
                    rec(rock * 2024, depth - 1, rock_age_count_cache)
                }
            };
            rock_age_count_cache.insert((rock, depth), count);

            count
        }
        rec(rock, depth, &mut rock_age_count_cache)
    };

    let rock_count = rocks.into_iter().map(|rock| count_rocks(rock, MAX_AGE)).sum();
    Ok(rock_count)
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

}
