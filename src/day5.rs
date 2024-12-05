use core::slice::memchr;
use std::cmp::Ordering;

pub fn part1(input: &str) -> u32 {
    generic_impl::<true>(input)
}

#[test]
fn test_part1_example() {
    assert_eq!(part1(include_str!("../input/day5_part1_example")), 143);
}

#[test]
fn test_part1_input() {
    assert_eq!(part1(include_str!("../input/day5_part1")), 6260);
}

fn generic_impl<const IS_PART1: bool>(input: &str) -> u32 {
    let input = input.as_bytes();
    let mut map: [[bool; 100]; 100] = [[false; 100]; 100];
    let mut sum = 0;
    let mut rem = input;
    loop {
        let offset = memchr::memchr(b'\n', rem).unwrap();
        let line = &rem[..offset];
        rem = &rem[offset + 1..];
        if line.is_empty() {
            break;
        }
        let lhs = (line[0] - b'0') * 10 + line[1] - b'0';
        let rhs = (line[3] - b'0') * 10 + line[4] - b'0';
        map[lhs as usize][rhs as usize] = true;
    }
    loop {
        let Some(offset) = memchr::memchr(b'\n', rem) else { break };
        let line = &rem[..offset];
        rem = &rem[offset + 1..];
        let mut update = [0u8; 32];
        let mut max_i = 0;
        for (i, n) in line.split(|&b| b == b',').enumerate() {
            update[i] = (n[0] - b'0') * 10 + n[1] - b'0';
            max_i = i;
        }
        let old_update = update;
        let old_update = &old_update[..=max_i];
        let update = &mut update[..=max_i];
        update.sort_by(|&lhs, &rhs| {
            if map[lhs as usize][rhs as usize] {
                Ordering::Less
            } else if map[rhs as usize][rhs as usize] {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });

        if IS_PART1 {
            if update == old_update {
                sum += old_update[old_update.len() / 2] as u32;
            }
        } else if update != old_update {
            sum += update[update.len() / 2] as u32;
        }
    }
    sum
}

pub fn part2(input: &str) -> u32 {
    generic_impl::<false>(input)
}

#[test]
fn test_part2_example() {
    assert_eq!(part2(include_str!("../input/day5_part1_example")), 123);
}

#[test]
fn test_part2_input() {
    assert_eq!(part2(include_str!("../input/day5_part1")), 5346);
}
