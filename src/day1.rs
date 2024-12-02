use std::collections::HashMap;

pub const EXAMPLE_1: &str = "
3   4
4   3
2   5
1   3
3   9
3   3";

pub fn part1(input: &str) -> u32 {
    let [mut lhs_list, mut rhs_list] = parse(input.trim());

    lhs_list.sort_unstable();
    rhs_list.sort_unstable();

    lhs_list
        .iter()
        .zip(rhs_list)
        .map(|(lhs, rhs)| lhs.abs_diff(rhs))
        .sum()
}

pub fn parse(input: &str) -> [Vec<u32>; 2] {
    let input = input.trim();
    let input_bytes = input.as_bytes();
    match input.len() {
        PRECISE_RN_PATH_LEN => part1_rn_precise_path(input_bytes.try_into().unwrap()),
        PRECISE_NL_PATH_LEN => part1_nl_precise_path(input_bytes.try_into().unwrap()),
        _ => part1_parse_generic(input),
    }
}

const PRECISE_RN_PATH_LEN: usize = 1000 * 15 - 2;
fn part1_rn_precise_path(input: &[u8; PRECISE_RN_PATH_LEN]) -> [Vec<u32>; 2] {
    let mut lhs_list = Vec::with_capacity(1000);
    let mut rhs_list = Vec::with_capacity(1000);

    for i in 0..1000 {
        let i = i * 15;
        let lhs = parse_int5(input[i..i + 5].try_into().unwrap());
        let rhs = parse_int5(input[i + 8..i + 13].try_into().unwrap());
        lhs_list.push(lhs);
        rhs_list.push(rhs);
    }
    [lhs_list, rhs_list]
}

const PRECISE_NL_PATH_LEN: usize = 1000 * 14 - 1;
fn part1_nl_precise_path(input: &[u8; PRECISE_NL_PATH_LEN]) -> [Vec<u32>; 2] {
    let mut lhs_list = Vec::with_capacity(1000);
    let mut rhs_list = Vec::with_capacity(1000);

    for i in 0..1000 {
        let i = i * 14;
        let lhs = parse_int5(input[i..i + 5].try_into().unwrap());
        let rhs = parse_int5(input[i + 8..i + 13].try_into().unwrap());
        lhs_list.push(lhs);
        rhs_list.push(rhs);
    }
    [lhs_list, rhs_list]
}

fn part1_parse_generic(input: &str) -> [Vec<u32>; 2] {
    let mut lhs_list = Vec::with_capacity(1000);
    let mut rhs_list = Vec::with_capacity(1000);

    for line in input.lines() {
        let [lhs, rhs] = parse_line(line);
        lhs_list.push(lhs);
        rhs_list.push(rhs);
    }
    [lhs_list, rhs_list]
}

pub fn parse_line(line: &str) -> [u32; 2] {
    match line.len() {
        13 => {
            let lhs = parse_int5(line.as_bytes()[0..5].try_into().unwrap());
            let rhs = parse_int5(line.as_bytes()[8..13].try_into().unwrap());
            [lhs, rhs]
        }
        5 => [
            (line.as_bytes()[0] - b'0') as u32,
            (line.as_bytes()[4] - b'0') as u32,
        ],
        _ => {
            let (lhs, rhs) = line.split_once("   ").unwrap();
            [lhs, rhs].map(|num| num.parse().unwrap())
        }
    }
}

fn parse_int5(bytes: &[u8; 5]) -> u32 {
    let zero = b'0' as u32;
    let offset = zero * 10000 + zero * 1000 + zero * 100 + zero * 10 + zero;

    (bytes[0] as u32 * 10000
        + (bytes[1]) as u32 * 1000
        + (bytes[2]) as u32 * 100
        + (bytes[3]) as u32 * 10
        + (bytes[4]) as u32)
        - offset
}

#[test]
fn part1_example() {
    assert_eq!(part1(EXAMPLE_1), 11);
}

#[test]
fn part1_input() {
    let input = include_str!("../input/day1_part1");
    assert_eq!(part1(input), 2086478);
}

pub fn part2(input: &str) -> u32 {
    let [lhs_list, rhs_list] = parse(input.trim());
    let mut rhs_counts = HashMap::<u32, u16>::with_capacity(1000);

    for val in rhs_list {
        *rhs_counts.entry(val).or_default() += 1;
    }

    let mut score = 0;
    for val in lhs_list {
        score += val * rhs_counts.get(&val).copied().unwrap_or(0) as u32;
    }
    score
}

#[test]
fn part2_example() {
    assert_eq!(part2(EXAMPLE_1), 31);
}

#[test]
fn part2_input() {
    let input = include_str!("../input/day1_part1");
    assert_eq!(part2(input), 24941624);
}
