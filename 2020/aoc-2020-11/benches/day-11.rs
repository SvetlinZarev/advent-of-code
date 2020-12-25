use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use aoc_2020_11::{
    parse_input, part_one_v1, part_one_v2, part_one_v3, part_two, DEFAULT_INPUT_PATH,
};
use aoc_2020_common::input::load_input;

criterion_group!(benches, input_parsing, part_one, part_two);
criterion_main!(benches);

fn input_parsing(c: &mut Criterion) {
    let input = load_input(DEFAULT_INPUT_PATH);

    c.bench_with_input(
        BenchmarkId::new("day-11-p01-parsing", ""),
        &input,
        |b, i| b.iter(|| black_box(parse_input(i))),
    );

    c.bench_with_input(
        BenchmarkId::new("day-11-p01-v3-parsing", ""),
        &input,
        |b, i| b.iter(|| black_box(part_one_v3::parse_input(i))),
    );
}

fn part_one(c: &mut Criterion) {
    let raw_input = load_input(DEFAULT_INPUT_PATH);
    let input = parse_input(&raw_input);

    c.bench_with_input(BenchmarkId::new("day-11-p01-v1", ""), &input, |b, i| {
        b.iter(|| black_box(part_one_v1::solve(&mut i.clone())))
    });

    c.bench_with_input(BenchmarkId::new("day-11-p01-v2", ""), &input, |b, i| {
        b.iter(|| black_box(part_one_v2::solve(&mut i.clone())))
    });

    let input = part_one_v3::parse_input(&raw_input);
    c.bench_with_input(BenchmarkId::new("day-11-p01-v3", ""), &input, |b, i| {
        b.iter(|| black_box(part_one_v3::solve(&mut i.clone())))
    });
}

fn part_two(c: &mut Criterion) {
    let input = load_input(DEFAULT_INPUT_PATH);
    let input = parse_input(&input);

    c.bench_with_input(BenchmarkId::new("day-11-p02", ""), &input, |b, i| {
        b.iter(|| black_box(part_two::solve(&mut i.clone())))
    });
}
