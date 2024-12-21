#![expect(static_mut_refs)]

use bstr::ByteSlice;
use std::{hint::assert_unchecked, ops::Index};

const GRID_SIZE: usize = LINE_LEN * GRID_LEN;
const LINE_END: usize = GRID_LEN - 1;
const GRID_LEN: usize = LINE_LEN - 1;
const LINE_LEN: usize = 142;

const MIN_SPEEDUP: u16 = 100;
static mut ELAPSED: [u16; GRID_SIZE] = [u16::MAX; GRID_SIZE];

const DIRECTIONS: [isize; 4] = [-142, 1, 142, -1];
const CYCLE_RIGHT: [usize; 4] = [3, 0, 1, 2];
const CYCLE_LEFT: [usize; 4] = [1, 2, 3, 0];

#[inline(never)]
unsafe fn fill_grid(input: &[u8]) {
    let mut current = input.find_byte(b'S').unwrap_unchecked();
    let end = input.find_byte(b'E').unwrap_unchecked();

    let mut dir = (0..4)
        .find(|&dir| *input.get_unchecked(current.wrapping_add_signed(DIRECTIONS[dir])) != b'#')
        .unwrap_unchecked();

    for elapsed in 0.. {
        *ELAPSED.get_unchecked_mut(current) = elapsed;
        if current == end {
            break;
        }
        assert_unchecked(dir < 4);
        if *input.get_unchecked(current.wrapping_add_signed(DIRECTIONS[CYCLE_RIGHT[dir]])) != b'#' {
            dir = CYCLE_RIGHT[dir];
        } else if *input.get_unchecked(current.wrapping_add_signed(DIRECTIONS[CYCLE_LEFT[dir]])) != b'#' {
            dir = CYCLE_LEFT[dir];
        }
        current = current.wrapping_add_signed(DIRECTIONS[dir]);
    }
}

unsafe fn part1_inner(input: &[u8]) -> u16 {
    fill_grid(input);
    let mut sum = 0;
    for y in 1..LINE_END {
        for x in 1..LINE_END {
            let pos = Pos { x, y };
            if ELAPSED[pos] != u16::MAX {
                if pos.x < 138 {
                    sum += is_faster(&ELAPSED, pos, Pos { x: pos.x + 2, y: pos.y }) as u16;
                }
                if pos.y < 138 {
                    sum += is_faster(&ELAPSED, pos, Pos { x: pos.x, y: pos.y + 2 }) as u16;
                }
            }
        }
    }
    sum
}

unsafe fn part2_inner(input: &[u8]) -> u32 {
    fill_grid(input);
    let mut sum = 0;
    for y in 1..LINE_END {
        for x in 1..LINE_END {
            let pos = Pos { x, y };
            if ELAPSED[pos] == u16::MAX {
                continue;
            }
            for y in 1..(21.min(LINE_END - pos.y)) {
                for x in (pos.x.saturating_sub(20 - y))..(LINE_END.min(pos.x + (21 - y))) {
                    sum += is_faster(&ELAPSED, pos, Pos { x, y: pos.y + y }) as u32;
                }
            }
            for x in 2..(21.min(LINE_END - pos.x)) {
                sum += is_faster(&ELAPSED, pos, Pos { x: pos.x + x, y: pos.y }) as u32;
            }
        }
    }
    sum
}

#[expect(clippy::cast_possible_truncation)]
unsafe fn is_faster(elapsed: &[u16], from: Pos, to: Pos) -> bool {
    if elapsed[to] == u16::MAX {
        return false;
    }
    let elapsed_distance = elapsed[from].abs_diff(elapsed[to]);
    let physical_distance = (from.x.abs_diff(to.x) + from.y.abs_diff(to.y)) as u16;
    (elapsed_distance - physical_distance) >= MIN_SPEEDUP
}

#[derive(Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

impl<T> Index<Pos> for [T] {
    type Output = T;
    #[inline(always)]
    fn index(&self, index: Pos) -> &Self::Output {
        // cursed
        unsafe { self.get_unchecked(LINE_LEN * index.y + index.x) }
    }
}

pub fn part1(input: &str) -> u16 {
    unsafe { part1_inner(input.as_bytes()) }
}

pub fn part2(input: &str) -> u32 {
    unsafe { part2_inner(input.as_bytes()) }
}

#[test]
fn test_part1() {
    let _guard = LOCK.lock();
    assert_eq!(part1(include_str!("../input/day20.txt")), 1338);
}

#[test]
fn test_part2() {
    let _guard = LOCK.lock();
    assert_eq!(part2(include_str!("../input/day20.txt")), 975_376);
}

#[cfg(test)]
static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
