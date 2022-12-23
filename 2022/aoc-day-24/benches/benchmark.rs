use criterion::{black_box, criterion_group, criterion_main, Criterion};


criterion_group!(benches, benchmark_parsing, benchmark_part_one, benchmark_part_two);
criterion_main!(benches);


fn benchmark_parsing(c: &mut Criterion) {
    c.bench_function("parsing", |b| {
        b.iter(|| black_box(todo!()));
    });
}


fn benchmark_part_one(c: &mut Criterion) {
    c.bench_function("part-1", |b| {
        b.iter(|| black_box(todo!()));
    });
}


fn benchmark_part_two(c: &mut Criterion) {
    c.bench_function("part-2", |b| {
        b.iter(|| black_box(todo!()));
    });
}
