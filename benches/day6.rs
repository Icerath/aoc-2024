use aoc_2024::day6::part1;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_part1(c: &mut Criterion) {
    let input = include_str!("../input/day6_part1");
    c.bench_function("day6_part1", |b| b.iter(|| part1(black_box(input))));
}

criterion_group!(benches, bench_part1);
criterion_main!(benches);
