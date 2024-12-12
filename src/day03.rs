use anyhow::*;
use nom::bytes::complete::{tag, take, take_while_m_n};
use nom::character::complete::char;
use nom::combinator::{map, map_res, value};
use nom::{AsChar, IResult};
use nom::branch::alt;
use nom::multi::{many1, many_till};
use nom::sequence::{delimited, separated_pair};

fn digit1to3(input: &str) -> IResult<&str, &str> {
	take_while_m_n(1, 3, AsChar::is_dec_digit)(input)
}

fn parse1to3_u64(input: &str) -> IResult<&str, u64> {
	map_res(digit1to3, |num: &str| num.parse())(input)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct MultiplyInstruction {
	a: u64,
	b: u64
}
impl MultiplyInstruction {
	pub fn from((a, b): (u64, u64)) -> Self {
		Self { a, b }
	}
	pub fn result(&self) -> u64 {
		self.a * self.b
	}
}

fn parse_mul_instruction(input: &str) -> IResult<&str, MultiplyInstruction> {
	map(delimited(
		tag("mul("),
		separated_pair(
			parse1to3_u64,
			char(','),
			parse1to3_u64,
		),
		tag(")")
	), |val| MultiplyInstruction::from(val))(input)
}

fn take_till_mul_instruction(input: &str) -> IResult<&str, MultiplyInstruction> {
	map(many_till(
		take(1usize),
		parse_mul_instruction
	), |(_, mul)| mul)(input)
}

fn many1_mul_instruction(input: &str) -> IResult<&str, Vec<MultiplyInstruction>> {
	many1(take_till_mul_instruction)(input)
}

fn parse(input: &str) -> Vec<MultiplyInstruction> {
	let (_, parsed) = many1_mul_instruction(input).unwrap();
	parsed
}

pub fn part1(input: &str) -> Result<u64> {
	let valid_instructions = parse(input);
	Ok(valid_instructions.into_iter().fold(0, |sum, inst| sum + inst.result()))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Instruction {
	Multiply(MultiplyInstruction),
	Enable,
	Disable
}

fn parse_mul_instruction_part2(input: &str) -> IResult<&str, Instruction> {
	map(delimited(
		tag("mul("),
		separated_pair(
			parse1to3_u64,
			char(','),
			parse1to3_u64,
		),
		tag(")")
	), |val| Instruction::Multiply(MultiplyInstruction::from(val)))(input)
}

fn parse_enable_instruction(input: &str) -> IResult<&str, Instruction> {
	value(Instruction::Enable, tag("do()"))(input)
}

fn parse_disable_instruction(input: &str) -> IResult<&str, Instruction> {
	value(Instruction::Disable, tag("don't()"))(input)
}

fn take_till_instruction(input: &str) -> IResult<&str, Instruction> {
	map(many_till(
		take(1usize),
		alt((parse_enable_instruction, parse_disable_instruction, parse_mul_instruction_part2))
	), |(_, inst)| inst)(input)
}

fn many1_instruction(input: &str) -> IResult<&str, Vec<Instruction>> {
	many1(take_till_instruction)(input)
}

fn parse_part2(input: &str) -> Vec<Instruction> {
	let (_, parsed) = many1_instruction(input).unwrap();
	parsed
}

pub fn part2(input: &str) -> Result<u64> {
	let valid_instructions = parse_part2(input);
	let (sum, _) = valid_instructions.into_iter().fold((0, true), |(sum, mul_enable), inst| {
		match inst {
			Instruction::Multiply(mul_inst) => {
				let new_sum = if mul_enable {
					sum + mul_inst.result()
				} else {
					sum
				};
				(new_sum, mul_enable)
			},
			Instruction::Enable => (sum, true),
			Instruction::Disable => (sum, false),
		}
	});
	Ok(sum)
}

#[cfg(test)]
mod tests {
	use crate::day03::*;

	const TEST: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

	#[test]
	fn test_part_one() -> Result<()> {
		assert_eq!(161, part1(TEST)?);
		Ok(())
	}

	const TEST2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

	#[test]
	fn test_part_two() -> Result<()> {
		assert_eq!(48, part2(TEST2)?);
		Ok(())
	}
}
