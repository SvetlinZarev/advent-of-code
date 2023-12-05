use aoc_shared::input::load_text_input_from_file;
use criterion::{criterion_group, criterion_main, Criterion};

use aoc_day_05::{parse_input, part_one, part_two_v1, part_two_v2};

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
        b.iter(|| parse_input(&input));
    });
}

fn benchmark_part_one(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let input = parse_input(input);

    c.bench_function("part-1", |b| {
        b.iter(|| part_one(&input));
    });
}

fn benchmark_part_two(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let input = parse_input(input);

    c.bench_function("part-2/v1", |b| {
        b.iter(|| part_two_v1(&input));
    });

    c.bench_function("part-2/v2", |b| {
        b.iter(|| part_two_v2(&input));
    });
}
