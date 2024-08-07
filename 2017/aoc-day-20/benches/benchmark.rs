use aoc_shared::input::load_text_input_from_file;
use criterion::{Criterion, criterion_group, criterion_main};

use aoc_day_20::{parse_input, part_one, part_two};

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
    let (p, v, a) = parse_input(&input);

    c.bench_function("part-1", |b| {
        b.iter(|| part_one(&p, &v, &a));
    });
}

fn benchmark_part_two(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let (p, v, a) = parse_input(&input);

    c.bench_function("part-2", |b| {
        b.iter(|| part_two(&p, &v, &a));
    });
}
