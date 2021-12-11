use aoc_day_09::{part_one, part_two};
use aoc_shared::input::load_text_input_from_file;
use aoc_shared::parsing::parse_u8_numeric_grid;
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
        b.iter(|| black_box(parse_u8_numeric_grid(black_box(&input))));
    });
}

fn benchmark_part_1(c: &mut Criterion) {
    let input = parse_u8_numeric_grid(load_text_input_from_file("inputs/input.txt"));

    c.bench_function("part-1", |b| {
        b.iter(|| black_box(part_one(black_box(&input))));
    });
}

fn benchmark_part_2(c: &mut Criterion) {
    let input = parse_u8_numeric_grid(load_text_input_from_file("inputs/input.txt"));

    c.bench_function("part-2", |b| {
        b.iter(|| black_box(part_two(black_box(&input))));
    });
}
