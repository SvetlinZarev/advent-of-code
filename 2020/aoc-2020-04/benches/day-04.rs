use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use aoc_2020_04::{part_two, DAY};
use aoc_2020_common::input::{default_test_input, load_input};

criterion_group!(benches, part_two);
criterion_main!(benches);

fn part_two(c: &mut Criterion) {
    let input = load_input(default_test_input(DAY));

    c.bench_with_input(BenchmarkId::new("day-04-p02", ""), &input, |b, i| {
        b.iter(|| black_box(part_two::solve(i)))
    });
}
