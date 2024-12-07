#![expect(clippy::cast_possible_truncation)]
use std::hint::{assert_unchecked, unreachable_unchecked};

macro_rules! impl_part {
    ($input: ident, $check: ident) => {{
        let mut sum = 0;
        let mut operands_buf: Vec<u16> = vec![];
        for line in $input[..$input.len() - 1].as_bytes().split(|&b| b == b'\n') {
            let sep = line.iter().position(|&b| b == b':').unwrap();
            let operands = &line[sep + 2..];
            let expected = parse_int(&line[..sep]);
            operands_buf.clear();
            operands_buf.extend(operands.split(|&b| b == b' ').map(|s| parse_int(s) as u16));

            if $check(expected, &operands_buf) {
                sum += expected;
            }
        }
        sum
    }};
}

pub fn part1(input: &str) -> u64 {
    unsafe { impl_part!(input, check_part1) }
}

unsafe fn check_part1(expected: u64, operands: &[u16]) -> bool {
    match *operands {
        [] => unreachable_unchecked(),
        [last] => last as u64 == expected,
        [ref operands @ .., last] => {
            let last = last as u64;
            assert_unchecked(last != 0);
            (expected % last == 0 && check_part1(expected / last, operands))
                || (expected >= last && check_part1(expected - last, operands))
        }
    }
}

#[test]
fn test_part1_example() {
    assert_eq!(part1(include_str!("../input/day7_part1_example")), 3749);
}

#[test]
fn test_part1_input() {
    assert_eq!(part1(include_str!("../input/day7_part1")), 5_837_374_519_342);
}

pub fn part2(input: &str) -> u64 {
    unsafe { impl_part!(input, check_part2) }
}

unsafe fn check_part2(expected: u64, operands: &[u16]) -> bool {
    match *operands {
        [last] => last as u64 == expected,
        [ref operands @ .., last] => {
            assert_unchecked(last != 0);
            (expected % last as u64 == 0 && check_part2(expected / last as u64, operands))
                || (expected >= last as u64 && check_part2(expected - last as u64, operands))
                || {
                    let concat = fast_10pow_log10(last);
                    assert_unchecked(concat != 0);
                    (expected % concat) == last as u64 && check_part2(expected / concat, operands)
                }
        }

        _ => unsafe { unreachable_unchecked() },
    }
}

#[test]
fn test_part2_example() {
    assert_eq!(part2(include_str!("../input/day7_part1_example")), 11387);
}

#[test]
fn test_part2_input() {
    assert_eq!(part2(include_str!("../input/day7_part1")), 492_383_931_650_959);
}

#[inline(always)]
fn parse_int(n: &[u8]) -> u64 {
    unsafe { std::str::from_utf8_unchecked(n) }.parse::<u64>().unwrap()
}

#[inline]
unsafe fn fast_10pow_log10(n: u16) -> u64 {
    match n {
        0..10 => 10,
        10..100 => 100,
        100..1000 => 1000,
        _ => unreachable_unchecked(),
    }
}
