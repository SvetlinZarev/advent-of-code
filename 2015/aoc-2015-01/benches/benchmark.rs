use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use aoc_2015_01::{part_one, part_two, DAY};
use aoc_2015_common::input::{default_test_input, load_input};

criterion_group!(benches, part_one, part_two);
criterion_main!(benches);

fn part_one(c: &mut Criterion) {
    let input = load_input(default_test_input(DAY));
    let input = input.trim().as_bytes();

    c.bench_with_input(BenchmarkId::new("day-01-p01", ""), &input, |b, i| {
        b.iter(|| black_box(part_one::solve(i)))
    });
}

fn part_two(c: &mut Criterion) {
    let input = load_input(default_test_input(DAY));
    let input = input.trim().as_bytes();

    c.bench_with_input(BenchmarkId::new("day-01-p02", ""), &input, |b, i| {
        b.iter(|| black_box(part_two::solve(i)))
    });
}
