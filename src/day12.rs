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
			let region_perimeter = region_cells.iter().map(|&(region_row, region_col)|
				4 - Direction::values().into_iter()
					.flat_map(|dir| dir.offset_from((region_row, region_col)))
					.filter_map(|(nbr_row, nbr_col)| region_cells.get(&(nbr_row, nbr_col)))
					.count() as u64
			).sum();

			regions.push(RegionInfo::with_size(region_id, region_area as u64, region_perimeter));
		}
	});

	Ok(regions.into_iter().map(|region| region.area * region.perimeter).sum())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RegionInfoPartTwo {
	id: usize,
	area: u64,
	sides: u64
}
impl RegionInfoPartTwo {
	pub fn with_size(id: usize, area: u64, sides: u64) -> Self {
		Self { id, area, sides }
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Edge {
	Horizontal{ col: isize, inside_row: isize, outside_row: isize },
	Vertical{ row: isize, inside_col: isize, outside_col: isize },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ContinuousEdge {
	Horizontal{ start_col: isize, end_col: isize, inside_row: isize, outside_row: isize },
	Vertical{ start_row: isize, end_row: isize, inside_col: isize, outside_col: isize }
}

pub fn part2(input: &str) -> Result<u64> {
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
			let edges = region_cells.iter().sorted_unstable().flat_map(|&(region_row, region_col)| {
				// for this cell, find neighboring cells that are not part of this region
				Direction::values().into_iter()
					.flat_map(move |dir| dir.offset_from((region_row, region_col)))
					.filter(|&(nbr_row, nbr_col)| region_cells.get(&(nbr_row, nbr_col)).is_none())
					// and find the edges
					.map(move |(nbr_row, nbr_col)| {
						if nbr_row == region_row {
							Edge::Vertical { row: region_row, inside_col: region_col, outside_col: nbr_col }
						} else {
							Edge::Horizontal { col: region_col, inside_row: region_row, outside_row: nbr_row }
						}
					})
			}).collect_vec();
			let distinct_edges = edges.into_iter().fold(Vec::new(), |mut edge_runs, next_edge| {
				if let Some(matching_edge_ix) = edge_runs.iter().position(|&edge_run| {
					// are these the same kind of edge? are they adjacent?
					match (edge_run, next_edge) {
						(ContinuousEdge::Horizontal { start_col, end_col, inside_row: run_inside_row, outside_row: run_outside_row },
							Edge::Horizontal { col: edge_col, inside_row: edge_inside_row, outside_row: edge_outside_row }) => {
							edge_inside_row == run_inside_row && edge_outside_row == run_outside_row && (edge_col == start_col - 1 || edge_col == end_col + 1)
						},
						(ContinuousEdge::Vertical { start_row, end_row, inside_col: run_inside_col, outside_col: run_outside_col },
							Edge::Vertical { row: edge_row, inside_col: edge_inside_col, outside_col: edge_outside_col }) => {
							edge_inside_col == run_inside_col && edge_outside_col == run_outside_col && (edge_row == start_row - 1 || edge_row == end_row + 1)
						},
						_ => false,
					}
				}) {
					match (edge_runs.get_mut(matching_edge_ix).unwrap(), next_edge) {
						(ContinuousEdge::Horizontal { start_col, end_col, .. }, Edge::Horizontal { col: edge_col, .. }) => {
							*start_col = edge_col.min(*start_col);
							*end_col = edge_col.max(*end_col);
						},
						(ContinuousEdge::Vertical { start_row, end_row, .. }, Edge::Vertical { row: edge_row, .. }) => {
							*start_row = edge_row.min(*start_row);
							*end_row = edge_row.max(*end_row);
						},
						_ => {panic!("found matching edge but second pattern match failed!");}
					};
				} else {
					let new_edge_run = match next_edge {
						Edge::Horizontal { col, inside_row, outside_row } =>
							ContinuousEdge::Horizontal { start_col: col, end_col: col, inside_row, outside_row },
						Edge::Vertical { row, inside_col, outside_col } =>
							ContinuousEdge::Vertical { start_row: row, end_row: row, inside_col, outside_col }
					};
					edge_runs.push(new_edge_run);
				}
				edge_runs
			}).len();

			regions.push(RegionInfoPartTwo::with_size(region_id, region_area as u64, distinct_edges as u64));
		}
	});

	Ok(regions.into_iter().map(|region| region.area * region.sides).sum())
}

#[cfg(test)]
mod tests {
	use crate::day12::*;

	const SMALL_TEST: &str = "AAAA
BBCD
BBCC
EEEC";

	const MEDIUM_TEST: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
	
	const LARGE_TEST: &str = "RRRRIICCFF
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
	fn test_part_one_small() -> Result<()> {
		assert_eq!(140, part1(SMALL_TEST)?);
		Ok(())
	}

	#[test]
	fn test_part_one_medium() -> Result<()> {
		assert_eq!(772, part1(MEDIUM_TEST)?);
		Ok(())
	}

	#[test]
	fn test_part_one_large() -> Result<()> {
		assert_eq!(1930, part1(LARGE_TEST)?);
		Ok(())
	}



	#[test]
	fn test_part_two_small() -> Result<()> {
		assert_eq!(80, part2(SMALL_TEST)?);
		Ok(())
	}

	#[test]
	fn test_part_two_medium() -> Result<()> {
		assert_eq!(436, part2(MEDIUM_TEST)?);
		Ok(())
	}

	#[test]
	fn test_part_two_E() -> Result<()> {
		assert_eq!(236, part2("EEEEE
EXXXX
EEEEE
EXXXX
EEEEE")?);
		Ok(())
	}

	#[test]
	fn test_part_two_mobius() -> Result<()> {
		assert_eq!(368, part2("AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA")?);
		Ok(())
	}

	#[test]
	fn test_part_two_large() -> Result<()> {
		assert_eq!(1206, part2(LARGE_TEST)?);
		Ok(())
	}
}
