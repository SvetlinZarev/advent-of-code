use aoc_shared::input::load_line_delimited_input_from_file;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aoc_day_01::{part_one, part_two};


criterion_group!(benches, benchmark_part_one, benchmark_part_two);
criterion_main!(benches);

fn benchmark_part_one(c: &mut Criterion) {
    let input = load_line_delimited_input_from_file("inputs/input.txt");

    c.bench_function("part-1", |b| {
        b.iter(|| black_box(part_one(black_box(&input))));
    });
}

fn benchmark_part_two(c: &mut Criterion) {
    let input = load_line_delimited_input_from_file("inputs/input.txt");

    c.bench_function("part-2", |b| {
        b.iter(|| black_box(part_two(black_box(&input))));
    });
}