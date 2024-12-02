use anyhow::*;
use nom::character::complete::{digit1, line_ending, space1};
use nom::combinator::{all_consuming, map_res};
use nom::multi::separated_list1;
use nom::IResult;
use std::cmp::{max, min, Ordering, PartialEq};
use std::ops::ControlFlow;

fn parse_u64(input: &str) -> IResult<&str, u64> {
    map_res(digit1, |num: &str| num.parse())(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(space1, parse_u64)(input)
}

fn parse(input: &str) -> Vec<Vec<u64>> {
    all_consuming(separated_list1(line_ending, parse_line))(input)
        .unwrap()
        .1
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum LevelDirection {
    Increasing,
    Decreasing,
    Unchanged,
    Unknown,
}
impl Default for LevelDirection {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
struct PreviousLevel {
    pub value: u64,
    pub direction: LevelDirection,
}
impl PreviousLevel {
    fn start_at(value: u64) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }
    fn continue_from(prev_val: u64, value: u64) -> Self {
        Self {
            value,
            direction: match value.cmp(&prev_val) {
                Ordering::Less => LevelDirection::Decreasing,
                Ordering::Equal => LevelDirection::Unchanged,
                Ordering::Greater => LevelDirection::Increasing,
            },
        }
    }
}

pub fn part1(input: &str) -> Result<u64> {
    let reports = parse(input);
    let safe = reports.into_iter().fold(0, |cnt, report| {
        let x = report
            .into_iter()
            .try_fold(None, |prev_level: Option<PreviousLevel>, level| {
                if let Some(prev_level) = prev_level {
                    let pass_val = PreviousLevel::continue_from(prev_level.value, level);
                    let value_diff = max(prev_level.value, level) - min(prev_level.value, level);
                    if prev_level.direction != LevelDirection::Unknown {
                        if value_diff > 3 || pass_val.direction != prev_level.direction {
                            ControlFlow::Break(())
                        } else {
                            ControlFlow::Continue(Some(pass_val))
                        }
                    } else {
                        // don't know direction yet, only check value difference
                        if value_diff > 3 || pass_val.direction == LevelDirection::Unchanged {
                            ControlFlow::Break(())
                        } else {
                            ControlFlow::Continue(Some(pass_val))
                        }
                    }
                } else {
                    // prev_level empty means this is the first iteration
                    // always considered safe at this point, just continue
                    ControlFlow::Continue(Some(PreviousLevel::start_at(level)))
                }
            });
        match x {
            ControlFlow::Continue(_) => cnt + 1,
            ControlFlow::Break(_) => cnt
        }
    });

    Ok(safe)
}

pub fn part2(input: &str) -> Result<u64> {
    let _ = input;
    Ok(0)
}

#[cfg(test)]
mod tests {
    use crate::day02::*;

    const TEST: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(2, part1(TEST)?);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        assert_eq!(0, part2(TEST)?);
        Ok(())
    }
}
