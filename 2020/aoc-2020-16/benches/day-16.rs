use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use aoc_2020_16::{parse_input, part_one, part_two, DEFAULT_INPUT_PATH};
use aoc_2020_common::input::load_input;

criterion_group!(benches, input_parsing, part_one, part_two);
criterion_main!(benches);

fn input_parsing(c: &mut Criterion) {
    let input = load_input(DEFAULT_INPUT_PATH);

    c.bench_with_input(BenchmarkId::new("day-16-parsing", ""), &input, |b, i| {
        b.iter(|| black_box(parse_input(i)))
    });
}

fn part_one(c: &mut Criterion) {
    let input = load_input(DEFAULT_INPUT_PATH);
    let input = parse_input(&input);

    c.bench_with_input(BenchmarkId::new("day-16-p01", ""), &input, |b, i| {
        b.iter(|| {
            let mut input = i.clone();
            black_box(part_one::solve(&mut input))
        });
    });
}

fn part_two(c: &mut Criterion) {
    let input = load_input(DEFAULT_INPUT_PATH);
    let mut input = parse_input(&input);

    //remove invalid
    part_one::solve(&mut input);

    c.bench_with_input(BenchmarkId::new("day-16-p02-v1", ""), &input, |b, i| {
        b.iter(|| black_box(part_two::solve_v1(i)));
    });

    c.bench_with_input(BenchmarkId::new("day-16-p02-v2", ""), &input, |b, i| {
        b.iter(|| black_box(part_two::solve_v2(i)));
    });
}
