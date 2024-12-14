use std::{
    hint::unreachable_unchecked,
    simd::{cmp::SimdPartialEq, u8x8, Simd},
};

unsafe fn both_parts<const OFFSET: i64>(input: &[u8]) -> i64 {
    let mut remaining = input;
    let mut result = 0;
    loop {
        let x1 = parse2(remaining.get_unchecked(12..14));
        let y1 = parse2(remaining.get_unchecked(18..20));

        let x2 = parse2(remaining.get_unchecked(12 + 21..14 + 21));
        let y2 = parse2(remaining.get_unchecked(18 + 21..20 + 21));

        let [z1, z2] = parse_pair(remaining.as_ptr().add(9 + 42));
        let [z1, z2] = [z1 + OFFSET, z2 + OFFSET];

        // Help.
        let b = (z2 * x1 - z1 * y1) / (y2 * x1 - x2 * y1);
        let a = (z1 - b * x2) / x1;
        result += if (x1 * a + x2 * b, y1 * a + y2 * b) == (z1, z2) { a * 3 + b } else { 0 };

        if remaining.len() < 80 {
            break;
        }
        let next = u8x8::from_array(remaining.get_unchecked(19 + 42..27 + 42).try_into().unwrap_unchecked());
        let nl_offset = next.simd_eq(Simd::splat(b'\n')).first_set().unwrap() + 19 + 42 + 2;
        remaining = remaining.get_unchecked(nl_offset..);
    }
    result
}

pub fn part1(input: &str) -> i64 {
    unsafe { both_parts::<0>(input.as_bytes()) }
}

pub fn part2(input: &str) -> i64 {
    unsafe { both_parts::<10_000_000_000_000>(input.as_bytes()) }
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../input/day13.txt")), 32067);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("../input/day13.txt")), 92_871_736_253_789);
}

#[inline(always)]
unsafe fn parse2(bytes: &[u8]) -> i64 {
    (*bytes.get_unchecked(0) as i64 * 10 + *bytes.get_unchecked(1) as i64) - (b'0' as i64) * 11
}

#[inline(always)]
unsafe fn parse_pair(mut ptr: *const u8) -> [i64; 2] {
    let mut lhs = 0;
    let mut rhs = 0;

    loop {
        match ptr.read() {
            b',' => break,
            i @ b'0'..=b'9' => lhs = lhs * 10 + (i - b'0') as i64,
            _ => unreachable_unchecked(),
        }
        ptr = ptr.add(1);
    }
    ptr = ptr.add(4);
    loop {
        match ptr.read() {
            b'\n' => break,
            i @ b'0'..=b'9' => rhs = rhs * 10 + (i - b'0') as i64,
            _ => unreachable_unchecked(),
        }
        ptr = ptr.add(1);
    }
    [lhs, rhs]
}
