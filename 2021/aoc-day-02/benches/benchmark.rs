use aoc_day_02::{part_one, part_two, Direction};
use aoc_shared::input::{load_line_delimited_input_from_file, load_text_input_from_file};
use aoc_shared::parsing::parse_line_delimited;
use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};

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
        b.iter_batched(
            || input.as_str(),
            |data| black_box(parse_line_delimited::<_, Direction>(data)),
            BatchSize::PerIteration,
        )
    });
}

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
