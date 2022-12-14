use aoc_shared::input::load_text_input_from_file;
use criterion::{BatchSize, black_box, Criterion, criterion_group, criterion_main};

use aoc_day_14::{parse_input, part_one, part_two_v1, part_two_v2};

criterion_group!(benches, benchmark_parsing, benchmark_part_one, benchmark_part_two);
criterion_main!(benches);


fn benchmark_parsing(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");

    c.bench_function("parsing", |b| {
        b.iter(|| parse_input(black_box(&input)));
    });
}


fn benchmark_part_one(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let (grid, last_row, initial_column) = parse_input(input);

    c.bench_function("part-1", |b| {
        b.iter_batched(
            || grid.clone(),
            |g| part_one(g, last_row, initial_column),
            BatchSize::LargeInput,
        )
    });
}


fn benchmark_part_two(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let (grid, _, initial_column) = parse_input(input);

    c.bench_function("part-2 (simulation)", |b| {
        b.iter_batched(
            || grid.clone(),
            |g| part_two_v1(g, initial_column),
            BatchSize::LargeInput,
        )
    });

    c.bench_function("part-2 (flood-fill)", |b| {
        b.iter_batched(
            || grid.clone(),
            |g| part_two_v2(g, initial_column),
            BatchSize::LargeInput,
        )
    });
}
