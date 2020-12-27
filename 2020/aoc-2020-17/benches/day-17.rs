use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use aoc_2020_17::{part_one, part_two, DAY};
use aoc_2020_common::input::{default_test_input, load_input};

criterion_group!(benches, input_parsing, part_one, part_two);
criterion_main!(benches);

fn input_parsing(c: &mut Criterion) {
    let input = load_input(default_test_input(DAY));

    c.bench_with_input(
        BenchmarkId::new("day-17-p01-parsing", ""),
        &input,
        |b, i| b.iter(|| black_box(part_one::parse_input(&i))),
    );

    c.bench_with_input(
        BenchmarkId::new("day-17-p02-parsing", ""),
        &input,
        |b, i| b.iter(|| black_box(part_two::parse_input(&i))),
    );
}

fn part_one(c: &mut Criterion) {
    let input = load_input(default_test_input(DAY));
    let input = part_one::parse_input(&input);

    c.bench_with_input(BenchmarkId::new("day-17-p01", ""), &input, |b, i| {
        b.iter(|| black_box(part_one::solve(&mut i.clone())))
    });
}

fn part_two(c: &mut Criterion) {
    let input = load_input(default_test_input(DAY));
    let input = part_two::parse_input(&input);

    c.bench_with_input(BenchmarkId::new("day-17-p02", ""), &input, |b, i| {
        b.iter(|| black_box(part_two::solve(&mut i.clone())))
    });
}
