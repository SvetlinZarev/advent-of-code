use aoc_shared::input::load_text_input_from_file;
use criterion::{criterion_group, criterion_main, Criterion};

use aoc_day_07::{load_input, part_one, part_two};

criterion_group!(
    benches,
    benchmark_parsing,
    benchmark_part_one,
    benchmark_part_two
);
criterion_main!(benches);

fn benchmark_parsing(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");

    c.bench_function("parsing", |b| {
        b.iter_with_large_drop(|| load_input(&input));
    });
}

fn benchmark_part_one(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let graph = load_input(&input).unwrap();

    c.bench_function("part-1", |b| {
        b.iter(|| part_one(&graph));
    });
}

fn benchmark_part_two(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let graph = load_input(&input).unwrap();

    c.bench_function("part-2", |b| {
        b.iter(|| part_two(&graph));
    });
}
