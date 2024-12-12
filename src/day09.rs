use std::collections::{BTreeMap, BTreeSet};
use std::iter::repeat;
use anyhow::*;
use nom::bytes::complete::take;
use nom::character::complete::line_ending;
use nom::combinator::{all_consuming, map_res, opt};
use nom::{Finish, IResult};
use nom::multi::many1;
use nom::sequence::terminated;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DiskContent {
	File{ id: u64 },
	FreeSpace
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DiskMap {
	content: DiskContent,
	length: u64
}

fn parse_digit(input: &str) -> IResult<&str, u64> {
	map_res(take(1u8), |dgt| u64::from_str_radix(dgt, 10))(input)
}

fn parse(input: &str) -> Vec<u64> {
	let (_, result) = all_consuming(terminated(many1(parse_digit), opt(line_ending)))(input).finish().unwrap();
	result
}

pub fn part1(input: &str) -> Result<u64> {
	let disk_map = parse(input);

	let disk_map = {
		let mut next_id = 0;
		disk_map.into_iter().enumerate().map(move |(ix, length)| {
			let content = if ix % 2 == 0 {
				let id = next_id;
				next_id += 1;
				DiskContent::File{ id }
			} else {
				DiskContent::FreeSpace
			};
			DiskMap { content, length }
		})
	};

	let (mut free_indices, mut filled_indices, _) = disk_map.fold((BTreeSet::new(), BTreeMap::new(), 0), |(mut free_set, mut fill_map, start_ix), mapping| {
		let next_ix = start_ix + mapping.length;
		let ix_iter = start_ix..next_ix;
		match mapping.content {
			DiskContent::File { id } => fill_map.extend(ix_iter.zip(repeat(id))),
			DiskContent::FreeSpace => free_set.extend(ix_iter),
		};
		(free_set, fill_map, next_ix)
	});

	while filled_indices.last_key_value().unwrap().0 > free_indices.first().unwrap() {
		let (max_filled_index, max_filled_value) = filled_indices.pop_last().unwrap();
		let min_free_space = free_indices.pop_first().unwrap();

		filled_indices.insert(min_free_space, max_filled_value);
		free_indices.insert(max_filled_index);
	}

	let checksum = filled_indices.into_iter().fold(0, |sum, (pos_ix, id)| sum + (pos_ix * id));

	Ok(checksum)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DiskMapPartTwo {
	content: DiskContent,
	length: u64,
	start: u64
}

pub fn part2(input: &str) -> Result<u64> {
	let disk_map = parse(input);

	let disk_map = {
		let mut next_id = 0;
		let mut start_ix = 0;
		disk_map.into_iter().enumerate().map(move |(ix, length)| {
			let content = if ix % 2 == 0 {
				let id = next_id;
				next_id += 1;
				DiskContent::File{ id }
			} else {
				DiskContent::FreeSpace
			};
			let start = start_ix;
			start_ix += length;
			DiskMapPartTwo { content, length, start }
		})
	};

	let (mut file_handles, mut free_blocks) = disk_map.fold((BTreeMap::new(), BTreeMap::new()), |(mut files, mut free_map), mapping| {
		match mapping.content {
			DiskContent::File { id } => {
				files.insert(id, mapping);
			},
			DiskContent::FreeSpace => {
				free_map.insert(mapping.start, mapping.length);
			},
		};
		(files, free_map)
	});

	// println!("file_handles: {:?}", file_handles);
	for (_, file_mapping) in file_handles.iter_mut().rev() {
		let file_length = file_mapping.length;
		if let Some((&free_start, &free_length)) = free_blocks.iter().find(|(_, &length)| length >= file_length) {
			if free_start < file_mapping.start {
				free_blocks.remove(&free_start);
				file_mapping.start = free_start;

				let remaining_free_len = free_length - file_length;
				if remaining_free_len > 0 {
					free_blocks.insert(free_start + file_length, remaining_free_len);
				}
			}
		}
	}
	// println!("file_handles: {:?}", file_handles);

	let checksum = file_handles.into_iter().fold(0, |sum, (id, mapping)| {
		let range = mapping.start..(mapping.start + mapping.length);
		let range_sum: u64 = range.sum();
		sum + (range_sum * id)
	});

	Ok(checksum)
}

#[cfg(test)]
mod tests {
	use crate::day09::*;

	const TEST: &str = "2333133121414131402";

	#[test]
	fn test_part_one() -> Result<()> {
		assert_eq!(1928, part1(TEST)?);
		Ok(())
	}

	#[test]
	fn test_part_two() -> Result<()> {
		assert_eq!(2858, part2(TEST)?);
		Ok(())
	}
}
