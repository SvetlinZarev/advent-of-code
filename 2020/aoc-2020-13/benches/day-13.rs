use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use aoc_2020_13::{part_one, part_two, DEFAULT_INPUT_PATH};
use aoc_2020_common::input::load_input;

criterion_group!(benches, part_one, part_two);
criterion_main!(benches);

fn part_one(c: &mut Criterion) {
    let input = load_input(DEFAULT_INPUT_PATH);
    let (arrival, schedule) = part_one::parse_input_data(&input);

    c.bench_with_input(
        BenchmarkId::new("day-13-p01", ""),
        &(arrival, &schedule),
        |b, (arrival, schedule)| b.iter(|| black_box(part_one::solve(*arrival, schedule))),
    );
}

fn part_two(c: &mut Criterion) {
    let input = load_input(DEFAULT_INPUT_PATH);
    let input = part_two::parse_input_data(&input);

    c.bench_with_input(BenchmarkId::new("day-13-p02", ""), &input, |b, i| {
        b.iter(|| black_box(part_two::solve(i)))
    });
}
