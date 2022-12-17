use aoc_day_17::{part_one, part_two};
use aoc_shared::input::load_text_input_from_file;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

criterion_group!(benches, benchmark_part_one, benchmark_part_two);
criterion_main!(benches);

fn benchmark_part_one(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let input = input.trim_end().as_bytes();

    c.bench_function("part-1", |b| {
        b.iter(|| part_one(black_box(input)));
    });
}

fn benchmark_part_two(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let input = input.trim_end().as_bytes();

    c.bench_function("part-2", |b| {
        b.iter(|| part_two(black_box(input)));
    });
}
