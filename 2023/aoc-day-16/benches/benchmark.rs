use aoc_shared::input::load_text_input_from_file;
use aoc_shared::parsing::parse_u8_grid_borrowed;
use criterion::{criterion_group, criterion_main, Criterion};

use aoc_day_16::{part_one, part_two, part_two_rayon};

criterion_group!(benches, benchmark_part_one, benchmark_part_two);
criterion_main!(benches);

fn benchmark_part_one(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let input = parse_u8_grid_borrowed(&input);

    c.bench_function("part-1", |b| {
        b.iter(|| part_one(&input));
    });
}

fn benchmark_part_two(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let input = parse_u8_grid_borrowed(&input);

    c.bench_function("part-2/single thread", |b| {
        b.iter(|| part_two(&input));
    });

    c.bench_function("part-2/rayon", |b| {
        b.iter(|| part_two_rayon(&input));
    });
}
