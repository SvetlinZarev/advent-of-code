use aoc_shared::input::load_text_input_from_file;
use criterion::{Criterion, black_box, criterion_group, criterion_main};

use aoc_day_05::{parse_input, part_one, part_two_sorting, part_two_topo_sort};

criterion_group!(
    benches,
    benchmark_input_parsing,
    benchmark_part_one,
    benchmark_part_two
);
criterion_main!(benches);

fn benchmark_input_parsing(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");

    c.bench_function("parsing", |b| {
        b.iter(|| parse_input(&input));
    });
}

fn benchmark_part_one(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let (graph, updates) = parse_input(&input).unwrap();

    c.bench_function("part-1", |b| {
        b.iter(|| part_one(black_box(&graph), black_box(&updates)));
    });
}

fn benchmark_part_two(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let (graph, updates) = parse_input(&input).unwrap();

    c.bench_function("part-2/sorting", |b| {
        b.iter(|| part_two_sorting(black_box(&graph), black_box(&updates)));
    });

    c.bench_function("part-2/topo_sorting", |b| {
        b.iter(|| part_two_topo_sort(black_box(&graph), black_box(&updates)));
    });
}
