use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use aoc_2020_25::{solve_v1, solve_v2, solve_v3, DAY};
use aoc_2020_common::input::{default_test_input, load_input};
use aoc_2020_common::parsing::parse_line_delimited;

criterion_group!(benches, part_one);
criterion_main!(benches);

fn part_one(c: &mut Criterion) {
    let input = load_input(default_test_input(DAY));
    let input = parse_line_delimited(&input);

    c.bench_with_input(BenchmarkId::new("day-25-p01-v1", ""), &input, |b, i| {
        b.iter(|| black_box(solve_v1(i)))
    });

    c.bench_with_input(BenchmarkId::new("day-25-p01-v2", ""), &input, |b, i| {
        b.iter(|| black_box(solve_v2(i)))
    });

    c.bench_with_input(BenchmarkId::new("day-25-p01-v3", ""), &input, |b, i| {
        b.iter(|| black_box(solve_v3(i)))
    });
}
