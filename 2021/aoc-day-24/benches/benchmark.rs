use aoc_day_24::{part_one, part_two};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

criterion_group!(benches, benchmark_part_1, benchmark_part_2);
criterion_main!(benches);

fn benchmark_part_1(c: &mut Criterion) {
    c.bench_function("part-1", |b| b.iter(|| black_box(part_one())));
}

fn benchmark_part_2(c: &mut Criterion) {
    c.bench_function("part-2", |b| b.iter(|| black_box(part_two())));
}
