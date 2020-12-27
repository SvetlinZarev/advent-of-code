use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use aoc_2020_19::{modify_input, parse_input, solve, DAY};
use aoc_2020_common::input::{default_test_input, load_input};

criterion_group!(benches, input_parsing, part_one, part_two);
criterion_main!(benches);

fn input_parsing(c: &mut Criterion) {
    let input = load_input(default_test_input(DAY));

    c.bench_with_input(BenchmarkId::new("day-19-parsing", ""), &input, |b, i| {
        b.iter(|| black_box(parse_input(i)))
    });
}

fn part_one(c: &mut Criterion) {
    let input = load_input(default_test_input(DAY));
    let input = parse_input(&input);

    c.bench_with_input(BenchmarkId::new("day-19-p01", ""), &input, |b, i| {
        let (rules, msgs) = i;
        b.iter(|| black_box(solve(rules, msgs)))
    });
}

fn part_two(c: &mut Criterion) {
    let input = load_input(default_test_input(DAY));
    let (mut rules, msgs) = parse_input(&input);
    modify_input(&mut rules);

    c.bench_with_input(
        BenchmarkId::new("day-19-p02", ""),
        &(rules, msgs),
        |b, i| {
            let (rules, msgs) = i;
            b.iter(|| black_box(solve(rules, msgs)))
        },
    );
}
