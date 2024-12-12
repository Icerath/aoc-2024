use aoc_2024::day11::{part1, part2};
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

fn bench_part1(c: &mut Criterion) {
    let input = include_str!("../input/day11.txt");
    c.bench_function("day11_part1", |b| b.iter(|| part1(black_box(input))));
}

fn bench_part2(c: &mut Criterion) {
    let input = include_str!("../input/day11.txt");
    c.bench_function("day11_part2", |b| b.iter(|| part2(black_box(input))));
}

criterion_group!(benches, bench_part1, bench_part2,);
criterion_main!(benches);
