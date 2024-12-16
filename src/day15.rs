use std::{
    hint::unreachable_unchecked,
    simd::{cmp::SimdPartialEq, u8x64, Simd},
};

use bstr::ByteSlice;

pub fn part1(input: &str) -> usize {
    unsafe { part1_inner(input.as_bytes()) }
}

unsafe fn part1_inner(input: &[u8]) -> usize {
    let mut grid: [u8; 51 * 50] = input.get_unchecked(..51 * 50).try_into().unwrap_unchecked();
    let mut pos = input.find_byte(b'@').unwrap_unchecked();
    let directions = input.get_unchecked(51 * 50 + 2..);

    'outer: for dir in directions {
        let dir = match dir {
            b'^' => -51,
            b'>' => 1,
            b'v' => 51,
            b'<' => -1,
            _ => continue,
        };

        let next_pos = pos.wrapping_add_signed(dir);
        let mut end_pos = next_pos;

        loop {
            match *grid.get_unchecked(end_pos) {
                b'#' => continue 'outer,
                b'.' => break,
                b'O' => end_pos = end_pos.wrapping_add_signed(dir),
                _ => unreachable_unchecked(),
            }
        }
        *grid.get_unchecked_mut(pos) = *grid.get_unchecked(end_pos);
        *grid.get_unchecked_mut(end_pos) = *grid.get_unchecked(next_pos);
        *grid.get_unchecked_mut(next_pos) = b'@';
        pos = next_pos;
    }
    let mut result = 0;
    for i in 0..50 {
        let line = u8x64::load_or_default(grid.get_unchecked(i * 51..));
        let mut mask = line.simd_eq(Simd::splat(b'O')).to_bitmask();
        mask &= (1 << 50) - 1;
        while mask > 0 {
            let j = mask.trailing_zeros() as usize;
            result += 100 * i + j;
            mask &= mask - 1;
        }
    }
    result
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../input/day15.txt")), 1_412_971);
}
