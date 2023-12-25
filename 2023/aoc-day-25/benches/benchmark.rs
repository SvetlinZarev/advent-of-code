use aoc_shared::input::load_text_input_from_file;
use criterion::{criterion_group, criterion_main, Criterion};

use aoc_day_25::{parse_input, v2, v3};

criterion_group!(benches, benchmark_parsing, benchmark_part_one);
criterion_main!(benches);

fn benchmark_parsing(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");

    c.bench_function("parsing", |b| {
        b.iter(|| parse_input(&input));
    });
}

fn benchmark_part_one(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");

    c.bench_function("v2 (bfs)", |b| {
        b.iter(|| v2::part_one(&input));
    });

    c.bench_function("v3 (Karger)", |b| {
        b.iter(|| v3::part_one(&input));
    });
}
