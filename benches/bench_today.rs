use aoc_2024::day19::{part1, part2};
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

fn bench_part1(c: &mut Criterion) {
    let input = include_str!("../input/day19.txt");
    c.bench_function("day19_part1", |b| b.iter(|| part1(black_box(input))));

    assert_eq!(part1(input), 258);
}

fn bench_part2(c: &mut Criterion) {
    let input = include_str!("../input/day19.txt");
    c.bench_function("day19_part2", |b| b.iter(|| part2(black_box(input))));
    assert_eq!(part2(input), 632_423_618_484_345);
}

criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
