use aoc_shared::input::load_text_input_from_file;
use criterion::{criterion_group, criterion_main, Criterion};

use aoc_day_06::{
    parse_input, part_one, part_two_binary_search, part_two_math, part_two_naive, part_two_naive2,
};

criterion_group!(
    benches,
    benchmark_parsing,
    benchmark_part_one,
    benchmark_part_two
);
criterion_main!(benches);

fn benchmark_parsing(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");

    c.bench_function("parsing", |b| {
        b.iter(|| parse_input(&input));
    });
}

fn benchmark_part_one(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let input = parse_input(input);

    c.bench_function("part-1", |b| {
        b.iter(|| part_one(&input));
    });
}

fn benchmark_part_two(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let input = parse_input(input);

    c.bench_function("part-2/naive", |b| {
        b.iter(|| part_two_naive(&input));
    });

    c.bench_function("part-2/naive2", |b| {
        b.iter(|| part_two_naive2(&input));
    });

    c.bench_function("part-2/math", |b| {
        b.iter(|| part_two_math(&input));
    });

    c.bench_function("part-2/binary-search", |b| {
        b.iter(|| part_two_binary_search(&input));
    });
}
