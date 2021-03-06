use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use aoc_2020_05::{part_one, part_two, DAY};
use aoc_2020_common::input::{default_test_input, load_input};

criterion_group!(benches, part_one, part_two);
criterion_main!(benches);

fn part_one(c: &mut Criterion) {
    let input = load_input(default_test_input(DAY));

    c.bench_with_input(
        BenchmarkId::new("day-05-p01", ""),
        &input.as_bytes(),
        |b, i| b.iter(|| black_box(part_one::solve(i))),
    );
}

fn part_two(c: &mut Criterion) {
    let input = load_input(default_test_input(DAY));

    c.bench_with_input(
        BenchmarkId::new("day-05-p02-plain", ""),
        &input.as_bytes(),
        |b, i| b.iter(|| black_box(part_two::solve_v1(i))),
    );

    c.bench_with_input(
        BenchmarkId::new("day-05-p02-xor", ""),
        &input.as_bytes(),
        |b, i| b.iter(|| black_box(part_two::solve_v2_xor(i))),
    );

    c.bench_with_input(
        BenchmarkId::new("day-05-p02-bitwise", ""),
        &input.as_bytes(),
        |b, i| b.iter(|| black_box(part_two::solve_v3_bitwise(i))),
    );
}
