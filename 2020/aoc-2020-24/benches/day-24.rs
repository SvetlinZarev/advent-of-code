use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use aoc_2020_24::{parse_input, part_one, part_two, DAY};
use aoc_2020_common::input::{default_test_input, load_input};

criterion_group!(benches, bench_parse_input, part_one, part_two);
criterion_main!(benches);

fn bench_parse_input(c: &mut Criterion) {
    let input = load_input(default_test_input(DAY));

    c.bench_with_input(BenchmarkId::new("day-24-parsing", ""), &input, |b, i| {
        b.iter(|| black_box(parse_input(i)))
    });
}

fn part_one(c: &mut Criterion) {
    let input = load_input(default_test_input(DAY));
    let input = parse_input(&input);

    c.bench_with_input(BenchmarkId::new("day-24-p01", ""), &input, |b, i| {
        b.iter(|| black_box(part_one::solve(i)))
    });
}

fn part_two(c: &mut Criterion) {
    let input = load_input(default_test_input(DAY));
    let input = parse_input(&input);
    let (_, tiles) = part_one::solve(&input);

    c.bench_with_input(BenchmarkId::new("day-24-p02", ""), &tiles, |b, i| {
        b.iter(|| black_box(part_two::solve(i)))
    });
}
