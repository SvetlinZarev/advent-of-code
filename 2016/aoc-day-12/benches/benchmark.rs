use aoc_shared::input::{load_line_delimited_input_from_file, load_text_input_from_file};
use aoc_shared::parsing::parse_line_delimited;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use aoc_day_12::{part_one, part_two, OpCode};

criterion_group!(
    benches,
    benchmark_parsing,
    benchmark_part_one,
    benchmark_part_two
);
criterion_main!(benches);

fn benchmark_parsing(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");

    c.bench_with_input(
        BenchmarkId::new("parsing", ""),
        &input.as_str(),
        |b, &input| b.iter_with_large_drop(|| parse_line_delimited::<_, OpCode>(input)),
    );
}

fn benchmark_part_one(c: &mut Criterion) {
    let input = load_line_delimited_input_from_file("inputs/input.txt");

    c.bench_function("part-1", |b| b.iter(|| part_one(input.as_slice())));
}

fn benchmark_part_two(c: &mut Criterion) {
    let input = load_line_delimited_input_from_file("inputs/input.txt");

    c.bench_function("part-2", |b| b.iter(|| part_two(input.as_slice())));
}
