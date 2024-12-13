use std::cell::Cell;
use std::collections::BTreeSet;
use ahash::AHashSet;
use anyhow::*;
use grid::Grid;
use itertools::Itertools;
use nom::bytes::complete::take_while1;
use nom::character::complete::line_ending;
use nom::combinator::{all_consuming, map, opt};
use nom::{Finish, IResult};
use nom::multi::many1;
use nom::sequence::terminated;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
	North,
	East,
	South,
	West,
}
impl Direction {
	pub const fn get_offset(&self) -> (isize, isize) {
		match self {
			Direction::North => (-1, 0),
			Direction::East => (0, 1),
			Direction::South => (1, 0),
			Direction::West => (0, -1),
		}
	}

	pub fn offset_from(&self, (row, col): (impl TryInto<isize>, impl TryInto<isize>)) -> Option<(isize, isize)> {
		let row_isize = row.try_into().ok()?;
		let col_isize = col.try_into().ok()?;
		let (row_offset, col_offset) = self.get_offset();
		Some((row_isize + row_offset, col_isize + col_offset))
	}

	pub const fn values() -> [Direction; 4] {
		[
			Direction::North,
			Direction::East,
			Direction::South,
			Direction::West
		]
	}
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct MapCell {
	plant: char,
	region: Cell<Option<usize>>,
}
impl MapCell {
	pub fn new(plant: char) -> Self {
		Self { plant, region: Cell::new(None) }
	}
}

fn parse_line(input: &str) -> IResult<&str, Vec<MapCell>> {
	terminated(
		map(
			take_while1(|chr: char| chr.is_ascii_alphabetic()),
			|line: &str| line.chars().map(MapCell::new).collect_vec()
		),
		opt(line_ending)
	)(input)
}

fn parse(input: &str) -> Vec<Vec<MapCell>> {
	let (_, result) = all_consuming(many1(parse_line))(input).finish().unwrap();
	result
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RegionInfo {
	id: usize,
	area: u64,
	perimeter: u64
}
impl RegionInfo {
	// pub fn new(id: usize) -> Self {
	// 	Self { id, area: 0, perimeter: 0 }
	// }
	pub fn with_size(id: usize, area: u64, perimeter: u64) -> Self {
		Self { id, area, perimeter }
	}
}

pub fn part1(input: &str) -> Result<u64> {
	let grid: Grid<_> = parse(input).into();

	let mut regions = Vec::new();
	grid.indexed_iter().for_each(|((grid_row, grid_col), grid_cell)| {
		let (grid_row, grid_col) = (grid_row.try_into().unwrap(), grid_col.try_into().unwrap());
		if grid_cell.region.get().is_none() {

			let region_id = regions.len();
			let region_plant = grid_cell.plant;
			let mut region_cells = AHashSet::from([(grid_row, grid_col)]);
			let mut region_cells_queue = BTreeSet::from([(grid_row, grid_col)]);

			while let Some((row, col)) = region_cells_queue.pop_first() {
				// does this cell exist? is it already assigned a region? is it the right plant?
				if let Some(next_cell) = grid.get(row, col) {
					if next_cell.region.get().is_none() && next_cell.plant == region_plant {
						// add this cell to the current region
						next_cell.region.set(Some(region_id));
						region_cells.insert((row, col));

						// add neighboring cells to queue
						region_cells_queue.extend(Direction::values().into_iter()
							.flat_map(|dir| dir.offset_from((row, col))));
					}
				}
			}

			let region_area = region_cells.len();
			let region_perimeter = region_cells.iter().map(|&(region_row, region_col)| {
				4 - Direction::values().into_iter()
					.flat_map(|dir| dir.offset_from((region_row, region_col)))
					.filter_map(|(nbr_row, nbr_col)| region_cells.get(&(nbr_row, nbr_col)))
					.count() as u64
			}).sum();

			regions.push(RegionInfo::with_size(region_id, region_area as u64, region_perimeter));
		}
	});

	Ok(regions.into_iter().map(|region| region.area * region.perimeter).sum())
}

pub fn part2(input: &str) -> Result<u64> {
	let _ = input;
	Ok(0)
}

#[cfg(test)]
mod tests {
	use crate::day12::*;

	const TEST: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

	#[test]
	fn test_part_one() -> Result<()> {
		assert_eq!(1930, part1(TEST)?);
		Ok(())
	}

	#[test]
	fn test_part_two() -> Result<()> {
		assert_eq!(1206, part2(TEST)?);
		Ok(())
	}
}
