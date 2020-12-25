use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use aoc_2020_23::{parse_input, part_one, part_two, DEFAULT_INPUT_PATH};
use aoc_2020_common::input::load_input;

criterion_group!(benches, part_one, part_two);
criterion_main!(benches);

fn part_one(c: &mut Criterion) {
    let input = load_input(DEFAULT_INPUT_PATH);
    let input = parse_input(&input);

    c.bench_with_input(BenchmarkId::new("day-23-p01", ""), &input, |b, i| {
        b.iter(|| black_box(part_one::solve(i)))
    });
}

fn part_two(c: &mut Criterion) {
    let input = load_input(DEFAULT_INPUT_PATH);
    let input = parse_input(&input);

    c.bench_with_input(BenchmarkId::new("day-23-p02", ""), &input, |b, i| {
        b.iter(|| black_box(part_two::solve(i)))
    });
}
