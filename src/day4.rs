use std::{
    ops::{BitAnd, Shl},
    simd::{cmp::SimdPartialEq, u8x16, u8x64},
};

#[derive(Clone, Copy, Default)]
struct BitSet([u64; 3]);

impl BitSet {
    #[inline(always)]
    fn count_ones(&self) -> u32 {
        self.0[0].count_ones() + self.0[1].count_ones() + self.0[2].count_ones()
    }
}

impl Shl<u8> for BitSet {
    type Output = Self;
    #[inline(always)]
    fn shl(mut self, n: u8) -> Self {
        let shift = 64 - n;

        let co0 = self.0[0] >> shift;
        let co1 = self.0[1] >> shift;

        self.0[0] <<= n;
        self.0[1] <<= n;
        self.0[2] <<= n;

        self.0[1] |= co0;
        self.0[2] |= co1;

        self
    }
}

impl BitAnd for BitSet {
    type Output = Self;
    #[inline(always)]
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self.0[0] &= rhs.0[0];
        self.0[1] &= rhs.0[1];
        self.0[2] &= rhs.0[2];
        self
    }
}

#[derive(Default, Clone, Copy)]
struct LineData {
    x: BitSet,
    m: BitSet,
    a: BitSet,
    s: BitSet,
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
#[inline]
unsafe fn accum(input: &[u8]) -> [LineData; 140] {
    let mut out = std::array::from_fn(|_| LineData::default());

    for line_num in 0..140 {
        let line = &input[line_num * 141..];
        let first_block = u8x64::from_array(line[..64].try_into().unwrap());
        out[line_num].x.0[0] = first_block.simd_eq(u8x64::from([b'X'; 64])).to_bitmask();
        out[line_num].m.0[0] = first_block.simd_eq(u8x64::from([b'M'; 64])).to_bitmask();
        out[line_num].a.0[0] = first_block.simd_eq(u8x64::from([b'A'; 64])).to_bitmask();
        out[line_num].s.0[0] = first_block.simd_eq(u8x64::from([b'S'; 64])).to_bitmask();

        let second_block = u8x64::from_array(line[64..128].try_into().unwrap());
        out[line_num].x.0[1] = second_block.simd_eq(u8x64::from([b'X'; 64])).to_bitmask();
        out[line_num].m.0[1] = second_block.simd_eq(u8x64::from([b'M'; 64])).to_bitmask();
        out[line_num].a.0[1] = second_block.simd_eq(u8x64::from([b'A'; 64])).to_bitmask();
        out[line_num].s.0[1] = second_block.simd_eq(u8x64::from([b'S'; 64])).to_bitmask();

        let third_block = u8x16::load_or_default(line[128..140].try_into().unwrap());
        out[line_num].x.0[2] = third_block.simd_eq(u8x16::from([b'X'; 16])).to_bitmask();
        out[line_num].m.0[2] = third_block.simd_eq(u8x16::from([b'M'; 16])).to_bitmask();
        out[line_num].a.0[2] = third_block.simd_eq(u8x16::from([b'A'; 16])).to_bitmask();
        out[line_num].s.0[2] = third_block.simd_eq(u8x16::from([b'S'; 16])).to_bitmask();
    }
    out
}

pub fn part1(input: &str) -> u32 {
    unsafe { part1_inner(input.as_bytes()) }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn part1_inner(input: &[u8]) -> u32 {
    let line_data = accum(input);

    let mut count = 0;
    // vertical
    for [a, b, c, d] in line_data.array_windows::<4>() {
        count += (a.x & b.m & c.a & d.s).count_ones();
        count += (a.s & b.a & c.m & d.x).count_ones();
    }
    // horizontal
    for line in &line_data {
        count += (line.s & (line.a << 1) & (line.m << 2) & (line.x << 3)).count_ones();
        count += (line.x & (line.m << 1) & (line.a << 2) & (line.s << 3)).count_ones();
    }
    // diagonal
    for [a, b, c, d] in line_data.array_windows::<4>() {
        count += (a.x & b.m << 1 & c.a << 2 & d.s << 3).count_ones();
        count += (a.s & b.a << 1 & c.m << 2 & d.x << 3).count_ones();

        count += (d.x & c.m << 1 & b.a << 2 & a.s << 3).count_ones();
        count += (d.s & c.a << 1 & b.m << 2 & a.x << 3).count_ones();
    }
    count
}

#[test]
#[ignore]
fn test_part1_example() {
    let input = include_str!("../input/day4_part1_example");
    assert_eq!(part1(input), 18);
}

#[test]
fn test_part1_input() {
    let input = include_str!("../input/day4_part1");
    assert_eq!(part1(input), 2571);
}

pub fn part2(input: &str) -> u32 {
    unsafe { part2_inner(input.as_bytes()) }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn part2_inner(input: &[u8]) -> u32 {
    let line_data = accum(input);
    let mut count = 0;
    for [a, b, c] in line_data.array_windows::<3>() {
        let mid = b.a << 1;

        let top = a.m & (a.s << 2);
        let bot = c.m & (c.s << 2);
        count += (top & mid & bot).count_ones();

        let top = a.s & (a.m << 2);
        let bot = c.s & (c.m << 2);
        count += (top & mid & bot).count_ones();

        let top = a.m & (a.m << 2);
        let bot = c.s & (c.s << 2);
        count += (top & mid & bot).count_ones();

        let top = a.s & (a.s << 2);
        let bot = c.m & (c.m << 2);
        count += (top & mid & bot).count_ones();
    }
    count
}

#[test]
#[ignore]
fn test_part2_example() {
    let input = include_str!("../input/day4_part1_example");
    assert_eq!(part2(input), 9);
}

#[test]
fn test_part2_input() {
    let input = include_str!("../input/day4_part1");
    assert_eq!(part2(input), 1992);
}
