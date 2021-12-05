use aoc_shared::input::load_line_delimited_input_from_file;
use criterion::{black_box, criterion_group, criterion_main, Criterion, BatchSize};
use aoc_day_03::{part_two_v1, part_two_v2};


criterion_group!(benches, benchmark);
criterion_main!(benches);

fn benchmark(c: &mut Criterion) {
    let input: Vec<String> = load_line_delimited_input_from_file("inputs/input.txt");

    c.bench_function("with_two_pointers", |b| {
        b.iter_batched(
            || input.clone(),
            |mut input| black_box(part_two_v1(&mut input)),
            BatchSize::PerIteration,
        )
    });

    c.bench_function("with_sorting", |b| {
        b.iter_batched(
            || input.clone(),
            |mut input| black_box(part_two_v2(&mut input)),
            BatchSize::PerIteration,
        )
    });
}