use std::collections::BTreeSet;
use ahash::{AHashMap, AHashSet};
use anyhow::*;
use itertools::Itertools;
use nom::character::complete::{alpha1, char, line_ending};
use nom::combinator::all_consuming;
use nom::{Finish, IResult};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;

fn parse_line(input: &str) -> IResult<&str, (&str, &str)> {
	separated_pair(
		alpha1,
		char('-'),
		alpha1
	)(input)
}

fn parse(input: &str) -> Vec<(&str, &str)> {
	let (_, result) = all_consuming(separated_list1(line_ending, parse_line))(input).finish().unwrap();
	result
}

pub fn part1(input: &str) -> Result<usize> {
	let connection_pairs = parse(input);

	let connection_map = connection_pairs.into_iter().fold(AHashMap::new(), |mut connection_map, (node_one, node_two)| {
		connection_map.entry(node_one)
			.and_modify(|node_one_entry: &mut BTreeSet<_>| { node_one_entry.insert(node_two); })
			.or_insert(BTreeSet::from([node_two]));
		connection_map.entry(node_two)
			.and_modify(|node_two_entry| { node_two_entry.insert(node_one); })
			.or_insert(BTreeSet::from([node_one]));

		connection_map
	});

	let nodes_to_check = connection_map.iter().filter(|(node_name, _)| node_name.starts_with('t'));
	let three_member_connections = nodes_to_check.into_iter().fold(AHashSet::new(), |three_nodes_set, (curr_node, curr_neighbors)| {
		// for any given node, iterate through its list of neighbors
		// any nodes present in both the initial list of neighbors and the neighbors list of neighbors
		// are connected to both nodes selected
		curr_neighbors.iter().fold(three_nodes_set, |three_nodes_set, nbr_node| {
			let nbr_neighbors = connection_map.get(nbr_node).unwrap();
			let shared_neighbors = curr_neighbors.intersection(nbr_neighbors);
			shared_neighbors.fold(three_nodes_set, |mut three_nodes_set, shared_nbr| {
				three_nodes_set.insert(BTreeSet::from([curr_node, nbr_node, shared_nbr]));
				three_nodes_set
			})
		})
	});

	Ok(three_member_connections.len())
}

pub fn part2(input: &str) -> Result<String> {
	let connection_pairs = parse(input);

	let connection_map = connection_pairs.into_iter().fold(AHashMap::new(), |mut connection_map, (node_one, node_two)| {
		connection_map.entry(node_one)
			.and_modify(|node_one_entry: &mut BTreeSet<_>| { node_one_entry.insert(node_two); })
			.or_insert(BTreeSet::from([node_two]));
		connection_map.entry(node_two)
			.and_modify(|node_two_entry| { node_two_entry.insert(node_one); })
			.or_insert(BTreeSet::from([node_one]));

		connection_map
	});

	let max_clique = |set: &BTreeSet<_>| {
		fn bron_kerbosch(r: BTreeSet<&str>, p: BTreeSet<&str>, x: BTreeSet<&str>, connection_map: &AHashMap<&str, BTreeSet<&str>>) -> BTreeSet<BTreeSet<String>> {
			if p.is_empty() && x.is_empty() {
				BTreeSet::from([BTreeSet::from_iter(r.into_iter().map(|str_slice| str_slice.to_string()))])
			} else {
				// let (pivot, pivot_neighbors) = p.union(&x)
				// 	.map(|key| (key, connection_map.get(key).unwrap()))
				// 	.min_by_key(|(_, nbr_set)| nbr_set.len()).unwrap();
				// let rec_call = p.clone().into_iter().filter(|&key| !pivot_neighbors.contains(key))
				// 	.try_fold((p, x), |(mut p, mut x), key| {
				// 		let new_r = BTreeSet::from_iter(r.union(&BTreeSet::from([key])).cloned());
				// 		let new_p = BTreeSet::from_iter(p.intersection(connection_map.get(key).unwrap()).cloned());
				// 		let new_x = BTreeSet::from_iter(x.intersection(connection_map.get(key).unwrap()).cloned());
				// 		let result = bron_kerbosch(new_r, new_p, new_x, connection_map);
				// 		if let Some(result) = result {
				// 			ControlFlow::Break(result)
				// 		} else {
				// 			p.remove(key);
				// 			x.insert(key);
				// 			ControlFlow::Continue((p, x))
				// 		}
				// });

				let (_, _, result) = p.clone().into_iter()
					.fold((p, x, BTreeSet::default()), |(mut p, mut x, mut results), key| {
						let new_r = BTreeSet::from_iter(r.union(&BTreeSet::from([key])).cloned());
						let new_p = BTreeSet::from_iter(p.intersection(connection_map.get(key).unwrap()).cloned());
						let new_x = BTreeSet::from_iter(x.intersection(connection_map.get(key).unwrap()).cloned());
						bron_kerbosch(new_r, new_p, new_x, connection_map).into_iter().for_each(|result| {
							results.insert(result);
						});

						p.remove(key);
						x.insert(key);

						(p, x, results)
					});
				result
			}
		}

		bron_kerbosch(BTreeSet::default(), set.clone(), BTreeSet::default(), &connection_map)
	};

	let all_nodes = BTreeSet::from_iter(connection_map.keys().cloned());
	let max_clique_set = max_clique(&all_nodes);
	// println!("max_clique_set: {max_clique_set:?}");
	let maximum_clique = max_clique_set.into_iter().max_by_key(|clique| clique.len()).unwrap();
	// println!("maximum_clique: {maximum_clique:?}");

	Ok(maximum_clique.into_iter().join(","))
}

#[cfg(test)]
mod tests {
	use crate::day23::*;

	const TEST: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

	#[test]
	fn test_part_one() -> Result<()> {
		assert_eq!(7, part1(TEST)?);
		Ok(())
	}

	#[test]
	fn test_part_two() -> Result<()> {
		assert_eq!("co,de,ka,ta", part2(TEST)?);
		Ok(())
	}
}
