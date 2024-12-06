use std::cmp::Ordering;

use bstr::ByteSlice;

pub fn part1(input: &str) -> u32 {
    unsafe { generic_impl::<true>(input) }
}

#[test]
fn test_part1_example() {
    assert_eq!(part1(include_str!("../input/day5_part1_example")), 143);
}

#[test]
fn test_part1_input() {
    assert_eq!(part1(include_str!("../input/day5_part1")), 6260);
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn generic_impl<const IS_PART1: bool>(input: &str) -> u32 {
    let input = input.as_bytes();
    let mut map: [[bool; 100]; 100] = [[false; 100]; 100];
    let mut sum = 0;
    let rules_end = input.find(b"\n\n").unwrap();
    let mut rules_text = &input[..rules_end];

    while rules_text.len() > 32 {
        let block: [u8; 32] = rules_text[..32].try_into().unwrap();
        rules_text = &rules_text[30..];

        for i in 0..5 {
            let i = i * 6;
            let lhs = (block[i] - b'0') * 10 + (block[i + 1] - b'0');
            let rhs = (block[i + 3] - b'0') * 10 + (block[i + 4] - b'0');
            map[lhs as usize][rhs as usize] = true;
        }
    }
    let mut rem = &input[rules_end - rules_text.len()..];
    loop {
        let offset = 5;
        let line = &rem[..offset];
        rem = &rem[offset + 1..];
        let lhs = (line[0] - b'0') * 10 + line[1] - b'0';
        let rhs = (line[3] - b'0') * 10 + line[4] - b'0';
        map[lhs as usize][rhs as usize] = true;

        if rem[0] == b'\n' {
            rem = &rem[1..];
            break;
        }
    }

    loop {
        let Some(offset) = rem.find_byte(b'\n') else { break };
        let line = &rem[..offset];
        rem = &rem[offset + 1..];
        let mut update = [0u8; 24];
        let max_i = line.len() / 3;
        for i in 0..max_i + 1 {
            update[i] = (line[i * 3] - b'0') * 10 + line[i * 3 + 1] - b'0';
        }
        let old_update = update;
        let old_update = &old_update[..=max_i];
        let update = &mut update[..=max_i];
        if IS_PART1 {
            if !old_update.is_sorted_by(|&lhs, &rhs| map[lhs as usize][rhs as usize]) {
                continue;
            }
        } else if old_update.is_sorted_by(|&lhs, &rhs| map[lhs as usize][rhs as usize]) {
            continue;
        }
        update.sort_unstable_by(
            |&lhs, &rhs| {
                if map[lhs as usize][rhs as usize] {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            },
        );

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
    unsafe { generic_impl::<false>(input) }
}

#[test]
fn test_part2_example() {
    assert_eq!(part2(include_str!("../input/day5_part1_example")), 123);
}

#[test]
fn test_part2_input() {
    assert_eq!(part2(include_str!("../input/day5_part1")), 5346);
}
