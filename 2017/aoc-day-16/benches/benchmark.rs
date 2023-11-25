use aoc_shared::input::load_text_input_from_file;
use aoc_shared::parsing::parse_csv;
use criterion::{criterion_group, criterion_main, Criterion};

use aoc_day_16::{part_one, part_two};

criterion_group!(benches, benchmark_part_one, benchmark_part_two);
criterion_main!(benches);

fn benchmark_part_one(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let parsed = parse_csv(input);

    c.bench_function("part-1", |b| {
        b.iter(|| part_one(&parsed));
    });
}

fn benchmark_part_two(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let parsed = parse_csv(input);

    c.bench_function("part-2", |b| {
        b.iter(|| part_two(&parsed));
    });
}
