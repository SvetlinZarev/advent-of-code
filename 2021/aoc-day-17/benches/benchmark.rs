use aoc_day_17::{parse_input, part_one, part_two};
use aoc_shared::input::load_text_input_from_file;
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
        b.iter(|| black_box(parse_input(black_box(&input))));
    });
}

fn benchmark_part_1(c: &mut Criterion) {
    let (x0, x1, y0, y1) = parse_input(load_text_input_from_file("inputs/input.txt"));

    c.bench_function("part-1", |b| {
        b.iter(|| {
            black_box(part_one(
                black_box(x0),
                black_box(x1),
                black_box(y0),
                black_box(y1),
            ))
        });
    });
}

fn benchmark_part_2(c: &mut Criterion) {
    let (x0, x1, y0, y1) = parse_input(load_text_input_from_file("inputs/input.txt"));

    c.bench_function("part-2", |b| {
        b.iter(|| {
            black_box(part_two(
                black_box(x0),
                black_box(x1),
                black_box(y0),
                black_box(y1),
            ))
        });
    });
}
