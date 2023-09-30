use aoc_shared::input::load_text_input_from_file;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};

use aoc_day_11::{parse_input, part_one, part_two};

criterion_group!(
    benches,
    benchmark_parsing,
    benchmark_part_one,
    benchmark_part_two
);
criterion_main!(benches);

fn benchmark_parsing(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");

    c.bench_with_input(BenchmarkId::new("parsing", ""), &input.as_str(), |b, i| {
        b.iter_with_large_drop(|| parse_input(i))
    });
}

fn benchmark_part_one(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let input = parse_input(&input);

    let mut g = c.benchmark_group("part-1");
    g.sample_size(10);

    g.bench_function("part-1", |b| {
        b.iter_batched(
            || input.clone(),
            |input| part_one(input),
            BatchSize::PerIteration,
        )
    });
}

fn benchmark_part_two(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let input = parse_input(&input);

    let mut g = c.benchmark_group("part-2");
    g.sample_size(10);

    g.bench_function("part-2", |b| {
        b.iter_batched(
            || input.clone(),
            |input| part_two(input),
            BatchSize::PerIteration,
        )
    });
    g.finish();
}
