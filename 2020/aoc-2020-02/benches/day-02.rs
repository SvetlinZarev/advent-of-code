use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use aoc_2020_02::{part_one, part_two, DAY};
use aoc_2020_common::input::{default_test_input, load_input};

criterion_group!(benches, part_one, part_two);
criterion_main!(benches);

fn part_one(c: &mut Criterion) {
    let input = load_input(default_test_input(DAY));

    c.bench_with_input(BenchmarkId::new("day-02-p01", ""), &input, |b, i| {
        b.iter(|| black_box(part_one::solve(i)))
    });
}

fn part_two(c: &mut Criterion) {
    let input = load_input(default_test_input(DAY));

    c.bench_with_input(BenchmarkId::new("day-02-p02", ""), &input, |b, i| {
        b.iter(|| black_box(part_two::solve(i)))
    });
}
