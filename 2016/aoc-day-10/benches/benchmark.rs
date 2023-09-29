use aoc_shared::input::load_text_input_from_file;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};

use aoc_day_10::{parse_input, part_two};

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
    let (start_node, graph) = parse_input(&input);

    c.bench_function("part-1", |b| {
        b.iter_batched(
            || (start_node, graph.clone()),
            |(start_node, graph)| part_two(start_node, graph),
            BatchSize::PerIteration,
        )
    });
}

fn benchmark_part_two(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let (start_node, graph) = parse_input(&input);

    c.bench_function("part-2", |b| {
        b.iter_batched(
            || (start_node, graph.clone()),
            |(start_node, graph)| part_two(start_node, graph),
            BatchSize::PerIteration,
        )
    });
}
