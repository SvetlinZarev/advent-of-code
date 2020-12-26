use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use aoc_2015_07::{parse_input, solve, DAY};
use aoc_2015_common::input::{default_test_input, load_input};

criterion_group!(benches, part_one);
criterion_main!(benches);

fn part_one(c: &mut Criterion) {
    let input = load_input(default_test_input(DAY));
    let input = parse_input(&input);

    c.bench_with_input(BenchmarkId::new("day-07", ""), &input, |b, i| {
        b.iter(|| black_box(solve(i)))
    });
}
