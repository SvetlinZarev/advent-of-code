use aoc_shared::input::load_text_input_from_file;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

use aoc_day_11::{part_one_v1, part_one_v2, part_two};

criterion_group!(benches, benchmark_part_one, benchmark_part_two);
criterion_main!(benches);

fn benchmark_part_one(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");

    c.bench_function("part-1/v1", |b| {
        b.iter(|| part_one_v1(black_box(&input)));
    });

    c.bench_function("part-1/v2", |b| {
        b.iter(|| part_one_v2(black_box(&input)));
    });
}

fn benchmark_part_two(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");

    c.bench_function("part-2", |b| {
        b.iter(|| part_two(black_box(&input)));
    });
}
