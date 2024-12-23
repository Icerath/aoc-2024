use std::{
    hint::assert_unchecked,
    mem::transmute,
    simd::{num::SimdUint, u32x4},
};

pub fn part1(input: &str) -> u64 {
    unsafe { part1_inner(input.as_bytes()) }
}
unsafe fn part1_inner(mut input: &[u8]) -> u64 {
    static LUT: [u32; PRUNE as usize] = unsafe { transmute(*include_bytes!("../luts/d22")) };
    let mut sum = 0;
    while !input.is_empty() {
        let mut number = 0;
        while *input.get_unchecked(0) != b'\n' {
            number = number * 10 + (input.get_unchecked(0) - b'0') as u32;
            input = input.get_unchecked(1..);
        }
        input = input.get_unchecked(1..);
        sum += LUT[number as usize] as u64;
    }
    sum
}

pub fn part2(input: &str) -> u16 {
    unsafe { part2_inner(input.as_bytes()) }
}

unsafe fn part2_inner(mut input: &[u8]) -> u16 {
    let mut sequences = vec![0; P4];
    let mut seen = vec![0u64; P4 / 64];

    while !input.is_empty() {
        let mut number = 0;
        while *input.get_unchecked(0) != b'\n' {
            number = number * 10 + (input.get_unchecked(0) - b'0') as u32;
            input = input.get_unchecked(1..);
        }
        input = input.get_unchecked(1..);
        seen.fill(0);

        let mut prev = (number % 10) as u8;
        let mut changes = u32x4::splat(0);

        for i in 1..4 {
            number = evolve(number);
            let price = (number % 10) as u8;
            changes[i] = difference(prev, price);
            prev = price;
        }
        for _ in 0..1997 {
            number = evolve(number);
            let price = (number % 10) as u8;
            changes = u32x4::from([changes[1], changes[2], changes[3], difference(prev, price)]);

            let index = to_index(u32x4::from(changes));
            assert_unchecked(index < P4);
            assert_unchecked(index / 64 < P4 / 64);

            let is_new = seen[index / 64] & (1 << (index % 64)) == 0;
            seen[index / 64] |= 1 << (index % 64);
            sequences[index] += price as u16 * is_new as u16;
            prev = price;
        }
    }
    *sequences.iter().max().unwrap_unchecked()
}

const P1: u32 = 19u32.pow(1);
const P2: u32 = 19u32.pow(2);
const P3: u32 = 19u32.pow(3);
const P4: usize = 19usize.pow(4);

#[inline(always)]
fn difference(prev: u8, price: u8) -> u32 {
    (9 + price - prev) as u32
}

#[inline(always)]
fn to_index(changes: u32x4) -> usize {
    (changes * const { u32x4::from_array([P3, P2, P1, 1]) }).reduce_sum() as usize
}

#[inline(always)]
fn evolve(mut secret: u32) -> u32 {
    secret = ((secret << 6) ^ secret) & (PRUNE - 1);
    secret = ((secret >> 5) ^ secret) & (PRUNE - 1);
    secret = ((secret << 11) ^ secret) & (PRUNE - 1);
    secret
}
const PRUNE: u32 = 1 << 24; // 16777216

pub const PART1_OUT: u64 = 15_006_633_487;
pub const PART2_OUT: u16 = 1710;
