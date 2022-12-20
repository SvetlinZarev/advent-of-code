use aoc_day_20::{v1, v2};
use aoc_shared::input::load_line_delimited_input_from_file;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

criterion_group!(benches, benchmark_part_one, benchmark_part_two);
criterion_main!(benches);

fn benchmark_part_one(c: &mut Criterion) {
    let mut g = c.benchmark_group("part-1");
    g.sample_size(30);

    let input = load_line_delimited_input_from_file("inputs/input.txt");

    g.bench_function("Rc<RefCell<>>", |b| {
        b.iter(|| v1::part_one(black_box(&input)));
    });

    g.bench_function("indexing", |b| {
        b.iter(|| v2::part_one(black_box(&input)));
    });
}

fn benchmark_part_two(c: &mut Criterion) {
    let mut g = c.benchmark_group("part-2");
    g.sample_size(10);

    let input = load_line_delimited_input_from_file("inputs/input.txt");

    g.bench_function("Rc<RefCell<>>", |b| {
        b.iter(|| v1::part_two(black_box(&input)));
    });

    g.bench_function("indexing", |b| {
        b.iter(|| v2::part_two(black_box(&input)));
    });
}
