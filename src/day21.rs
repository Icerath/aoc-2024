use std::{hint::assert_unchecked, mem::transmute};

#[inline(always)]
pub fn part1(input: &str) -> u64 {
    unsafe { both_parts(input.as_bytes(), &LUT_P1) }
}
#[inline(always)]
pub fn part2(input: &str) -> u64 {
    unsafe { both_parts(input.as_bytes(), &LUT_P2) }
}

static LUT_P1: [u64; 1000] = unsafe { transmute(*include_bytes!("../luts/21a")) };
static LUT_P2: [u64; 1000] = unsafe { transmute(*include_bytes!("../luts/21b")) };

pub const PART1_OUT: u64 = 136_780;
pub const PART2_OUT: u64 = 167_538_833_832_712;

#[inline(always)]
unsafe fn both_parts(input: &[u8], lut: &[u64; 1000]) -> u64 {
    let input = input.as_ptr();

    let i = |i: usize| (input.add(i).read() - b'0') as usize;

    let n1 = i(0) * 100 + i(1) * 10 + i(2);
    let n2 = i(5) * 100 + i(6) * 10 + i(7);
    let n3 = i(10) * 100 + i(11) * 10 + i(12);
    let n4 = i(15) * 100 + i(16) * 10 + i(17);
    let n5 = i(20) * 100 + i(21) * 10 + i(22);

    macro_rules! assert_lt { ($($idx: ident),+) => { $(assert_unchecked($idx < 1000));+ }; }
    assert_lt!(n1, n2, n3, n4, n5);

    lut[n1] + lut[n2] + lut[n3] + lut[n4] + lut[n5]
}
