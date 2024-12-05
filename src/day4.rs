use std::ops::{BitAnd, Shl};

#[derive(Clone, Copy, Default)]
struct BitSet([u64; 3]);

impl BitSet {
    fn set_bit(&mut self, bit: u8) {
        let shift = bit % 64;
        let int_index = (bit / 64) as usize;
        let int = &mut self.0[int_index];
        *int |= 1 << shift;
    }
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

fn accum(input: &[u8]) -> Box<[LineData]> {
    let mut out = vec![LineData::default()];

    let mut line_num = 0;
    let mut line_index = 0;
    let mut i = 0;
    while i < input.len() {
        match input[i] {
            b'\n' => {
                out.push(LineData::default());
                line_num += 1;
                line_index = 0;
                i += 1;
                continue;
            }
            b'X' => out[line_num].x.set_bit(line_index),
            b'M' => out[line_num].m.set_bit(line_index),
            b'A' => out[line_num].a.set_bit(line_index),
            b'S' => out[line_num].s.set_bit(line_index),
            _ => unreachable!(),
        }
        i += 1;
        line_index += 1;
    }
    out.into()
}

pub fn part1(input: &str) -> u32 {
    let input = input.as_bytes();
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
fn test_part1_example() {
    let input = include_str!("../input/day4_part1_example");
    assert_eq!(part1(input), 18);
}

#[test]
fn test_part1_input() {
    let input = include_str!("../input/day4_part1");
    assert_eq!(part1(input), 2571);
}
