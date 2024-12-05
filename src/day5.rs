use std::{cmp::Ordering, collections::HashMap};

#[expect(clippy::missing_panics_doc)]
pub fn part1(input: &str) -> u32 {
    let mut lines = input.lines();

    let mut data = HashMap::<u8, Vec<u8>>::new();

    let mut sum = 0;
    for line in &mut lines {
        if line.is_empty() {
            break;
        }
        let [lhs, rhs] = [&line[..2], &line[3..]].map(|s| s.parse().unwrap());
        data.entry(lhs).or_default().push(rhs);
    }

    for update in lines {
        if update.is_empty() {
            continue;
        }
        let mut update = update.split(',').map(|s| s.parse::<u8>().expect(s)).collect::<Vec<_>>();
        let old_update = update.clone();
        update.sort_by(|lhs, rhs| match data.get(lhs) {
            Some(nums) if nums.contains(rhs) => Ordering::Less,
            _ => match data.get(rhs) {
                Some(nums) if nums.contains(lhs) => Ordering::Greater,
                _ => Ordering::Equal,
            },
        });
        if update == old_update {
            sum += old_update[old_update.len() / 2] as u32;
        }
    }
    sum
}

#[test]
fn test_part1_example() {
    assert_eq!(part1(include_str!("../input/day5_part1_example")), 143);
}

#[test]
fn test_part1_input() {
    assert_eq!(part1(include_str!("../input/day5_part1")), 6260);
}

#[expect(clippy::missing_panics_doc)]
pub fn part2(input: &str) -> u32 {
    let mut lines = input.lines();

    let mut data = HashMap::<u8, Vec<u8>>::new();

    let mut sum = 0;
    for line in &mut lines {
        if line.is_empty() {
            break;
        }
        let [lhs, rhs] = [&line[..2], &line[3..]].map(|s| s.parse().unwrap());
        data.entry(lhs).or_default().push(rhs);
    }

    for update in lines {
        if update.is_empty() {
            continue;
        }
        let mut update = update.split(',').map(|s| s.parse::<u8>().expect(s)).collect::<Vec<_>>();
        let old_update = update.clone();
        update.sort_by(|lhs, rhs| match data.get(lhs) {
            Some(nums) if nums.contains(rhs) => Ordering::Less,
            _ => match data.get(rhs) {
                Some(nums) if nums.contains(lhs) => Ordering::Greater,
                _ => Ordering::Equal,
            },
        });
        if update != old_update {
            sum += update[old_update.len() / 2] as u32;
        }
    }
    sum
}

#[test]
fn test_part2_example() {
    assert_eq!(part2(include_str!("../input/day5_part1_example")), 123);
}

#[test]
fn test_part2_input() {
    assert_eq!(part2(include_str!("../input/day5_part1")), 5346);
}
