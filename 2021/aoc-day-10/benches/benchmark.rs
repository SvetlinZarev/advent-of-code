use aoc_day_10::{parse_input, part_one, part_two};
use aoc_shared::input::load_text_input_from_file;
use aoc_shared::parsing::parse_line_delimited;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

criterion_group!(benches, benchmark_part_1, benchmark_part_2);
criterion_main!(benches);

fn benchmark_part_1(c: &mut Criterion) {
    let input = parse_input(load_text_input_from_file("inputs/input.txt"));

    c.bench_function("part-1", |b| {
        b.iter(|| black_box(part_one(black_box(&input))));
    });
}

fn benchmark_part_2(c: &mut Criterion) {
    let input = parse_input(load_text_input_from_file("inputs/input.txt"));

    c.bench_function("part-2", |b| {
        b.iter(|| black_box(part_two(black_box(&input))));
    });
}
