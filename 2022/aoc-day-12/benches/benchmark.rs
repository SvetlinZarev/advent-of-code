use aoc_shared::input::load_text_input_from_file;
use aoc_shared::parsing::parse_u8_grid;
use criterion::{BatchSize, Criterion, criterion_group, criterion_main};

use aoc_day_12::{part_one_v1, part_one_v2, part_two_v1, part_two_v2};

criterion_group!(benches, benchmark_part_one, benchmark_part_two);
criterion_main!(benches);

fn benchmark_part_one(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let grid = parse_u8_grid(input);

    c.bench_function("part-1 (a*, s->e)", |b| {
        b.iter_batched(
            || grid.clone(),
            |i| part_one_v1(i),
            BatchSize::LargeInput,
        );
    });

    c.bench_function("part-1 (bfs, e->s)", |b| {
        b.iter_batched(
            || grid.clone(),
            |i| part_one_v2(i),
            BatchSize::LargeInput,
        );
    });
}


fn benchmark_part_two(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let grid = parse_u8_grid(input);

    c.bench_function("part-2 (a*, s->e)", |b| {
        b.iter_batched(
            || grid.clone(),
            |i| part_two_v1(i),
            BatchSize::LargeInput,
        );
    });

    c.bench_function("part-2 (bfs, e->s)", |b| {
        b.iter_batched(
            || grid.clone(),
            |i| part_two_v2(i),
            BatchSize::LargeInput,
        );
    });
}
