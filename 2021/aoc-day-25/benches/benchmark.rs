use aoc_day_25::part_one;
use aoc_shared::input::load_text_input_from_file;
use aoc_shared::parsing::parse_u8_grid;
use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};

criterion_group!(benches, benchmark_parsing, benchmark_part_1,);
criterion_main!(benches);

fn benchmark_parsing(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");

    c.bench_function("parsing", |b| {
        b.iter(|| black_box(parse_u8_grid(black_box(&input))));
    });
}

fn benchmark_part_1(c: &mut Criterion) {
    let input = parse_u8_grid(load_text_input_from_file("inputs/input.txt"));

    c.bench_function("part-1", |b| {
        b.iter_batched(
            || input.clone(),
            |i| black_box(part_one(i)),
            BatchSize::PerIteration,
        );
    });
}
