use criterion::{criterion_group, criterion_main, Criterion};
use aoc_24::start_day;

use aoc_24::day16::*;

pub fn bench(c: &mut Criterion) {
	let input_file = start_day("16").unwrap();
	let input = input_file.as_str();
	let mut group = c.benchmark_group("day16");

	group.bench_function("part1", |b| b.iter(|| part1(input)));
	group.bench_function("part2", |b| b.iter(|| part2(input)));
}

criterion_group!(benches, bench);
criterion_main!(benches);
