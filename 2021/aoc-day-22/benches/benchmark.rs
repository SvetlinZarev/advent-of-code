use aoc_day_22::{part_one_v1, part_one_v2, part_two, Command};
use aoc_shared::input::{load_line_delimited_input_from_file, load_text_input_from_file};
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
    let input = load_text_input_from_file("inputs/input.txt");

    c.bench_function("parsing", |b| {
        b.iter(|| black_box(parse_line_delimited::<_, Command>(black_box(&input))));
    });
}

fn benchmark_part_1(c: &mut Criterion) {
    let input = load_line_delimited_input_from_file("inputs/input.txt");

    c.bench_function("part-1-v1", |b| {
        b.iter(|| black_box(part_one_v1(black_box(&input))))
    });

    c.bench_function("part-1-v2", |b| {
        b.iter(|| black_box(part_one_v2(black_box(&input))))
    });
}

fn benchmark_part_2(c: &mut Criterion) {
    let input = load_line_delimited_input_from_file("inputs/input.txt");

    c.bench_function("part-2", |b| {
        b.iter(|| black_box(part_two(black_box(&input))))
    });
}
