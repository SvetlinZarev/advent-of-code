use aoc_shared::input::load_text_input_from_file;
use aoc_shared::parsing::parse_u8_numeric_grid;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

use aoc_day_08::{part_one, part_two};

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
        b.iter(|| parse_u8_numeric_grid(black_box(&input)));
    });
}

fn benchmark_part_one(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let grid = parse_u8_numeric_grid(&input);

    c.bench_function("part-1", |b| {
        b.iter(|| part_one(black_box(&grid)));
    });
}

fn benchmark_part_two(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let grid = parse_u8_numeric_grid(&input);

    c.bench_function("part-2", |b| {
        b.iter(|| part_two(black_box(&grid)));
    });
}
