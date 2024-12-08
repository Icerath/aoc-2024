use std::hint::{assert_unchecked, unreachable_unchecked};

macro_rules! impl_part {
    ($input: ident, $check: ident) => {{
        let input = $input.as_bytes();
        let mut sum = 0;
        let mut operands_buf = [0u16; 32];
        let operands_buf: *mut u16 = (&raw mut operands_buf).cast::<u16>();
        let mut num_operands;
        let input_end = input.as_ptr().add(input.len());
        let mut input = input.as_ptr();
        loop {
            num_operands = 0;
            let mut expected = (input.read() - b'0') as u64;
            input = input.add(1);
            while input.read() != b':' {
                expected = expected * 10 + (input.read() - b'0') as u64;
                input = input.add(1);
            }
            input = input.add(2);
            let mut operand = (input.read() - b'0') as u16;
            input = input.add(1);
            loop {
                match input.read() {
                    b'\n' => {
                        *operands_buf.add(num_operands) = operand;
                        num_operands += 1;
                        input = input.add(1);
                        break;
                    }
                    b' ' => {
                        *operands_buf.add(num_operands) = operand;
                        num_operands += 1;
                        operand = 0;
                    }
                    _ => operand = operand * 10 + (input.read() - b'0') as u16,
                }
                input = input.add(1);
            }
            if $check(expected, std::slice::from_raw_parts(operands_buf, num_operands)) {
                sum += expected;
            }
            if input == input_end {
                break;
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

#[inline]
unsafe fn fast_10pow_log10(n: u16) -> u64 {
    match n {
        0..10 => 10,
        10..100 => 100,
        100..1000 => 1000,
        _ => unreachable_unchecked(),
    }
}
