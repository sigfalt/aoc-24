use ahash::AHashMap;
use anyhow::*;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, line_ending, one_of};
use nom::combinator::{all_consuming, map_res};
use nom::{Finish, IResult};
use nom::multi::{many1, separated_list1};
use nom::sequence::{preceded, separated_pair};

fn parse_usize(input: &str) -> IResult<&str, usize> {
	map_res(digit1, |num: &str| num.parse())(input)
}

fn parse_register(input: &str) -> IResult<&str, (Register, usize)> {
	separated_pair(
		preceded(tag("Register "), map_res(one_of("ABC"), |chr| match chr {
			'A' => Ok(Register::A),
			'B' => Ok(Register::B),
			'C' => Ok(Register::C),
			_ => bail!("unrecognized register in input")
		})),
		tag(": "),
		parse_usize
	)(input)
}

fn parse_registers(input: &str) -> IResult<&str, Vec<(Register, usize)>> {
	separated_list1(line_ending, parse_register)(input)
}

fn parse_program(input: &str) -> IResult<&str, Vec<usize>> {
	preceded(tag("Program: "), separated_list1(char(','), parse_usize))(input)
}

fn parse(input: &str) -> (Vec<(Register, usize)>, Vec<usize>) {
	let (_, result) = all_consuming(separated_pair(
		parse_registers, many1(line_ending), parse_program
	))(input).finish().unwrap();
	result
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Opcode {
	Adv,
	Bxl,
	Bst,
	Jnz,
	Bxc,
	Out,
	Bdv,
	Cdv,
}
impl Opcode {
	pub const fn try_parse(val: usize) -> Option<Self> {
		match val {
			0 => Some(Opcode::Adv),
			1 => Some(Opcode::Bxl),
			2 => Some(Opcode::Bst),
			3 => Some(Opcode::Jnz),
			4 => Some(Opcode::Bxc),
			5 => Some(Opcode::Out),
			6 => Some(Opcode::Bdv),
			7 => Some(Opcode::Cdv),
			_ => None,
		}
	}
	pub fn get_operand(&self, val: usize) -> impl Iterator<Item = Operand> {
		match self {
			Opcode::Adv | Opcode::Bdv | Opcode::Cdv =>
				vec![Operand::Register(Register::A), Operand::parse_combo(val).unwrap()],
			Opcode::Bxl => vec![Operand::Register(Register::B), Operand::Literal(val)],
			Opcode::Bst => vec![Operand::parse_combo(val).unwrap()],
			Opcode::Jnz => vec![Operand::Register(Register::A), Operand::Literal(val)],
			Opcode::Bxc => vec![Operand::Register(Register::B), Operand::Register(Register::C)],
			Opcode::Out => vec![Operand::parse_combo(val).unwrap()],
		}.into_iter()
	}
	pub fn execute(&self, data: Vec<usize>) -> Option<(Register, usize)> {
		let args = data.len();
		match (self, args) {
			(Opcode::Adv, 2) => Some((Register::A, data[0] / 2usize.pow(data[1] as u32))),
			(Opcode::Bxl, 2) => Some((Register::B, data[0] ^ data[1])),
			(Opcode::Bst, 1) => Some((Register::B, data[0] % 8)),
			(Opcode::Jnz, 2) => {
				if data[0] == 0 {
					None
				} else {
					Some((Register::IP, data[1]))
				}
			},
			(Opcode::Bxc, 2) => Some((Register::B, data[0] ^ data[1])),
			(Opcode::Out, 1) => Some((Register::OUT, data[0] % 8)),
			(Opcode::Bdv, 2) => Some((Register::B, data[0] / 2usize.pow(data[1] as u32))),
			(Opcode::Cdv, 2) => Some((Register::C, data[0] / 2usize.pow(data[1] as u32))),
			_ => None
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Register {
	A,
	B,
	C,
	IP,
	OUT
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Operand {
	Literal(usize),
	Register(Register)
}
impl Operand {
	pub const fn parse_combo(val: usize) -> Option<Operand> {
		match val {
			1..=3 => Some(Operand::Literal(val)),
			4 => Some(Operand::Register(Register::A)),
			5 => Some(Operand::Register(Register::B)),
			6 => Some(Operand::Register(Register::C)),
			7 => None, // reserved? part 2 alarms blaring
			_ => None,
		}
	}
}

pub fn part1(input: &str) -> Result<String> {
	let (init_registers, program_memory) = parse(input);

	let mut register_file = AHashMap::new();
	register_file.insert(Register::IP, 0);
	// assume all registers are input with a specific starting value
	init_registers.into_iter().for_each(|(register, value)| {
		register_file.insert(register, value);
	});
	let mut output = Vec::new();

	while let Some(&instruction) = program_memory.get(register_file[&Register::IP]) {
		let opcode = Opcode::try_parse(instruction).unwrap();
		// description says the computer halts when it tries to read an *opcode* past the end of the program
		// assume operand always exists in program memory after opcode
		let operand_request = opcode.get_operand(program_memory[register_file[&Register::IP] + 1]);
		let operand_values = operand_request.map(|operand_request| {
			match operand_request {
				Operand::Literal(val) => val,
				Operand::Register(reg) => register_file[&reg]
			}
		}).collect_vec();

		let mut jumped = false;
		if let Some((reg_to_modify, new_data)) = opcode.execute(operand_values) {
			match reg_to_modify {
				Register::A | Register::B | Register::C => { *register_file.get_mut(&reg_to_modify).unwrap() = new_data; },
				Register::IP => {
					*register_file.get_mut(&reg_to_modify).unwrap() = new_data;
					jumped = true;
				},
				Register::OUT => {
					output.push(new_data);
				},
			};
		}

		if !jumped { *register_file.get_mut(&Register::IP).unwrap() += 2; }
	}

	Ok(output.into_iter().join(",").to_string())
}

pub fn part2(input: &str) -> Result<u64> {
	let _ = input;
	Ok(0)
}

#[cfg(test)]
mod tests {
	use crate::day17::*;

	const TEST: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

	#[test]
	fn test_part_one() -> Result<()> {
		assert_eq!("4,6,3,5,6,3,5,2,1,0", part1(TEST)?);
		Ok(())
	}

	#[test]
	fn test_part_two() -> Result<()> {
		assert_eq!(0, part2(TEST)?);
		Ok(())
	}
}
