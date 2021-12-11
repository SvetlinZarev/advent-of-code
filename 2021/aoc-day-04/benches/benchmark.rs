use aoc_day_04::{parse_input, part_one, part_two};
use aoc_shared::input::load_text_input_from_file;
use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use std::fs::File;
use std::io::BufReader;

criterion_group!(
    benches,
    benchmark_parsing,
    benchmark_part_1,
    benchmark_part_2
);
criterion_main!(benches);

fn benchmark_parsing(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");

    c.bench_function("parse-input", |b| {
        b.iter_batched(
            || BufReader::new(input.as_bytes()),
            |reader| black_box(parse_input(reader)),
            BatchSize::PerIteration,
        );
    });
}

fn benchmark_part_1(c: &mut Criterion) {
    let file = File::open("inputs/input.txt").unwrap();
    let (numbers, boards) = parse_input(BufReader::new(file));

    c.bench_function("part-1", |b| {
        b.iter(|| black_box(part_one(black_box(&numbers), black_box(&boards))));
    });
}

fn benchmark_part_2(c: &mut Criterion) {
    let file = File::open("inputs/input.txt").unwrap();
    let (numbers, boards) = parse_input(BufReader::new(file));

    c.bench_function("part-2", |b| {
        b.iter(|| black_box(part_two(black_box(&numbers), black_box(&boards))));
    });
}
