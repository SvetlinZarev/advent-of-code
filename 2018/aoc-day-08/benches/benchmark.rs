use aoc_shared::input::load_text_input_from_file;
use criterion::{criterion_group, criterion_main, Criterion};

use aoc_day_08::{parse_input, part_one_iter, part_one_rec, part_two_iter, part_two_rec};

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
        b.iter(|| parse_input(input.as_str()));
    });
}

fn benchmark_part_one(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let input = parse_input(&input).unwrap();

    c.bench_function("part-1/iterative", |b| {
        b.iter(|| part_one_iter(&input));
    });

    c.bench_function("part-1/recursive", |b| {
        b.iter(|| part_one_rec(&input));
    });
}

fn benchmark_part_two(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let input = parse_input(&input).unwrap();

    c.bench_function("part-2/iterative", |b| {
        b.iter(|| part_two_iter(&input));
    });

    c.bench_function("part-2/recursive", |b| {
        b.iter(|| part_two_rec(&input));
    });
}
