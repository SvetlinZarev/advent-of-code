use aoc_shared::input::load_text_input_from_file;
use criterion::{criterion_group, criterion_main, Criterion};

use aoc_day_25::{parse_input, part_one_v1, part_one_v2};

criterion_group!(benches, benchmark_parsing, benchmark_part_one,);
criterion_main!(benches);

fn benchmark_parsing(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");

    c.bench_function("parsing", |b| {
        b.iter(|| parse_input(&input));
    });
}

fn benchmark_part_one(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let input = parse_input(&input).unwrap();

    let mut g = c.benchmark_group("part-one");
    g.sample_size(30);

    g.bench_function("part-1/v1", |b| {
        b.iter(|| part_one_v1(&input));
    });

    g.bench_function("part-1/v2", |b| {
        b.iter(|| part_one_v2(&input));
    });

    g.finish();
}
