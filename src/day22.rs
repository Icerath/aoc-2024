#![expect(clippy::missing_panics_doc)]
use std::collections::{HashMap, HashSet};

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

#[expect(clippy::cast_possible_wrap)]
pub fn part2(input: &str) -> u32 {
    let mut sequences = HashMap::new();
    let mut seen = HashSet::new();
    for line in input.lines() {
        seen.clear();
        let mut number = line.parse().unwrap();
        let mut prev = (number % 10) as u8;
        let mut changes = [0; 4];

        for i in 1..4 {
            number = evolve(number);
            let price = (number % 10) as u8;
            changes[i] = price as i8 - prev as i8;
            prev = price;
        }
        for _ in 0..1997 {
            number = evolve(number);
            let price = (number % 10) as u8;
            changes = [changes[1], changes[2], changes[3], price as i8 - prev as i8];
            prev = price;

            if !seen.insert(changes) {
                continue;
            }
            *sequences.entry(changes).or_default() += price as u32;
        }
    }
    *sequences.values().max().unwrap()
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
