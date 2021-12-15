use aoc_day_15::{a_star, a_star_pf, expand_grid, Position};
use aoc_shared::input::load_text_input_from_file;
use aoc_shared::parsing::parse_numeric_grid;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

criterion_group!(
    benches,
    benchmark_parsing,
    benchmark_part_1,
    benchmark_part_2
);
criterion_main!(benches);

fn benchmark_parsing(c: &mut Criterion) {
    let input = load_text_input_from_file("inputs/input.txt");

    c.bench_function("parsing", |b| {
        b.iter(|| black_box(parse_numeric_grid::<_, u8>(black_box(&input))));
    });
}

fn benchmark_part_1(c: &mut Criterion) {
    let grid = parse_numeric_grid(load_text_input_from_file("inputs/input.txt"));
    let start = Position::new(0, 0);
    let dst = Position::new(grid.len() - 1, grid[0].len() - 1);

    c.bench_function("part-1: h=none", |b| {
        b.iter(|| black_box(a_star(black_box(&grid), start, dst, |_, _| 0)));
    });

    c.bench_function("part-1: h=manhattan", |b| {
        b.iter(|| black_box(a_star(black_box(&grid), start, dst, |p, d| p.manhattan(d))));
    });

    c.bench_function("part-1(pf): h=manhattan", |b| {
        b.iter(|| black_box(a_star_pf(black_box(&grid))));
    });
}

fn benchmark_part_2(c: &mut Criterion) {
    let grid = parse_numeric_grid(load_text_input_from_file("inputs/input.txt"));
    let grid = expand_grid(&grid);
    let start = Position::new(0, 0);
    let dst = Position::new(grid.len() - 1, grid[0].len() - 1);

    c.bench_function("part-2: h=none", |b| {
        b.iter(|| black_box(a_star(black_box(&grid), start, dst, |_, _| 0)));
    });

    c.bench_function("part-2: h=manhattan", |b| {
        b.iter(|| black_box(a_star(black_box(&grid), start, dst, |p, d| p.manhattan(d))));
    });

    c.bench_function("part-2(pf): h=manhattan", |b| {
        b.iter(|| black_box(a_star_pf(black_box(&grid))));
    });
}
