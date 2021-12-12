use aoc_day_12::{parse_input, part_one_v1, part_one_v2, part_two_v1, part_two_v2};
use aoc_shared::input::load_text_input_from_file;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

criterion_group!(
    benches,
    benchmark_parsing,
    benchmark_part_1,
    benchmark_part_2
);
criterion_main!(benches);

fn benchmark_parsing(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");

    c.bench_function("parsing", |b| {
        b.iter(|| black_box(parse_input(black_box(&input))));
    });
}

fn benchmark_part_1(c: &mut Criterion) {
    let (graph, limits) = parse_input(load_text_input_from_file("inputs/input.txt"));

    c.bench_function("part-1-v1", |b| {
        b.iter(|| black_box(part_one_v1(black_box(&graph), black_box(&limits))));
    });

    c.bench_function("part-1-v2", |b| {
        b.iter(|| black_box(part_one_v2(black_box(&graph), black_box(&limits))));
    });
}

fn benchmark_part_2(c: &mut Criterion) {
    let (graph, limits) = parse_input(load_text_input_from_file("inputs/input.txt"));

    c.bench_function("part-2-v1", |b| {
        b.iter(|| black_box(part_two_v1(black_box(&graph), black_box(&limits))));
    });

    c.bench_function("part-2-v1", |b| {
        b.iter(|| black_box(part_two_v2(black_box(&graph), black_box(&limits))));
    });
}
