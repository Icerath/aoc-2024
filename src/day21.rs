#[inline(always)]
pub fn part1(input: &str) -> u64 {
    unsafe { both_parts(input.as_ptr(), &LUT_P1) }
}
#[inline(always)]
pub fn part2(input: &str) -> u64 {
    unsafe { both_parts(input.as_ptr(), &LUT_P2) }
}

static LUT_P1: [u64; 1000] = include!("../luts/21a");
static LUT_P2: [u64; 1000] = include!("../luts/21b");

pub const PART1_OUT: u64 = 136_780;
pub const PART2_OUT: u64 = 167_538_833_832_712;

#[inline(always)]
unsafe fn both_parts(input: *const u8, lut: &[u64; 1000]) -> u64 {
    const ZERO: usize = b'0' as usize * 111;
    let i = |i| (input.add(i).read()) as usize;

    *lut.get_unchecked(i(0) * 100 + i(1) * 10 + i(2) - ZERO)
        + *lut.get_unchecked(i(5) * 100 + i(6) * 10 + i(7) - ZERO)
        + *lut.get_unchecked(i(10) * 100 + i(11) * 10 + i(12) - ZERO)
        + *lut.get_unchecked(i(15) * 100 + i(16) * 10 + i(17) - ZERO)
        + *lut.get_unchecked(i(20) * 100 + i(21) * 10 + i(22) - ZERO)
}
