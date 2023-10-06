use aoc_shared::input::load_text_input_from_file;
use aoc_shared::parsing::parse_line_delimited_after_row;
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

use aoc_day_22::{part_one_v1, part_one_v2, Info};

criterion_group!(benches, benchmark_parsing, benchmark_part_one,);
criterion_main!(benches);

fn benchmark_parsing(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    c.bench_function("parsing", |b| {
        b.iter(|| parse_line_delimited_after_row::<_, Info>(&input, 2))
    });
}
fn benchmark_part_one(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let mut input = parse_line_delimited_after_row(input, 2);

    let mut g = c.benchmark_group("part-1");
    g.bench_function("v1", |b| b.iter(|| part_one_v1(&input)));
    g.bench_function("v2", |b| {
        b.iter_batched(
            || input.clone(),
            |mut input| part_one_v2(&mut input),
            BatchSize::PerIteration,
        )
    });

    g.finish()
}
