use std::collections::HashSet;

use bstr::ByteSlice;

#[expect(clippy::cast_possible_wrap, clippy::missing_panics_doc, clippy::cast_sign_loss)]
pub fn part1(input: &str) -> u32 {
    let input = input.as_bytes();

    let line_width = input.find_byte(b'\n').unwrap() + 1;
    let mut guard_position = input.find_byte(b'^').unwrap();
    let mut direction = -(line_width as isize);

    let mut sum_positions = 1;
    let mut squares_traversed = HashSet::new();
    loop {
        let forward_position = guard_position as isize + direction;
        if forward_position < 0 || forward_position > input.len() as isize {
            break;
        }
        let forward = input[forward_position as usize];
        if forward == b'#' {
            if direction == -(line_width as isize) {
                direction = 1;
            } else if direction == (line_width as isize) {
                direction = -1;
            } else if direction == -1 {
                direction = -(line_width as isize);
            } else {
                direction = line_width as isize;
            }
        } else if forward == b'\n' {
            break;
        } else {
            if squares_traversed.insert(guard_position) {
                sum_positions += 1;
            }
            guard_position = guard_position.wrapping_add_signed(direction);
        }
    }
    sum_positions
}

#[test]
fn test_part1_example() {
    assert_eq!(part1(include_str!("../input/day6_part1_example")), 41);
}

#[test]
fn test_part1_input() {
    assert_eq!(part1(include_str!("../input/day6_part1")), 5453);
}
