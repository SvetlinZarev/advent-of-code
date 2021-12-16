use aoc_shared::input::load_text_input_from_file;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aoc_day_16::{decode_packets, parse_to_binary, part_one, part_two};

criterion_group!(
    benches,
    benchmark_parsing,
    benchmark_part_1,
    benchmark_part_2
);
criterion_main!(benches);

fn benchmark_parsing(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");

    c.bench_function("parse-to-binary", |b| {
        b.iter(|| black_box(parse_to_binary(black_box(&input))));
    });

    let (binary, bits) = parse_to_binary(&input);
    c.bench_function("decode-binary", |b| {
        b.iter(|| black_box(decode_packets(black_box(&binary), black_box(bits))));
    });
}

fn benchmark_part_1(c: &mut Criterion) {
    let (binary, bits) = parse_to_binary(load_text_input_from_file("inputs/input.txt"));
    let packet = decode_packets(&binary, bits);

    c.bench_function("part-1", |b| {
        b.iter(|| black_box(part_one(black_box(&packet))));
    });
}

fn benchmark_part_2(c: &mut Criterion) {
    let (binary, bits) = parse_to_binary(load_text_input_from_file("inputs/input.txt"));
    let packet = decode_packets(&binary, bits);

    c.bench_function("part-2", |b| {
        b.iter(|| black_box(part_two(black_box(&packet))));
    });
}
