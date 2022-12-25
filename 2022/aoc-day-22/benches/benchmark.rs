use aoc_shared::input::load_text_input_from_file;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

use aoc_day_22::parse_input;
use aoc_day_22::part_one::part_one;

criterion_group!(benches, benchmark_part_one, benchmark_part_two);
criterion_main!(benches);

fn benchmark_part_one(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let (map, instr) = parse_input(input);

    c.bench_function("part-1", |b| {
        b.iter(|| part_one(black_box(&map), black_box(&instr)));
    });
}

fn benchmark_part_two(c: &mut Criterion) {
    //
}
