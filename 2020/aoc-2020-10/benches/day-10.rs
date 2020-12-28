use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use aoc_2020_10::{part_one, part_two, DAY};
use aoc_2020_common::input::{default_test_input, load_input};
use aoc_2020_common::parsing::parse_line_delimited;

criterion_group!(benches, part_one, part_two);
criterion_main!(benches);

fn part_one(c: &mut Criterion) {
    let input = load_input(default_test_input(DAY));
    let input = parse_line_delimited(&input);

    c.bench_with_input(BenchmarkId::new("day-10-p01-v1", ""), &input, |b, i| {
        b.iter(|| black_box(part_one::solve_v1(&mut i.clone())))
    });

    c.bench_with_input(BenchmarkId::new("day-10-p01-v2", ""), &input, |b, i| {
        b.iter(|| black_box(part_one::solve_v2(&mut i.clone())))
    });
}

fn part_two(c: &mut Criterion) {
    let input = load_input(default_test_input(DAY));
    let input = parse_line_delimited(&input);

    c.bench_with_input(
        BenchmarkId::new("day-10-p02-v1-O(N)", ""),
        &input,
        |b, i| b.iter(|| black_box(part_two::solve_v1(&mut i.clone()))),
    );

    c.bench_with_input(
        BenchmarkId::new("day-10-p02-v2-O(1)", ""),
        &input,
        |b, i| b.iter(|| black_box(part_two::solve_v2_const_mem(&mut i.clone()))),
    );
}
