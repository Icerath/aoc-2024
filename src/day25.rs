#![expect(static_mut_refs, clippy::cast_possible_truncation)]

use std::simd::{cmp::SimdPartialEq, u8x32, Simd};

pub fn part1(input: &str) -> u32 {
    unsafe { part1_inner(input.as_bytes()) }
}

unsafe fn part1_inner(mut input: &[u8]) -> u32 {
    static mut LOCKS: [u32; 250] = [0; 250];
    static mut KEYS: [u32; 250] = [0; 250];

    let mut num_locks = 0;
    let mut num_keys = 0;
    loop {
        let block = input.as_ptr().add(6).cast::<u8x32>().read_unaligned();
        let mut bits = block.simd_eq(Simd::splat(b'#')).to_bitmask() as u32;
        bits &= (1 << 30) - 1;

        if input[0] == b'#' {
            *LOCKS.get_unchecked_mut(num_locks) = bits;
            num_locks += 1;
        } else {
            *KEYS.get_unchecked_mut(num_keys) = bits;
            num_keys += 1;
        }
        input = match input.get(43..) {
            Some(input) => input,
            None => break,
        };
    }
    let mut num_matching = 0;
    for lock in &LOCKS {
        for key in &KEYS {
            num_matching += (lock & key == 0) as u32;
        }
    }
    num_matching
}

pub const PART1_OUT: u32 = 3255;
