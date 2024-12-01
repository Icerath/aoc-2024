use std::collections::HashMap;

pub const EXAMPLE_1: &str = "
3   4
4   3
2   5
1   3
3   9
3   3";

pub fn part1(input: &str) -> u32 {
    let mut lhs_list = vec![];
    let mut rhs_list = vec![];
    for line in input.trim().lines() {
        let (lhs, rhs) = line.split_once("   ").unwrap();
        lhs_list.push(lhs.parse::<i32>().unwrap());
        rhs_list.push(rhs.parse::<i32>().unwrap());
    }
    lhs_list.sort_unstable();
    rhs_list.sort_unstable();

    let mut total_diff = 0;
    for (lhs, rhs) in lhs_list.into_iter().zip(rhs_list) {
        total_diff += lhs.abs_diff(rhs)
    }
    total_diff
}

#[test]
fn part1_example() {
    assert_eq!(part1(EXAMPLE_1), 11);
}

#[test]
fn part1_input() {
    let input = std::fs::read_to_string("input/day1_part1").unwrap();
    assert_eq!(part1(&input), 2086478);
}

pub fn part2(input: &str) -> i32 {
    let mut lhs_list = vec![];
    let mut rhs_counts = HashMap::<i32, i32>::new();
    for line in input.trim().lines() {
        let (lhs, rhs) = line.split_once("   ").unwrap();
        lhs_list.push(lhs.parse::<i32>().unwrap());
        *rhs_counts.entry(rhs.parse().unwrap()).or_default() += 1;
    }

    let mut score = 0;
    for val in lhs_list {
        score += val * rhs_counts.get(&val).unwrap_or(&0);
    }
    score
}

#[test]
fn part2_example() {
    assert_eq!(part2(EXAMPLE_1), 31);
}

#[test]
fn part2_input() {
    let input = std::fs::read_to_string("input/day1_part1").unwrap();
    assert_eq!(part2(&input), 24941624);
}
