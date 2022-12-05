use aoc_shared::input::load_text_input_from_file;
use criterion::{black_box, Criterion, criterion_group, criterion_main};

use aoc_day_05::{parse_input, part_one_v1, part_one_v2, part_two};

criterion_group!(benches, benchmark_parsing, benchmark_part_one, benchmark_part_two);
criterion_main!(benches);


fn benchmark_parsing(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");

    c.bench_function("parsing", |b| {
        b.iter(|| parse_input(black_box(&input)));
    });
}


fn benchmark_part_one(c: &mut Criterion) {
    let (stacks, instructions) = parse_input(load_text_input_from_file("inputs/input.txt"));

    c.bench_function("part-1 (v1)", |b| {
        b.iter(|| part_one_v1(black_box(&stacks), black_box(&instructions)));
    });

    c.bench_function("part-1 (v2)", |b| {
        b.iter(|| part_one_v2(black_box(&stacks), black_box(&instructions)));
    });
}


fn benchmark_part_two(c: &mut Criterion) {
    let (stacks, instructions) = parse_input(load_text_input_from_file("inputs/input.txt"));

    c.bench_function("part-2", |b| {
        b.iter(|| part_two(black_box(&stacks), black_box(&instructions)));
    });
}
