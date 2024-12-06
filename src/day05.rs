use std::collections::{BTreeMap, BTreeSet};
use std::ops::ControlFlow;
use anyhow::*;
use nom::character::complete::{char, digit1, line_ending};
use nom::combinator::{all_consuming, map, map_res, opt};
use nom::{Finish, IResult};
use nom::multi::{many1, separated_list1};
use nom::sequence::{separated_pair, terminated};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct PageOrder {
    first: u64,
    second: u64,
}
impl PageOrder {
    fn invert(&self) -> Self {
        Self {
            first: self.second,
            second: self.first
        }
    }
}

fn parse_u64(input: &str) -> IResult<&str, u64> {
    map_res(digit1, |num: &str| num.parse())(input)
}

fn parse_page_ordering(input: &str) -> IResult<&str, PageOrder> {
    map(separated_pair(parse_u64, char('|'), parse_u64),
        |(first, second)| PageOrder { first, second })(input)
}

fn parse_page_ordering_line(input: &str) -> IResult<&str, PageOrder> {
    terminated(parse_page_ordering, line_ending)(input)
}

fn parse_page_updates(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(char(','), parse_u64)(input)
}

fn parse_page_update_line(input: &str) -> IResult<&str, Vec<u64>> {
    terminated(parse_page_updates, opt(line_ending))(input)
}

fn parse(input: &str) -> (Vec<PageOrder>, Vec<Vec<u64>>) {
    let (_, result) = all_consuming(separated_pair(
        many1(parse_page_ordering_line),
        line_ending,
        many1(parse_page_update_line)
    ))(input).finish().unwrap();
    result
}

pub fn part1(input: &str) -> Result<u64> {
    let (page_orders, printed_updates) = parse(input);

    // we have a set of page ordering rules, where if both the first and second values are present,
    // then the second value must come after the first in the printed updates
    //
    // for each page number, generate a set of associated page numbers which, if the first page is
    // present in the printed updates, then seeing any of the numbers in the set means this update
    // contains a pair of pages in the wrong order
    let mut invalid_orders: BTreeMap<u64, BTreeSet<u64>> = BTreeMap::new();
    page_orders.into_iter().for_each(|page_order| {
        let invalid_order = page_order.invert();
        invalid_orders.entry(invalid_order.first)
            .or_insert(BTreeSet::new())
            .insert(invalid_order.second);
    });

    let result = printed_updates.into_iter().filter_map(|update_list| {
        let mut invalid_watchlist = BTreeSet::new();
        let valid_update_list = update_list.iter().try_for_each(|updated_page| {
            if invalid_watchlist.contains(updated_page) {
                ControlFlow::Break(())
            } else {
                if let Some(new_invalid_pages) = invalid_orders.get(updated_page) {
                    invalid_watchlist.append(&mut new_invalid_pages.clone());
                }
                ControlFlow::Continue(())
            }
        }).is_continue();

        if valid_update_list {
            let len = update_list.len();
            Some(update_list[(len - 1) / 2])
        } else {
            None
        }
    }).sum();

    Ok(result)
}

pub fn part2(input: &str) -> Result<u64> {
    let (page_orders, printed_updates) = parse(input);

    let mut valid_orders = BTreeMap::new();
    let mut invalid_orders = BTreeMap::new();
    page_orders.into_iter().for_each(|page_order| {
        valid_orders.entry(page_order.first)
            .or_insert(BTreeSet::new())
            .insert(page_order.second);

        let invalid_order = page_order.invert();
        invalid_orders.entry(invalid_order.first)
            .or_insert(BTreeSet::new())
            .insert(invalid_order.second);
    });

    let result = printed_updates.into_iter().filter_map(|update_list| {
        let mut invalid_watchlist = BTreeSet::new();
        let invalid_update_list = update_list.iter().fold(false, |mut invalid_order, updated_page| {
            if invalid_watchlist.contains(updated_page) {
                invalid_order = true;
            }
            if let Some(new_invalid_pages) = invalid_orders.get(updated_page) {
                invalid_watchlist.append(&mut new_invalid_pages.clone());
            }

            invalid_order
        });

        if invalid_update_list {
            let mut list_priority = BTreeMap::new();
            let update_set = BTreeSet::from_iter(update_list);
            update_set.iter().for_each(|&updated_page| {
                let count = valid_orders.get(&updated_page).map(|valid_updates|
                    update_set.intersection(valid_updates).count()
                ).unwrap_or(0);
                list_priority.insert(count, updated_page);
            });

            let len = list_priority.len();
            Some(list_priority[&((len - 1) / 2)])
        } else {
            None
        }
    }).sum();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::day05::*;

    const TEST: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(143, part1(TEST)?);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        assert_eq!(123, part2(TEST)?);
        Ok(())
    }
}
