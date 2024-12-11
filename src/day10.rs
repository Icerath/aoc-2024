pub fn part1(input: &str) -> u32 {
    let line_len = input.lines().next().unwrap().len() + 1;
    let input = input.as_bytes();
    let mut sum = 0;
    for (i, &b) in input.iter().enumerate() {
        if b != b'0' {
            continue;
        }
        let mut places_visted = [false; 46 * 45];
        sum += trail_count_from(input, line_len, i, 0, &mut places_visted);
    }
    sum
}

fn trail_count_from(
    input: &[u8],
    line_len: usize,
    position: usize,
    mut current: u8,
    places_visted: &mut [bool],
) -> u32 {
    if places_visted[position] || input[position].wrapping_sub(b'0') != current {
        return 0;
    }
    places_visted[position] = true;
    if current == 9 {
        return 1;
    }
    current += 1;
    let mut sum = 0;
    sum += trail_count_from(input, line_len, position + 1, current, places_visted);
    if position != 0 {
        sum += trail_count_from(input, line_len, position - 1, current, places_visted);
    }
    if position >= line_len {
        sum += trail_count_from(input, line_len, position - line_len, current, places_visted);
    }
    if position < (input.len() - line_len) {
        sum += trail_count_from(input, line_len, position + line_len, current, places_visted);
    }
    sum
}

#[test]
fn test_part1_example() {
    assert_eq!(part1(include_str!("../input/day10_part1_example")), 36);
}

#[test]
fn test_part1_input() {
    assert_eq!(part1(include_str!("../input/day10_part1")), 552);
}

pub fn part2(input: &str) -> u32 {
    let line_len = input.lines().next().unwrap().len() + 1;
    let input = input.as_bytes();
    let mut sum = 0;
    for (i, &b) in input.iter().enumerate() {
        if b != b'0' {
            continue;
        }
        sum += part2_trail_count_from(input, line_len, i, 0);
    }
    sum
}

fn part2_trail_count_from(input: &[u8], line_len: usize, position: usize, mut current: u8) -> u32 {
    if input[position].wrapping_sub(b'0') != current {
        return 0;
    }
    if current == 9 {
        return 1;
    }
    current += 1;
    let mut sum = 0;
    sum += part2_trail_count_from(input, line_len, position + 1, current);
    if position != 0 {
        sum += part2_trail_count_from(input, line_len, position - 1, current);
    }
    if position >= line_len {
        sum += part2_trail_count_from(input, line_len, position - line_len, current);
    }
    if position < (input.len() - line_len) {
        sum += part2_trail_count_from(input, line_len, position + line_len, current);
    }
    sum
}

#[test]
fn test_part2_example() {
    assert_eq!(part2(include_str!("../input/day10_part1_example")), 81);
}

#[test]
fn test_part2_input() {
    assert_eq!(part2(include_str!("../input/day10_part1")), 1225);
}
