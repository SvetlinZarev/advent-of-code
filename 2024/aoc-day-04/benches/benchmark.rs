use aoc_shared::input::load_text_input_from_file;
use criterion::{Criterion, black_box, criterion_group, criterion_main};

use aoc_day_04::{parse_input, part_one_v1, part_one_v2, part_two_v1, part_two_v2};

criterion_group!(
    benches,
    benchmark_input_parsing,
    benchmark_part_one,
    benchmark_part_two
);
criterion_main!(benches);

fn benchmark_input_parsing(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");

    c.bench_function("parsing", |b| {
        b.iter(|| parse_input(&input));
    });
}

fn benchmark_part_one(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let parsed = parse_input(&input).unwrap();

    c.bench_function("part-1/recursive", |b| {
        b.iter(|| part_one_v1(black_box(&parsed)));
    });

    c.bench_function("part-1/iterative", |b| {
        b.iter(|| part_one_v2(black_box(&parsed)));
    });
}

fn benchmark_part_two(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let parsed = parse_input(&input).unwrap();

    c.bench_function("part-2/v1", |b| {
        b.iter(|| part_two_v1(black_box(&parsed)));
    });

    c.bench_function("part-2/v2", |b| {
        b.iter(|| part_two_v2(black_box(&parsed)));
    });
}