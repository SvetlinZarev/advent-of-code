use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use aoc_2020_19::{parse_input, solve, INPUT_PATH_P1, INPUT_PATH_P2};
use aoc_2020_common::input::load_input;

criterion_group!(benches, input_parsing, part_one, part_two);
criterion_main!(benches);

fn input_parsing(c: &mut Criterion) {
    let input = load_input(INPUT_PATH_P1);

    c.bench_with_input(BenchmarkId::new("day-19-parsing", ""), &input, |b, i| {
        b.iter(|| black_box(parse_input(i)))
    });
}

fn part_one(c: &mut Criterion) {
    let input = load_input(INPUT_PATH_P1);
    let input = parse_input(&input);

    c.bench_with_input(BenchmarkId::new("day-19-p01", ""), &input, |b, i| {
        let (rules, msgs) = i;
        b.iter(|| black_box(solve(rules, msgs)))
    });
}

fn part_two(c: &mut Criterion) {
    let input = load_input(INPUT_PATH_P2);
    let input = parse_input(&input);

    c.bench_with_input(BenchmarkId::new("day-19-p02", ""), &input, |b, i| {
        let (rules, msgs) = i;
        b.iter(|| black_box(solve(rules, msgs)))
    });
}
