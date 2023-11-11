use aoc_shared::input::load_text_input_from_file;
use criterion::{criterion_group, criterion_main, Criterion};

use aoc_day_15::{parse_input, part_one, part_two};

criterion_group!(benches, benchmark_part_one, benchmark_part_two);
criterion_main!(benches);

fn benchmark_part_one(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let (a, b) = parse_input(&input);

    let mut g = c.benchmark_group("part 1");
    g.sample_size(10);

    g.bench_function("part-1", |bencher| {
        bencher.iter(|| part_one(a, b));
    });

    g.finish();
}

fn benchmark_part_two(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let (a, b) = parse_input(&input);

    let mut g = c.benchmark_group("part 2");
    g.sample_size(10);

    g.bench_function("part-2", |bencher| {
        bencher.iter(|| part_two(a, b));
    });

    g.finish();
}
