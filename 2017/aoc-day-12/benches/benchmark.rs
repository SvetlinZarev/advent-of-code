use aoc_shared::input::load_text_input_from_file;
use criterion::{criterion_group, criterion_main, Criterion};

use aoc_day_12::part_one_and_two;

criterion_group!(benches, benchmark_part_one_and_two,);
criterion_main!(benches);

fn benchmark_part_one_and_two(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");

    c.bench_function("day-12", |b| {
        b.iter(|| part_one_and_two(&input));
    });
}
