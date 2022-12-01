use aoc_shared::input::load_text_input_from_file;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aoc_day_01::{parse_input, part_one, part_two};


criterion_group!(benches, benchmark_parsing, benchmark_part_one, benchmark_part_two);
criterion_main!(benches);


fn benchmark_parsing(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");

    c.bench_function("parsing", |b| {
        b.iter(|| black_box(parse_input(black_box(&input))));
    });
}


fn benchmark_part_one(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let parsed = parse_input(&input).unwrap();

    c.bench_function("part-1", |b| {
        b.iter(|| black_box(part_one(black_box(&parsed))));
    });
}


fn benchmark_part_two(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");
    let parsed = parse_input(&input).unwrap();

    c.bench_function("part-2", |b| {
        b.iter(|| black_box(part_two(black_box(&parsed))));
    });
}
