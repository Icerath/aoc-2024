#![expect(clippy::missing_panics_doc)]

use std::simd::{num::SimdUint, u32x4};

pub fn part1(input: &str) -> u64 {
    let mut sum = 0;
    for line in input.lines() {
        let mut secret_number = line.parse::<u32>().unwrap();
        for _ in 0..2000 {
            secret_number = evolve(secret_number);
        }
        sum += secret_number as u64;
    }
    sum
}

pub fn part2(input: &str) -> u32 {
    let mut sequences = vec![0; P4];
    let mut seen = vec![false; P4];
    for line in input.lines() {
        seen.fill(false);
        let mut number = line.parse().unwrap();
        let mut prev = (number % 10) as u8;
        let mut changes = u32x4::splat(0);

        for i in 1..4 {
            number = evolve(number);
            let price = (number % 10) as u8;
            changes[i] = change(prev, price);
            prev = price;
        }
        for _ in 0..1997 {
            number = evolve(number);
            let price = (number % 10) as u8;
            changes = u32x4::from([changes[1], changes[2], changes[3], change(prev, price)]);
            prev = price;
            let index = to_index(u32x4::from(changes));
            assert!(index < P4);

            if std::mem::replace(&mut seen[index], true) {
                continue;
            }
            sequences[index] += price as u32;
        }
    }
    sequences.into_iter().max().unwrap()
}

const P1: u32 = 19u32.pow(1);
const P2: u32 = 19u32.pow(2);
const P3: u32 = 19u32.pow(3);
const P4: usize = 19usize.pow(4);

fn change(prev: u8, price: u8) -> u32 {
    (9 + price - prev) as u32
}

fn to_index(changes: u32x4) -> usize {
    (u32x4::from(changes) * const { u32x4::from_array([P3, P2, P1, 1]) }).reduce_sum() as usize
}

fn evolve(mut secret: u32) -> u32 {
    secret = ((secret.wrapping_mul(64)) ^ secret) % PRUNE;
    secret = ((secret / 32) ^ secret) % PRUNE;
    secret = ((secret.wrapping_mul(2048)) ^ secret) % PRUNE;
    secret
}

const PRUNE: u32 = 1 << 24; // 16777216

pub const PART1_OUT: u64 = 15_006_633_487;
pub const PART2_OUT: u32 = 1710;
