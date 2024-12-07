use std::hint::unreachable_unchecked;

macro_rules! impl_part {
    ($input: ident, $check: ident) => {{
        let mut sum = 0;
        let mut operands_buf: Vec<u64> = vec![];
        for line in $input[..$input.len() - 1].as_bytes().split(|&b| b == b'\n') {
            let sep = line.iter().position(|&b| b == b':').unwrap();
            let operands = &line[sep + 2..];
            let expected = parse_int(&line[..sep]);

            operands_buf.clear();
            operands_buf.extend(operands.split(|&b| b == b' ').map(parse_int));

            if $check(expected, &operands_buf) {
                sum += expected;
            }
        }
        sum
    }};
}

pub fn part1(input: &str) -> u64 {
    impl_part!(input, check_part1)
}

fn check_part1(expected: u64, operands: &[u64]) -> bool {
    match operands {
        [] => unsafe { unreachable_unchecked() },
        [last] => *last == expected,
        [operands @ .., last] => {
            (expected % last == 0 && check_part1(expected / last, operands))
                || (expected >= *last && check_part1(expected - last, operands))
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
    impl_part!(input, check_part2)
}

fn check_part2(expected: u64, operands: &[u64]) -> bool {
    match operands {
        [last] => *last == expected,
        [operands @ .., last] => {
            (expected % last == 0 && check_part2(expected / last, operands))
                || (expected >= *last && check_part2(expected - last, operands))
                || {
                    let concat = 10_u64.pow(last.ilog10() + 1);
                    (expected % concat) == *last && check_part2(expected / concat, operands)
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
