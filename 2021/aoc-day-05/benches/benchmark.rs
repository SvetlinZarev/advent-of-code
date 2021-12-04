use aoc_day_05::{part_one, part_two, Line};
use aoc_shared::input::{file_line_delimited, load_input_from_file};
use aoc_shared::parsing::parse_line_delimited;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

criterion_group!(
    benches,
    benchmark_parsing,
    benchmark_part_1,
    benchmark_part_2
);
criterion_main!(benches);

fn benchmark_parsing(c: &mut Criterion) {
    let input = load_input_from_file("inputs/input.txt");

    c.bench_function("parse-input", |b| {
        b.iter(|| {
            let parsed: Vec<Line> = parse_line_delimited(&input);
            black_box(parsed);
        });
    });
}

fn benchmark_part_1(c: &mut Criterion) {
    let input = file_line_delimited("inputs/input.txt");

    c.bench_function("part-1", |b| {
        b.iter(|| black_box(part_one(&input)));
    });
}

fn benchmark_part_2(c: &mut Criterion) {
    let input = file_line_delimited("inputs/input.txt");

    c.bench_function("part-2", |b| {
        b.iter(|| black_box(part_two(&input)));
    });
}
