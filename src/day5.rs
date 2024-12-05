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
    let mut lines = input.trim().lines();

    let mut map: [[bool; 100]; 100] = [[false; 100]; 100];

    let mut sum = 0;
    for line in &mut lines {
        if line.is_empty() {
            break;
        }
        let [lhs, rhs] = [&line[..2], &line[3..]].map(|s| s.parse::<u8>().unwrap());
        map[lhs as usize][rhs as usize] = true;
    }
    for update in lines {
        let mut update = update.split(',').map(|s| s.parse::<u8>().expect(s)).collect::<Vec<_>>();
        let old_update = update.clone();
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
