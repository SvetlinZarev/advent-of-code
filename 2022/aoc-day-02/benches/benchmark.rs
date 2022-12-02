use aoc_day_02::{v1, v2, Game};
use aoc_shared::input::{load_line_delimited_input_from_file, load_text_input_from_file};
use aoc_shared::parsing::parse_line_delimited;
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
        b.iter(|| black_box(parse_line_delimited::<_, Game>(black_box(input.as_str()))));
    });
}

fn benchmark_part_one(c: &mut Criterion) {
    let input = load_line_delimited_input_from_file("inputs/input.txt");

    c.bench_function("part-1 (v1)", |b| {
        b.iter(|| black_box(v1::part_one(black_box(&input))));
    });

    c.bench_function("part-1 (v2)", |b| {
        b.iter(|| black_box(v2::part_one(black_box(&input))));
    });
}

fn benchmark_part_two(c: &mut Criterion) {
    let input = load_line_delimited_input_from_file("inputs/input.txt");

    c.bench_function("part-2 (v1)", |b| {
        b.iter(|| black_box(v1::part_two(black_box(&input))));
    });

    c.bench_function("part-2 (v2)", |b| {
        b.iter(|| black_box(v2::part_two(black_box(&input))));
    });
}
