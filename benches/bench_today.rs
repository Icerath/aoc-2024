macro_rules! bench_day {
    ($day: ident) => {
        use aoc_2024::$day::{part1, part2, PART1_OUT, PART2_OUT};
        use criterion::{criterion_group, criterion_main, Criterion};
        use std::hint::black_box;

        fn bench_part1(c: &mut Criterion) {
            let input = include_str!(concat!("../input/", stringify!($day), ".txt"));
            c.bench_function(concat!(stringify!($day), " part1"), |b| b.iter(|| part1(black_box(input))));
            assert_eq!(part1(input), PART1_OUT);
        }

        fn bench_part2(c: &mut Criterion) {
            let input = include_str!("../input/day20.txt");
            c.bench_function(concat!(stringify!($day), " part2"), |b| b.iter(|| part2(black_box(input))));
            assert_eq!(part2(input), PART2_OUT);
        }

        criterion_group!(benches, bench_part1, bench_part2);
        criterion_main!(benches);
    };
}

bench_day!(day20);
