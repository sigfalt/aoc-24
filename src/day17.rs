use std::ops::ControlFlow;
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

// wow I totally misjudged where part 2 would go...

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
struct ComputerState {
	reg_a: usize,
	reg_b: usize,
	reg_c: usize,
	inst_ptr: usize,
	output: Vec<usize>
}
impl ComputerState {
	pub fn read_register(&self, register: Register) -> usize {
		match register {
			Register::A => self.reg_a,
			Register::B => self.reg_b,
			Register::C => self.reg_c,
			Register::IP => self.inst_ptr,
			Register::OUT => unreachable!("OUT register not used in part 2")
		}
	}
}

pub fn part2(input: &str) -> Result<usize> {
	let (init_registers, program_memory) = parse(input);

	let mut orig_state = ComputerState::default();
	init_registers.into_iter().for_each(|(register, value)| {
		match register {
			Register::B => orig_state.reg_b = value,
			Register::C => orig_state.reg_c = value,
			_ => {}
		}
	});

	let parse_combo = |operand: usize, state: &ComputerState| -> usize {
		match Operand::parse_combo(operand).unwrap() {
			Operand::Literal(operand) => operand,
			Operand::Register(register) => state.read_register(register)
		}
	};

	// let mut best_output = 0;

	// println!("goal output: {:?}", program_memory);
	let found_value = (0..).flat_map(|val| [(val << 33) + 0o132621633, (val << 33) + 0o132621635]).try_for_each(|a_reg_init_val| {
		let mut state = orig_state.clone();
		state.reg_a = a_reg_init_val;

		while let Some(&instruction) = program_memory.get(state.inst_ptr) {
			let opcode = Opcode::try_parse(instruction).unwrap();
			let operand = program_memory[state.inst_ptr + 1];

			let mut jumped = false;
			match opcode {
				Opcode::Adv => { state.reg_a = state.reg_a / 2usize.pow(parse_combo(operand, &state) as u32); }
				Opcode::Bxl => { state.reg_b = state.reg_b ^ operand; }
				Opcode::Bst => { state.reg_b = parse_combo(operand, &state) % 8; }
				Opcode::Jnz => {
					if state.reg_a != 0 {
						state.inst_ptr = operand;
						jumped = true;
					}
				}
				Opcode::Bxc => { state.reg_b = state.reg_b ^ state.reg_c; }
				Opcode::Out => {
					let next_output = parse_combo(operand, &state) % 8;
					// outputting a value cannot be undone or modified
					// if next value to output does not match what we want to see output next,
					// we can stop testing for this initial value of the A register
					if Some(&next_output) != program_memory.get(state.output.len()) {
						break;
					}
					state.output.push(next_output);
					// if state.output.len() > 11 {
					// 	best_output = state.output.len();
					// 	println!("A(0o{:o}) output: {:?}", a_reg_init_val, state.output);
					// }
				}
				Opcode::Bdv => { state.reg_b = state.reg_a / 2usize.pow(parse_combo(operand, &state) as u32); }
				Opcode::Cdv => { state.reg_c = state.reg_a / 2usize.pow(parse_combo(operand, &state) as u32); }
			}

			if !jumped { state.inst_ptr += 2; }
		}

		// println!("A({:?}) -> {:?}", a_reg_init_val, state.output);
		if state.output == program_memory {
			ControlFlow::Break(a_reg_init_val)
		} else {
			ControlFlow::Continue(())
		}
	});

	Ok(found_value.break_value().unwrap())
}

#[cfg(test)]
mod tests {
	use crate::day17::*;

	#[test]
	fn test_part_one() -> Result<()> {
		assert_eq!("4,6,3,5,6,3,5,2,1,0", part1("Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0")?);
		Ok(())
	}

	#[test]
	fn test_part_two() -> Result<()> {
		assert_eq!(117440, part2("Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0")?);
		Ok(())
	}
}
