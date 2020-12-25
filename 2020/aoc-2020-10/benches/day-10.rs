use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use aoc_2020_10::{part_one, part_two, DEFAULT_INPUT_PATH};
use aoc_2020_common::input::load_input;
use aoc_2020_common::parsing::parse_lines_as_usize;

criterion_group!(benches, part_one, part_two);
criterion_main!(benches);

fn part_one(c: &mut Criterion) {
    let input = load_input(DEFAULT_INPUT_PATH);
    let input = parse_lines_as_usize(&input);

    c.bench_with_input(BenchmarkId::new("day-10-p01-v1", ""), &input, |b, i| {
        b.iter(|| black_box(part_one::solve_v1(&mut i.clone())))
    });

    c.bench_with_input(BenchmarkId::new("day-10-p01-v2", ""), &input, |b, i| {
        b.iter(|| black_box(part_one::solve_v2(&mut i.clone())))
    });
}

fn part_two(c: &mut Criterion) {
    let input = load_input(DEFAULT_INPUT_PATH);
    let input = parse_lines_as_usize(&input);

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
