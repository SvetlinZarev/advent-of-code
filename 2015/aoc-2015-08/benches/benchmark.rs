use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use aoc_2015_08::{solve_part_one, solve_part_two, DAY};
use aoc_2015_common::input::{default_test_input, load_input};

criterion_group!(benches, part_one, part_two);
criterion_main!(benches);

fn part_one(c: &mut Criterion) {
    let input = load_input(default_test_input(DAY));

    c.bench_with_input(BenchmarkId::new("day-08-p1", ""), &input, |b, i| {
        b.iter(|| black_box(solve_part_one(i)))
    });
}

fn part_two(c: &mut Criterion) {
    let input = load_input(default_test_input(DAY));

    c.bench_with_input(BenchmarkId::new("day-08-p2", ""), &input, |b, i| {
        b.iter(|| black_box(solve_part_two(i)))
    });
}
