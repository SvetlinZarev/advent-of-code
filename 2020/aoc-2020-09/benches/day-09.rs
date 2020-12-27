use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use aoc_2020_09::{part_one, part_two, DAY};
use aoc_2020_common::input::{default_test_input, load_input};
use aoc_2020_common::parsing::parse_lines_as_u64;

criterion_group!(benches, part_one, part_two);
criterion_main!(benches);

fn part_one(c: &mut Criterion) {
    let input = load_input(default_test_input(DAY));
    let input = parse_lines_as_u64(&input);

    c.bench_with_input(BenchmarkId::new("day-09-p01", ""), &input, |b, i| {
        b.iter(|| black_box(part_one::solve(i)))
    });
}

fn part_two(c: &mut Criterion) {
    let input = load_input(default_test_input(DAY));
    let input = parse_lines_as_u64(&input);

    let key = part_one::solve(&input).unwrap();

    c.bench_with_input(BenchmarkId::new("day-09-p02", ""), &input, |b, i| {
        b.iter(|| black_box(part_two::solve(i, key)))
    });
}
