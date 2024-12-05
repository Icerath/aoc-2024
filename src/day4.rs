use std::{
    hint::unreachable_unchecked,
    ops::{BitAnd, Shl},
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

#[derive(Default)]
struct LineData {
    x: BitSet,
    m: BitSet,
    a: BitSet,
    s: BitSet,
}

#[expect(clippy::needless_range_loop)]
#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn accum(input: &[u8]) -> [LineData; 140] {
    let mut out = std::array::from_fn(|_| LineData::default());

    for line_num in 0..140 {
        let line = &input[line_num * 141..];
        for i in 0..64 {
            match line[i] {
                b'X' => out[line_num].x.0[0] |= 1 << i,
                b'M' => out[line_num].m.0[0] |= 1 << i,
                b'A' => out[line_num].a.0[0] |= 1 << i,
                b'S' => out[line_num].s.0[0] |= 1 << i,
                _ => unsafe { unreachable_unchecked() },
            }
        }
        for i in 0..64 {
            match line[64 + i] {
                b'X' => out[line_num].x.0[1] |= 1 << i,
                b'M' => out[line_num].m.0[1] |= 1 << i,
                b'A' => out[line_num].a.0[1] |= 1 << i,
                b'S' => out[line_num].s.0[1] |= 1 << i,
                _ => unsafe { unreachable_unchecked() },
            }
        }
        for i in 0..12 {
            match line[128 + i] {
                b'X' => out[line_num].x.0[2] |= 1 << i,
                b'M' => out[line_num].m.0[2] |= 1 << i,
                b'A' => out[line_num].a.0[2] |= 1 << i,
                b'S' => out[line_num].s.0[2] |= 1 << i,
                _ => unsafe { unreachable_unchecked() },
            }
        }
    }
    out
}

pub fn part1(input: &str) -> u32 {
    let line_data = unsafe { accum(input.as_bytes()) };

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
    let line_data = unsafe { accum(input.as_bytes()) };
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
