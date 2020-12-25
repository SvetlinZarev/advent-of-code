use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use aoc_2020_01::{part_one, part_two, DEFAULT_INPUT_PATH};
use aoc_2020_common::input::load_input;
use aoc_2020_common::parsing::parse_lines_as_i32;

criterion_group!(benches, part_one, part_two);
criterion_main!(benches);

fn part_one(c: &mut Criterion) {
    let input = load_input(DEFAULT_INPUT_PATH);
    let input = parse_lines_as_i32(&input);

    c.bench_with_input(
        BenchmarkId::new("day-01-p01-bruteforce", ""),
        &input,
        |b, i| b.iter(|| black_box(part_one::solve_bruteforce(i))),
    );

    c.bench_with_input(
        BenchmarkId::new("day-01-p01-with-sorting", ""),
        &input,
        |b, i| b.iter(|| black_box(part_one::solve_with_sorting(&mut i.clone()))),
    );
}

fn part_two(c: &mut Criterion) {
    let input = load_input(DEFAULT_INPUT_PATH);
    let input = parse_lines_as_i32(&input);

    c.bench_with_input(
        BenchmarkId::new("day-01-p02-bruteforce", ""),
        &input,
        |b, i| b.iter(|| black_box(part_two::solve_with_bruteforce(i))),
    );

    c.bench_with_input(
        BenchmarkId::new("day-01-p02-with-sorting", ""),
        &input,
        |b, i| b.iter(|| black_box(part_two::solve_with_quadratic_alg(&mut i.clone()))),
    );
}
