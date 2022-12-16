use aoc_day_16::{p1v1, parse_input};
use aoc_shared::input::load_text_input_from_file;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

criterion_group!(benches, benchmark_parsing, benchmark_part_one,);
criterion_main!(benches);

fn benchmark_parsing(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");

    c.bench_function("parsing", |b| {
        b.iter(|| parse_input(black_box(&input)));
    });
}

fn benchmark_part_one(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let graph = parse_input(input);

    c.bench_function("part-1 (greedy)", |b| {
        b.iter(|| p1v1::part_one::<true>(black_box(&graph)));
    });

    c.bench_function("part-1 (full dp)", |b| {
        b.iter(|| p1v1::part_one::<true>(black_box(&graph)));
    });
}

// The second part of the problem takes around 25seconds and is too slow to benchmark!
