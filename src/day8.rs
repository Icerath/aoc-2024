use std::simd::{cmp::SimdPartialEq, u8x64};

const MAX_PAIRS: usize = 16;

#[expect(clippy::cast_possible_truncation, clippy::cast_possible_wrap, clippy::cast_sign_loss)]
unsafe fn both_parts<const PART1: bool>(input: &[u8]) -> u32 {
    let mut antennas = [0u16; 62 * MAX_PAIRS];
    let mut counts = [0u8; 62];
    for y in 0..50 {
        let chunk = input.get_unchecked(y as usize * 51..);
        let block = if y == 49 {
            let chunk = &chunk.get_unchecked(..50);
            u8x64::load_or_default(chunk)
        } else {
            let chunk = chunk.get_unchecked(..64).try_into().unwrap_unchecked();
            u8x64::from_array(chunk)
        };
        let mut non_dots = block.simd_ne(u8x64::splat(b'.')).to_bitmask();
        non_dots &= (1 << 50) - 1;
        while non_dots != 0 {
            let x = non_dots.trailing_zeros() as u8;
            non_dots &= non_dots - 1;

            let count_idx = byte_compress(block[x as usize]) as usize;
            let antenna_idx = count_idx * MAX_PAIRS + (*counts.get_unchecked(count_idx) as usize);
            *counts.get_unchecked_mut(count_idx) += 1;
            *antennas.get_unchecked_mut(antenna_idx) = u16::from_ne_bytes([y, x]);
        }
    }
    let mut antinodes = [0u64; 64];
    for (i, &count) in counts.iter().enumerate() {
        if count == 0 {
            continue;
        }
        let i = i * MAX_PAIRS;
        for j in 0..count - 1 {
            for k in j + 1..count {
                let [lhs_y, lhs_x] = antennas.get_unchecked(i + j as usize).to_ne_bytes();
                let [rhs_y, rhs_x] = antennas.get_unchecked(i + k as usize).to_ne_bytes();

                let [lhs_y, lhs_x] = [lhs_y as i8, lhs_x as i8];
                let [rhs_y, rhs_x] = [rhs_y as i8, rhs_x as i8];

                let [dy, dx] = [lhs_y - rhs_y, lhs_x - rhs_x];

                if PART1 {
                    let y = lhs_y + dy;
                    let x = lhs_x + dx;
                    if (0..50).contains(&y) && (0..50).contains(&x) {
                        *antinodes.get_unchecked_mut(y as usize) |= 1 << x;
                    }
                    let y = rhs_y - dy;
                    let x = rhs_x - dx;
                    if (0..50).contains(&y) && (0..50).contains(&x) {
                        *antinodes.get_unchecked_mut(y as usize) |= 1 << x;
                    }
                } else {
                    *antinodes.get_unchecked_mut(lhs_y as usize) |= 1 << lhs_x;
                    *antinodes.get_unchecked_mut(rhs_y as usize) |= 1 << rhs_x;
                    for i in 1..49 {
                        let y = lhs_y + i * dy;
                        let x = lhs_x + i * dx;
                        if !(0..50).contains(&y) || !(0..50).contains(&x) {
                            break;
                        }
                        *antinodes.get_unchecked_mut(y as usize) |= 1 << x;
                    }
                    for i in 1..49 {
                        let y = lhs_y - i * dy;
                        let x = lhs_x - i * dx;
                        if !(0..50).contains(&y) || !(0..50).contains(&x) {
                            break;
                        }
                        *antinodes.get_unchecked_mut(y as usize) |= 1 << x;
                    }
                }
            }
        }
    }
    antinodes.into_iter().map(u64::count_ones).sum()
}
pub fn part1(input: &str) -> u32 {
    unsafe { both_parts::<true>(input.as_bytes()) }
}

#[test]
#[ignore]
fn test_part1_example() {
    assert_eq!(part1(include_str!("../input/day8_part1_example")), 14);
}

#[test]
fn test_part1_input() {
    assert_eq!(part1(include_str!("../input/day8_part1")), 329);
}

pub fn part2(input: &str) -> u32 {
    unsafe { both_parts::<false>(input.as_bytes()) }
}

#[test]
#[ignore]
fn test_part2_example() {
    assert_eq!(part2(include_str!("../input/day8_part1_example")), 34);
}

#[test]
fn test_part2_input() {
    assert_eq!(part2(include_str!("../input/day8_part1")), 1190);
}

#[inline(always)]
unsafe fn byte_compress(byte: u8) -> u8 {
    const LUT: [u8; 255] = {
        let mut lut = [0; 255];
        let mut i = 0u8;
        while i < 255 {
            lut[i as usize] = match i {
                b'0'..=b'9' => i - b'0',
                b'A'..=b'Z' => (i - b'A') + 10,
                b'a'..=b'z' => (i - b'a') + 10 + 26,
                _ => 0,
            };
            i += 1;
        }
        lut
    };
    LUT[byte as usize]
}
