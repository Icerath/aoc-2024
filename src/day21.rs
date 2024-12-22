use std::mem::transmute;

#[inline(never)]
pub fn part1(input: &str) -> u64 {
    unsafe { both_parts(input.as_bytes(), &LUT_P1) }
}
#[inline(never)]
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

    lut.get_unchecked(n1)
        + lut.get_unchecked(n2)
        + lut.get_unchecked(n3)
        + lut.get_unchecked(n4)
        + lut.get_unchecked(n5)
}
