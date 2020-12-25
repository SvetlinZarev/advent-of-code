use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use aoc_2020_12::{part_one, part_two, DEFAULT_INPUT_PATH};
use aoc_2020_common::input::load_input;

criterion_group!(benches, input_parsing, part_one, part_two);
criterion_main!(benches);

fn input_parsing(c: &mut Criterion) {
    let input = load_input(DEFAULT_INPUT_PATH);

    c.bench_with_input(
        BenchmarkId::new("day-12-p01-parsing", ""),
        &input,
        |b, i| b.iter(|| black_box(part_one::parse_input_data(i))),
    );

    c.bench_with_input(
        BenchmarkId::new("day-12-p02-parsing", ""),
        &input,
        |b, i| b.iter(|| black_box(part_two::parse_input_data(i))),
    );
}

fn part_one(c: &mut Criterion) {
    let input = load_input(DEFAULT_INPUT_PATH);
    let input = part_one::parse_input_data(&input);

    c.bench_with_input(BenchmarkId::new("day-12-p01", ""), &input, |b, i| {
        b.iter(|| black_box(part_one::solve(i)))
    });
}

fn part_two(c: &mut Criterion) {
    let input = load_input(DEFAULT_INPUT_PATH);
    let input = part_two::parse_input_data(&input);

    c.bench_with_input(BenchmarkId::new("day-12-p02", ""), &input, |b, i| {
        b.iter(|| black_box(part_two::solve(i)))
    });
}
