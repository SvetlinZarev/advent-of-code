use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use aoc_2020_07::{part_one_dfs, part_one_recursive, part_two, DAY};
use aoc_2020_common::input::{default_test_input, load_input};

criterion_group!(benches, input_parsing, part_one, part_two);
criterion_main!(benches);

fn input_parsing(c: &mut Criterion) {
    let input = load_input(default_test_input(DAY));

    c.bench_with_input(
        BenchmarkId::new("day-07-p01-v1_recursive-parsing", ""),
        &input,
        |b, i| b.iter(|| black_box(part_one_recursive::parse_input(&i))),
    );

    c.bench_with_input(
        BenchmarkId::new("day-07-p01-v2_dfs-parsing", ""),
        &input,
        |b, i| b.iter(|| black_box(part_one_dfs::parse_input(&i))),
    );

    c.bench_with_input(
        BenchmarkId::new("day-07-p02-parsing", ""),
        &input,
        |b, i| b.iter(|| black_box(part_two::parse_input(&i))),
    );
}

fn part_one(c: &mut Criterion) {
    let input = load_input(default_test_input(DAY));

    let input_v1 = part_one_recursive::parse_input(&input);
    c.bench_with_input(
        BenchmarkId::new("day-07-p01-recursive", ""),
        &input_v1,
        |b, i| b.iter(|| black_box(part_one_recursive::solve_v1(i))),
    );

    let input_v2 = part_one_dfs::parse_input(&input);
    c.bench_with_input(BenchmarkId::new("day-07-p01|DFS", ""), &input_v2, |b, i| {
        b.iter(|| black_box(part_one_dfs::solve_dfs(i)))
    });
}

fn part_two(c: &mut Criterion) {
    let input = load_input(DEFAULT_INPUT_PATH);

    let input = part_two::parse_input(&input);
    c.bench_with_input(BenchmarkId::new("day-07-p02", ""), &input, |b, i| {
        b.iter(|| black_box(part_two::solve(i)))
    });
}
