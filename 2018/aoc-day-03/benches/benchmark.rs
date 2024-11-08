use aoc_shared::input::load_text_input_from_file;
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

use aoc_day_03::{parse_input, part_one, part_two};

criterion_group!(
    benches,
    benchmark_parsing,
    benchmark_part_one,
    benchmark_part_two
);
criterion_main!(benches);

fn benchmark_parsing(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");

    let mut g = c.benchmark_group("parsing");
    g.sample_size(10);

    g.bench_function("with-regex", |b| {
        b.iter(|| parse_input(black_box(input.as_str())));
    });

    g.finish()
}

fn benchmark_part_one(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let parsed = parse_input(&input).unwrap();

    let mut g = c.benchmark_group("part-one");
    g.sample_size(10);

    g.bench_function("naive", |b| {
        b.iter(|| part_one(&parsed));
    });

    g.finish()
}

fn benchmark_part_two(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let parsed = parse_input(&input).unwrap();

    let mut g = c.benchmark_group("part-two");
    g.sample_size(10);

    g.bench_function("naive", |b| {
        b.iter(|| part_two(&parsed));
    });

    g.finish()
}
