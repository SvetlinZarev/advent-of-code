use aoc_shared::input::load_text_input_from_file;
use aoc_shared::parsing::parse_u8_grid;
use criterion::{criterion_group, criterion_main, Criterion};

use aoc_day_19::solve;

criterion_group!(benches, benchmark_solution);
criterion_main!(benches);

fn benchmark_solution(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let input = parse_u8_grid(input);

    c.bench_function("solution", |b| {
        b.iter(|| solve(&input));
    });
}
