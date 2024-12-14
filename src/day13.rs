use std::hint::unreachable_unchecked;

unsafe fn both_parts<const OFFSET: i64>(input: &[u8]) -> i64 {
    let mut remaining = input;
    let mut result = 0;
    loop {
        let x1 = parse2(remaining.get_unchecked(12..14));
        let y1 = parse2(remaining.get_unchecked(18..20));

        let x2 = parse2(remaining.get_unchecked(12 + 21..14 + 21));
        let y2 = parse2(remaining.get_unchecked(18 + 21..20 + 21));

        remaining = remaining.get_unchecked(9 + 42..);
        let [z1, z2] = parse_pair(&mut remaining);
        let [z1, z2] = [z1 + OFFSET, z2 + OFFSET];

        let b = (z2 * x1 - z1 * y1).checked_div(y2 * x1 - x2 * y1).unwrap_unchecked();
        let a = (z1 - b * x2).checked_div(x1).unwrap_unchecked();
        result += if (x1 * a + x2 * b, y1 * a + y2 * b) == (z1, z2) { a * 3 + b } else { 0 };
        if remaining.len() < 2 {
            break;
        }
        remaining = remaining.get_unchecked(2..);
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
unsafe fn parse_pair(ptr: &mut &[u8]) -> [i64; 2] {
    let mut lhs = parse2(ptr);
    *ptr = ptr.get_unchecked(2..);

    loop {
        match ptr.get_unchecked(0) {
            b',' => break,
            i @ b'0'..=b'9' => lhs = lhs * 10 + (i - b'0') as i64,
            _ => unreachable_unchecked(),
        }
        *ptr = ptr.get_unchecked(1..);
    }
    *ptr = ptr.get_unchecked(4..);
    let mut rhs = parse2(ptr);

    *ptr = ptr.get_unchecked(2..);
    loop {
        match ptr.get_unchecked(0) {
            b'\n' => break,
            i @ b'0'..=b'9' => rhs = rhs * 10 + (i - b'0') as i64,
            _ => unreachable_unchecked(),
        }
        *ptr = ptr.get_unchecked(1..);
    }
    [lhs, rhs]
}
