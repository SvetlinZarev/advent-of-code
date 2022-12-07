use aoc_day_07::{parse_input, v1, v2};
use aoc_shared::input::load_text_input_from_file;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

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
        b.iter(|| parse_input(black_box(&input)));
    });
}

fn benchmark_part_one(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let entries = parse_input(&input);

    c.bench_function("part-1 (flat)", |b| {
        b.iter(|| v1::part_one(black_box(&entries)));
    });

    c.bench_function("part-1 (tree)", |b| {
        b.iter(|| v2::part_one(black_box(&entries)));
    });
}

fn benchmark_part_two(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let entries = parse_input(&input);

    c.bench_function("part-2 (flat)", |b| {
        b.iter(|| v1::part_two(black_box(&entries)));
    });

    c.bench_function("part-2 (tree)", |b| {
        b.iter(|| v2::part_two(black_box(&entries)));
    });
}
