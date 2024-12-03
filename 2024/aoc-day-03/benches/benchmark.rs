use aoc_shared::input::load_text_input_from_file;
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use aoc_day_03::with_fsm::{part_one_v2, part_two_v2};
use aoc_day_03::with_regex::{part_one_v1, part_two_v1};

criterion_group!(benches, benchmark_part_one, benchmark_part_two);
criterion_main!(benches);

fn benchmark_part_one(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");

    c.bench_function("part-1/regex", |b| {
        b.iter(|| part_one_v1(black_box(&input)));
    });

    c.bench_function("part-1/state-machine", |b| {
        b.iter(|| part_one_v2(black_box(&input)));
    });
}

fn benchmark_part_two(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");

    c.bench_function("part-2/regex", |b| {
        b.iter(|| part_two_v1(black_box(&input)));
    });

    c.bench_function("part-2/state-machine", |b| {
        b.iter(|| part_two_v2(black_box(&input)));
    });
}
