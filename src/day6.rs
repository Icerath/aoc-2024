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

#[expect(clippy::cast_possible_wrap, clippy::missing_panics_doc, clippy::cast_sign_loss)]
pub fn part2(input: &str) -> u32 {
    let input = input.as_bytes();
    let line_width = input.find_byte(b'\n').unwrap() + 1;
    let initial_guard_position = input.find_byte(b'^').unwrap();

    let mut squares_traversed = HashSet::new();

    let down = line_width as isize;
    let left = -1;
    let up = -down;
    let right = -left;

    {
        let mut guard_position = initial_guard_position;
        let mut direction = up;

        loop {
            let forward_position = guard_position as isize + direction;
            if forward_position < 0 || forward_position > input.len() as isize {
                break;
            }
            let forward = input[forward_position as usize];
            if forward == b'#' {
                if direction == up {
                    direction = right;
                } else if direction == right {
                    direction = down;
                } else if direction == down {
                    direction = left;
                } else {
                    direction = up;
                }
            } else if forward == b'\n' {
                break;
            } else {
                guard_position = guard_position.wrapping_add_signed(direction);
                squares_traversed.insert(guard_position);
            }
        }
    }

    let mut loop_obstacles = 0;
    for object_square in squares_traversed {
        if object_square == initial_guard_position {
            continue;
        }
        let mut history = [[false; 4]; 131 * 130];

        let mut guard_position = initial_guard_position;
        let mut direction = up;

        loop {
            let forward_position = guard_position as isize + direction;

            if forward_position < 0 || forward_position > input.len() as isize {
                break;
            }
            let forward_position = forward_position as usize;
            let forward = input[forward_position];

            if forward == b'#' || forward_position == object_square {
                if direction == up {
                    direction = right;
                } else if direction == right {
                    direction = down;
                } else if direction == down {
                    direction = left;
                } else {
                    direction = up;
                }
            } else if forward == b'\n' {
                break;
            } else {
                let direction_int = if direction == up {
                    1
                } else if direction == right {
                    2
                } else if direction == down {
                    3
                } else {
                    4
                } - 1;
                if history[guard_position][direction_int] {
                    loop_obstacles += 1;
                    break;
                }

                history[guard_position][direction_int] = true;
                guard_position = forward_position;
            }
        }
    }

    loop_obstacles
}

#[test]
fn test_part2_example() {
    assert_eq!(part2(include_str!("../input/day6_part1_example")), 6);
}

#[test]
fn test_part2_input() {
    assert_eq!(part2(include_str!("../input/day6_part1")), 2188);
}
