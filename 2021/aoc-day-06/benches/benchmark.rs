use aoc_day_06::{part_one_v1, part_one_v2, part_one_v3, part_two_v1, part_two_v2, part_two_v3};
use aoc_shared::input::load_text_input_from_file;
use aoc_shared::parsing::parse_csv;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

criterion_group!(benches, benchmark_part_1, benchmark_part_2);
criterion_main!(benches);

fn benchmark_part_1(c: &mut Criterion) {
    let input = parse_csv(load_text_input_from_file("inputs/input.txt"));

    c.bench_function("part-1-v1", |b| {
        b.iter(|| black_box(part_one_v1(black_box(&input))));
    });

    c.bench_function("part-1-v2", |b| {
        b.iter(|| black_box(part_one_v2(black_box(&input))));
    });

    c.bench_function("part-1-v3", |b| {
        b.iter(|| black_box(part_one_v3(black_box(&input))));
    });
}

fn benchmark_part_2(c: &mut Criterion) {
    let input = parse_csv(load_text_input_from_file("inputs/input.txt"));

    c.bench_function("part-2-v1", |b| {
        b.iter(|| black_box(part_two_v1(black_box(&input))));
    });

    c.bench_function("part-2-v2", |b| {
        b.iter(|| black_box(part_two_v2(black_box(&input))));
    });

    c.bench_function("part-2-v3", |b| {
        b.iter(|| black_box(part_two_v3(black_box(&input))));
    });
}
