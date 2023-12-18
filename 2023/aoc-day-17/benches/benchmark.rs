use aoc_shared::input::load_text_input_from_file;
use criterion::{criterion_group, criterion_main, Criterion};

use aoc_day_17::{v1, v2, v4};

criterion_group!(benches, benchmark_part_one, benchmark_part_two);
criterion_main!(benches);

fn benchmark_part_one(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");

    c.bench_function("part-1/v1", |b| {
        b.iter(|| v1::part_one(input.as_bytes()));
    });

    c.bench_function("part-1/v2", |b| {
        b.iter(|| v2::part_one(input.as_bytes()));
    });

    c.bench_function("part-1/v3", |b| {
        b.iter(|| v2::part_one(input.as_bytes()));
    });

    c.bench_function("part-1/v4", |b| {
        b.iter(|| v2::part_one(input.as_bytes()));
    });
}

fn benchmark_part_two(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");

    c.bench_function("part-2/v1", |b| {
        b.iter(|| v1::part_two(input.as_bytes()));
    });

    c.bench_function("part-2/v2", |b| {
        b.iter(|| v2::part_two(input.as_bytes()));
    });

    c.bench_function("part-2/v3", |b| {
        b.iter(|| v2::part_two(input.as_bytes()));
    });

    c.bench_function("part-2/v4", |b| {
        b.iter(|| v4::part_two(input.as_bytes()));
    });
}
