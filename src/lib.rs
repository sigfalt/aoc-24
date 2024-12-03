use std::fs;
use anyhow::*;

pub mod day01;
pub mod day02;
pub mod day03;

pub fn start_day(day: &str) -> Result<String> {
    println!("Advent of Code 2024 - Day {:0>2}", day);

    Ok(fs::read_to_string(format!("input/{}.txt", day))?)
}

// Additional common functions

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let _ = start_day("00");
    }
}
