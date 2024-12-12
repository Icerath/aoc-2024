use std::hint::unreachable_unchecked;

use rustc_hash::{FxBuildHasher, FxHashMap as HashMap};

pub fn part1(input: &str) -> u64 {
    unsafe { both_parts::<25>(input.as_bytes()) }
}

pub fn part2(input: &str) -> u64 {
    unsafe { both_parts::<75>(input.as_bytes()) }
}

#[inline(always)]
unsafe fn both_parts<const BLINKS: u8>(input: &[u8]) -> u64 {
    let mut stones = HashMap::with_capacity_and_hasher(7168, FxBuildHasher);
    let mut new_stones = HashMap::with_capacity_and_hasher(7168, FxBuildHasher);
    let mut num = 0;

    let mut input = input.as_ptr();
    loop {
        match input.read() {
            b'0'..=b'9' => num = (num * 10) + (input.read() - b'0') as u64,
            b' ' => *stones.entry(std::mem::take(&mut num)).or_default() += 1,
            b'\n' => break,
            _ => unreachable_unchecked(),
        }
        input = input.add(1);
    }
    *stones.entry(num).or_default() += 1;

    for _ in 0..BLINKS {
        for (&stone, &count) in &stones {
            if stone == 0 {
                *new_stones.entry(1).or_default() += count;
            } else {
                let digits = stone.ilog10() + 1;
                if digits % 2 == 0 {
                    let power = 10u64.pow(digits / 2);
                    *new_stones.entry(stone / power).or_default() += count;
                    *new_stones.entry(stone % power).or_default() += count;
                } else {
                    *new_stones.entry(stone * 2024).or_default() += count;
                }
            }
        }
        std::mem::swap(&mut stones, &mut new_stones);
        new_stones.clear();
    }
    stones.values().sum()
}

#[test]
fn test_part1_input() {
    assert_eq!(part1(include_str!("../input/day11_part1")), 194_557);
}

#[test]
fn test_part2_input() {
    assert_eq!(part2(include_str!("../input/day11_part1")), 231_532_558_973_909);
}
