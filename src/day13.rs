macro_rules! impl_part {
    ($input: ident, $offset: literal, $ty: ty) => {{
        let mut remaining = $input.as_ptr();
        let end = $input.as_ptr().add($input.len());
        let mut result = 0;
        while (remaining as usize) < end as usize {
            let x1 = parse2(remaining.add(12)) as $ty;
            let y1 = parse2(remaining.add(18)) as $ty;

            let x2 = parse2(remaining.add(12 + 21)) as $ty;
            let y2 = parse2(remaining.add(18 + 21)) as $ty;

            remaining = remaining.add(9 + 42);
            let [z1, z2] = parse_pair(&mut remaining);
            let [z1, z2] = [z1 as $ty + $offset, z2 as $ty + $offset];

            let b = (z2 * x1 - z1 * y1).checked_div(y2 * x1 - x2 * y1).unwrap_unchecked();
            let a = (z1 - b * x2).checked_div(x1).unwrap_unchecked();
            let res = a * 3 + b;
            result += res * ((x1 * a + x2 * b, y1 * a + y2 * b) == (z1, z2)) as $ty;
            // using ptr::add here would be UB
            remaining = remaining.wrapping_add(2);
        }
        result
    }};
}

pub fn part1(input: &str) -> i32 {
    unsafe { impl_part!(input, 0, i32) }
}

pub fn part2(input: &str) -> i64 {
    unsafe { impl_part!(input, 10_000_000_000_000, i64) }
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
unsafe fn parse2(bytes: *const u8) -> i32 {
    (bytes.read() as i32 * 10 + bytes.add(1).read() as i32) - (b'0' as i32 * 11)
}

#[inline(always)]
unsafe fn parse_pair(ptr: &mut *const u8) -> [i32; 2] {
    let mut lhs = parse2(*ptr);
    *ptr = ptr.add(2);

    while ptr.read() != b',' {
        lhs = lhs * 10 + (ptr.read() - b'0') as i32;
        *ptr = ptr.add(1);
    }
    *ptr = ptr.add(4);

    let mut rhs = parse2(*ptr);
    *ptr = ptr.add(2);

    while ptr.read() != b'\n' {
        rhs = rhs * 10 + (ptr.read() - b'0') as i32;
        *ptr = ptr.add(1);
    }
    [lhs, rhs]
}
