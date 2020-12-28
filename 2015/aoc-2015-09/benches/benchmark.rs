use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use aoc_2015_09::{parse_input, solve_part_one, DAY};
use aoc_2015_common::input::{default_test_input, load_input};

criterion_group!(benches, input_parsing, bench_solver);
criterion_main!(benches);

fn input_parsing(c: &mut Criterion) {
    let input = load_input(default_test_input(DAY));

    c.bench_with_input(BenchmarkId::new("day-09", ""), &input, |b, i| {
        b.iter(|| black_box(parse_input(i)))
    });
}

fn bench_solver(c: &mut Criterion) {
    let input = load_input(default_test_input(DAY));
    let input = parse_input(&input);

    c.bench_with_input(BenchmarkId::new("day-09: O(n!)", ""), &input, |b, i| {
        b.iter(|| black_box(solve_part_one(i)))
    });
}
