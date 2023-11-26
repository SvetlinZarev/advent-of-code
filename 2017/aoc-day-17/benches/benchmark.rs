use aoc_shared::input::load_text_input_from_file;
use criterion::{criterion_group, criterion_main, Criterion};

use aoc_day_17::{part_one_deque, part_one_list, part_two_deque, part_two_idxs};

criterion_group!(benches, benchmark_part_one, benchmark_part_two);
criterion_main!(benches);

fn benchmark_part_one(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let input = input.trim().parse().unwrap();

    c.bench_function("part-1: list", |b| {
        b.iter(|| part_one_list(input));
    });

    c.bench_function("part-1: deque", |b| {
        b.iter(|| part_one_deque(input));
    });
}

fn benchmark_part_two(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let input = input.trim().parse().unwrap();

    let mut g = c.benchmark_group("part 2");
    g.sample_size(10);

    g.bench_function("part-2: indexes", |b| {
        b.iter(|| part_two_idxs(input));
    });

    g.bench_function("part-2: deque", |b| {
        b.iter(|| part_two_deque(input));
    });

    g.finish();
}
