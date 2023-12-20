use aoc_shared::input::load_text_input_from_file;
use criterion::{criterion_group, criterion_main, Criterion};

use aoc_day_20::{v1, v2};

criterion_group!(
    benches,
    benchmark_parsing,
    benchmark_part_one_v1,
    benchmark_part_one_v2,
    benchmark_part_two_v1,
    benchmark_part_two_v2,
);
criterion_main!(benches);

fn benchmark_parsing(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");

    c.bench_function("parsing/v1", |b| {
        b.iter(|| v1::load_graph(&input));
    });

    c.bench_function("parsing/v2", |b| {
        b.iter(|| v2::load_graph(&input));
    });
}

fn benchmark_part_one_v1(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let graph = v1::load_graph(&input);

    c.bench_function("part-1/v1", |b| {
        b.iter(|| v1::part_one(&graph));
    });
}

fn benchmark_part_one_v2(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let graph = v2::load_graph(&input);

    c.bench_function("part-1/v2", |b| {
        b.iter(|| v2::part_one(&graph));
    });
}

fn benchmark_part_two_v1(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let graph = v1::load_graph(&input);

    c.bench_function("part-2/v1", |b| {
        b.iter(|| v1::part_two(&graph));
    });
}

fn benchmark_part_two_v2(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let graph = v2::load_graph(&input);

    c.bench_function("part-2/v2", |b| {
        b.iter(|| v2::part_two(&graph));
    });
}
