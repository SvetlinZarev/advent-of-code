use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use aoc_2020_15::{solve_v1, solve_v2, solve_v3, DAY, MAX_TURNS_PART_ONE};
use aoc_2020_common::input::{default_test_input, load_input};
use aoc_2020_common::parsing::parse_csv;

criterion_group!(benches, part_one);
criterion_main!(benches);

fn part_one(c: &mut Criterion) {
    let input = load_input(default_test_input(DAY));
    let input = parse_csv(&input);

    c.bench_with_input(
        BenchmarkId::new("day-15-p01-v1-vector", ""),
        &input,
        |b, i| b.iter(|| black_box(solve_v1(i, MAX_TURNS_PART_ONE))),
    );

    c.bench_with_input(
        BenchmarkId::new("day-15-p01-v2-vector", ""),
        &input,
        |b, i| b.iter(|| black_box(solve_v2(i, MAX_TURNS_PART_ONE))),
    );

    c.bench_with_input(BenchmarkId::new("day-15-p01-v3-map", ""), &input, |b, i| {
        b.iter(|| black_box(solve_v3(i, MAX_TURNS_PART_ONE)))
    });
}
