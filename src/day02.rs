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

#[derive(Clone, Copy, PartialEq, Eq)]
enum ReactorStatus {
    Safe,
    Tolerable,
    Unsafe,
}

impl ReactorStatus {
    pub fn is_safe(&self) -> bool {
        self != &ReactorStatus::Unsafe
    }
}

fn process_report(report: &[u64]) -> ReactorStatus {
    fn calc_level_diff(prev_level: u64, next_level: u64) -> (LevelDirection, u64) {
        match next_level.cmp(&prev_level) {
            Ordering::Less => (LevelDirection::Decreasing, prev_level - next_level),
            Ordering::Equal => (LevelDirection::Unchanged, 0),
            Ordering::Greater => (LevelDirection::Increasing, next_level - prev_level),
        }
    }

    fn process_report_rec(prev_level: Option<u64>, dir: LevelDirection, reports_to_process: &[u64], status: ReactorStatus) -> ReactorStatus {
        if status == ReactorStatus::Unsafe {
            // nothing can ever bring a reactor back from Unsafe
            return ReactorStatus::Unsafe;
        }
        if dir == LevelDirection::Unchanged {
            // an unchanging level means an Unsafe reactor
            return ReactorStatus::Unsafe;
        }

        let (&level, remaining_reports) = match reports_to_process.split_first() {
            Some(v) => v,
            // split_first() returns None when reports_to_process is empty, return current status
            None => return status,
        };

        // process assuming the current level will be executed

        // get previous level and level direction
        let recurse = if let Some(prev_level) = prev_level {
            if dir != LevelDirection::Unknown {
                let (new_dir, value_delta) = calc_level_diff(prev_level, level);
                if dir != new_dir || value_delta > 3 {
                    // if we include this value, then...
                    ReactorStatus::Unsafe
                } else {
                    process_report_rec(Some(level), new_dir, remaining_reports, status)
                }
            } else {
                // no level direction yet, we are the second level
                // only check level difference
                let (starting_dir, value_delta) = calc_level_diff(prev_level, level);
                if value_delta == 0 || value_delta > 3 {
                    // if we include this value, then...
                    ReactorStatus::Unsafe
                } else {
                    process_report_rec(Some(level), starting_dir, remaining_reports, status)
                }
            }
        } else {
            // no previous level means this is the first level
            process_report_rec(Some(level), dir, remaining_reports, status)
        };

        // if we haven't utilized the Problem Dampener yet and executing this step led to the reactor becoming unsafe...
        if status == ReactorStatus::Safe && recurse == ReactorStatus::Unsafe {
            // then utilize the Problem Dampener to skip this level reading
            // nothing else we can do if skipping this step doesn't make the reactor safer
            process_report_rec(prev_level, dir, remaining_reports, ReactorStatus::Tolerable)
        } else {
            recurse
        }
    }

    process_report_rec(None, LevelDirection::Unknown, report, ReactorStatus::Safe)
}

pub fn part2(input: &str) -> Result<u64> {
    let reports = parse(input);

    let safe = reports.into_iter().fold(0, |cnt, report| {
        let status = process_report(&report);
        if status.is_safe() {
            cnt + 1
        } else {
            cnt
        }
    });

    Ok(safe)
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
        assert_eq!(4, part2(TEST)?);
        Ok(())
    }
}
