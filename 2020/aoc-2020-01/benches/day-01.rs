use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use aoc_2020_01::{part_one, part_two, DAY};
use aoc_2020_common::input::{default_test_input, load_input};
use aoc_2020_common::parsing::parse_line_delimited;

criterion_group!(benches, part_one, part_two);
criterion_main!(benches);

fn part_one(c: &mut Criterion) {
    let input = load_input(default_test_input(DAY));
    let input = parse_line_delimited(&input);

    c.bench_with_input(
        BenchmarkId::new("day-01-p01-bruteforce", ""),
        &input,
        |b, i| b.iter(|| black_box(part_one::solve_bruteforce(i))),
    );

    c.bench_with_input(
        BenchmarkId::new("day-01-p01-with-sorting", ""),
        &input,
        |b, i| b.iter(|| black_box(part_one::solve_with_sorting(&i))),
    );
}

fn part_two(c: &mut Criterion) {
    let input = load_input(default_test_input(DAY));
    let input = parse_line_delimited(&input);

    c.bench_with_input(
        BenchmarkId::new("day-01-p02-bruteforce", ""),
        &input,
        |b, i| b.iter(|| black_box(part_two::solve_with_bruteforce(i))),
    );

    c.bench_with_input(
        BenchmarkId::new("day-01-p02-with-sorting", ""),
        &input,
        |b, i| b.iter(|| black_box(part_two::solve_with_quadratic_alg(&i))),
    );
}
