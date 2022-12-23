use aoc_day_23::{parse_input, part_one, part_two_v1, part_two_v2};
use aoc_shared::input::load_text_input_from_file;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

criterion_group!(benches, benchmark_part_one, benchmark_part_two);
criterion_main!(benches);

fn benchmark_part_one(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let parsed = parse_input(input);

    c.bench_function("part-1", |b| {
        b.iter(|| part_one(black_box(&parsed)));
    });
}

fn benchmark_part_two(c: &mut Criterion) {
    let mut g = c.benchmark_group("part-2");

    let input = load_text_input_from_file("inputs/input.txt");
    let parsed = parse_input(input);

    g.bench_function("v1", |b| {
        b.iter(|| part_two_v1(black_box(&parsed)));
    });

    g.bench_function("v2", |b| {
        b.iter(|| part_two_v2(black_box(&parsed)));
    });
}
