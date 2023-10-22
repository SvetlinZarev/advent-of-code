use aoc_shared::input::{load_line_delimited_input_from_file, load_text_input_from_file};
use aoc_shared::parsing::parse_line_delimited;
use criterion::{Criterion, criterion_group, criterion_main};

use aoc_day_23::{OpCode, part_one};

criterion_group!(
    benches,
    benchmark_parsing,
    benchmark_part_one,
);
criterion_main!(benches);

fn benchmark_parsing(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");

    c.bench_function("parsing", |b| {
        b.iter(|| parse_line_delimited::<_, OpCode>(&input));
    });
}

fn benchmark_part_one(c: &mut Criterion) {
    let input = load_line_delimited_input_from_file("inputs/input.txt");

    c.bench_function("part-1", |b| {
        b.iter(|| part_one(&input));
    });
}
