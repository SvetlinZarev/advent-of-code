use aoc_day_23::{parse_input, part_one, part_two};
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
    let input_one = load_text_input_from_file("inputs/input-1.txt");
    c.bench_function("parsing-a", |b| {
        b.iter(|| black_box(parse_input::<_, 2>(black_box(&input_one))));
    });

    let input_two = load_text_input_from_file("inputs/input-2.txt");
    c.bench_function("parsing-b", |b| {
        b.iter(|| black_box(parse_input::<_, 4>(black_box(&input_two))));
    });
}

fn benchmark_part_1(c: &mut Criterion) {
    let (rooms, hall) = parse_input(load_text_input_from_file("inputs/input-1.txt"));

    c.bench_function("part-1", |b| {
        b.iter(|| black_box(part_one(black_box(rooms), black_box(hall))))
    });
}

fn benchmark_part_2(c: &mut Criterion) {
    let (rooms, hall) = parse_input(load_text_input_from_file("inputs/input-2.txt"));

    c.bench_function("part-2", |b| {
        b.iter(|| black_box(part_two(black_box(rooms), black_box(hall))))
    });
}
