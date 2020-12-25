use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use aoc_2020_25::{solve_v1, solve_v2, solve_v3, DEFAULT_INPUT_PATH};
use aoc_2020_common::input::load_input;
use aoc_2020_common::parsing::parse_lines_as_usize;

criterion_group!(benches, part_one);
criterion_main!(benches);

fn part_one(c: &mut Criterion) {
    let input = load_input(DEFAULT_INPUT_PATH);
    let input = parse_lines_as_usize(&input);

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
