use aoc_shared::input::load_text_input_from_file;
use criterion::{criterion_group, criterion_main, Criterion};

use aoc_day_16::{solve_v1, solve_v2, PART_1_LEN, PART_2_LEN};

criterion_group!(benches, benchmark_part_one, benchmark_part_two);
criterion_main!(benches);

fn benchmark_part_one(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let input = input.trim();

    c.bench_function("part 1 (v1)", |b| {
        b.iter(|| solve_v1(&input, PART_1_LEN));
    });

    c.bench_function("part 1 (v2)", |b| {
        b.iter(|| solve_v2(&input, PART_1_LEN));
    });
}

fn benchmark_part_two(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let input = input.trim();

    let mut g = c.benchmark_group("part 2");
    g.sample_size(20);

    g.bench_function("v1", |b| {
        b.iter(|| solve_v1(&input, PART_2_LEN));
    });

    g.bench_function("v2", |b| {
        b.iter(|| solve_v2(&input, PART_2_LEN));
    });

    g.finish();
}
