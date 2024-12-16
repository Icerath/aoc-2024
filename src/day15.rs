#![allow(clippy::cast_possible_truncation)]

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
    let directions = input.get_unchecked(51 * 50 + 1..);

    let mut i = 0;
    for _ in 0..20 {
        for _ in 0..1000 {
            let dir = *directions.get_unchecked(i);
            let dir = *P1_DIRECTION_LUT.get_unchecked(dir as usize);
            i += 1;

            let next_pos = pos.wrapping_add_signed(dir);
            let mut end_pos = next_pos;
            while *grid.get_unchecked(end_pos) == b'O' {
                end_pos = end_pos.wrapping_add_signed(dir);
            }
            if *grid.get_unchecked(end_pos) == b'#' {
                continue;
            }
            *grid.get_unchecked_mut(pos) = *grid.get_unchecked(end_pos);
            *grid.get_unchecked_mut(end_pos) = *grid.get_unchecked(next_pos);
            *grid.get_unchecked_mut(next_pos) = b'@';
            pos = next_pos;
        }
        i += 1;
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

pub fn part2(input: &str) -> usize {
    unsafe { part2_inner(input.as_bytes()) }
}

unsafe fn part2_inner(input: &[u8]) -> usize {
    let directions = input.get_unchecked(51 * 50 + 1..);
    let pos = input.find_byte(b'@').unwrap_unchecked();
    let mut pos = pos % 51 * 2 + pos / 51 * 100;

    let mut grid = [0; 100 * 50];

    for i in 0..50 {
        for j in 0..50 {
            let new = match input.get_unchecked(i * 51 + j) {
                b'.' => b"..",
                b'#' => b"##",
                b'O' => b"[]",
                b'@' => b"@.",
                _ => unreachable_unchecked(),
            };
            grid[i * 100 + j * 2..i * 100 + j * 2 + 2].copy_from_slice(new);
        }
    }

    let mut i = 0;
    for _ in 0..20 {
        for _ in 0..1000 {
            let dir = *P2_DIRECTION_LUT.get_unchecked(directions[i] as usize);
            i += 1;
            let next_pos = pos.wrapping_add_signed(dir);
            if !can_push_box(&grid, next_pos, dir) {
                continue;
            };
            push_box(&mut grid, next_pos, dir);
            *grid.get_unchecked_mut(next_pos) = b'@';
            *grid.get_unchecked_mut(pos) = b'.';
            pos = next_pos;
        }
        i += 1;
    }
    let mut result = 0;
    for i in 1..49 {
        for j in 1..99 {
            if *grid.get_unchecked(i * 100 + j) == b'[' {
                result += 100 * i + j;
            }
        }
    }
    result
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../input/day15.txt")), 1_412_971);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("../input/day15.txt")), 1_429_299);
}

#[inline(always)]
unsafe fn can_push_box(grid: &[u8; 100 * 50], pos: usize, dir: isize) -> bool {
    if dir.abs() == 1 {
        can_push_box_horizontal(grid, pos, dir)
    } else {
        can_push_box_vertical(grid, pos, dir)
    }
}

#[inline(always)]
unsafe fn push_box(grid: &mut [u8; 100 * 50], pos: usize, dir: isize) {
    if dir.abs() == 1 {
        push_box_horizontal(grid, pos, dir);
    } else {
        push_box_vertical(grid, pos, dir);
    }
}

unsafe fn can_push_box_horizontal(grid: &[u8; 100 * 50], mut pos: usize, dir: isize) -> bool {
    loop {
        match *grid.get_unchecked(pos) {
            b'#' => return false,
            b'.' => return true,
            b'[' | b']' => pos = pos.wrapping_add_signed(dir),
            _ => unreachable_unchecked(),
        }
    }
}

unsafe fn can_push_box_vertical(grid: &[u8; 100 * 50], pos: usize, dir: isize) -> bool {
    match *grid.get_unchecked(pos) {
        b'#' => false,
        b'.' => true,
        b'[' => {
            can_push_box_vertical(grid, pos.wrapping_add_signed(dir), dir)
                && can_push_box_vertical(grid, pos.wrapping_add_signed(dir + 1), dir)
        }
        b']' => {
            can_push_box_vertical(grid, pos.wrapping_add_signed(dir), dir)
                && can_push_box_vertical(grid, pos.wrapping_add_signed(dir - 1), dir)
        }
        _ => unreachable_unchecked(),
    }
}

unsafe fn push_box_horizontal(grid: &mut [u8; 100 * 50], pos: usize, dir: isize) {
    if *grid.get_unchecked(pos) == b'.' {
        return;
    }
    let next_pos = pos.wrapping_add_signed(dir);
    push_box_horizontal(grid, next_pos, dir);
    *grid.get_unchecked_mut(next_pos) = *grid.get_unchecked(pos);
    *grid.get_unchecked_mut(pos) = b'.';
}

unsafe fn push_box_vertical(grid: &mut [u8; 100 * 50], pos: usize, dir: isize) {
    match *grid.get_unchecked(pos) {
        b'.' => {}
        b']' => push_box_vertical(grid, pos - 1, dir),
        b'[' => {
            let next_pos = pos.wrapping_add_signed(dir);
            push_box_vertical(grid, next_pos, dir);
            push_box_vertical(grid, next_pos + 1, dir);
            grid.get_unchecked_mut(next_pos..next_pos + 2).copy_from_slice(b"[]");
            grid.get_unchecked_mut(pos..pos + 2).copy_from_slice(b"..");
        }
        _ => unreachable_unchecked(),
    }
}

macro_rules! direction_lut {
    ($vert: literal) => {{
        let mut lut = [0; LARGEST_DIRECTION];
        let mut i = 0;
        while i < LARGEST_DIRECTION {
            lut[i] = match i as u8 {
                b'^' => -$vert,
                b'>' => 1,
                b'v' => $vert,
                b'<' => -1,
                _ => 0,
            };
            i += 1;
        }
        lut
    }};
}

const LARGEST_DIRECTION: usize = b'v' as usize + 1;
static P1_DIRECTION_LUT: [isize; LARGEST_DIRECTION] = direction_lut!(51);
static P2_DIRECTION_LUT: [isize; LARGEST_DIRECTION] = direction_lut!(100);
