// TODO: Rewrite all this absolute crap.
use std::{
    collections::HashSet,
    hint::unreachable_unchecked,
    simd::{cmp::SimdPartialEq, u8x32},
};

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

// #[test]
// fn test_part1_example() {
//     assert_eq!(part1(include_str!("../input/day6_part1_example")), 41);
// }

#[test]
fn test_part1_input() {
    assert_eq!(part1(include_str!("../input/day6.txt")), 5453);
}

#[expect(clippy::cast_possible_wrap, clippy::missing_panics_doc, clippy::cast_sign_loss)]
pub fn part2(input: &str) -> u32 {
    let input = input.as_bytes();
    let initial_guard_position = input.find_byte(b'^').unwrap();

    let mut squares_traversed = HashSet::new();

    {
        let mut guard_position = initial_guard_position;
        let mut direction = UP;

        loop {
            let forward_position = guard_position as isize + direction;
            if forward_position < 0 || forward_position > input.len() as isize {
                break;
            }
            let forward = input[forward_position as usize];
            if forward == b'#' {
                direction = match direction {
                    UP => RIGHT,
                    RIGHT => DOWN,
                    DOWN => LEFT,
                    LEFT => UP,
                    _ => unsafe { unreachable_unchecked() },
                };
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
        let mut guard_position = initial_guard_position;
        let mut history = [0; 131 * 130];
        if unsafe { go_up(input, object_square, &mut history, &mut guard_position) } {
            loop_obstacles += 1;
        }
    }

    loop_obstacles
}

// #[test]
// #[ignore]
// fn test_part2_example() {
//     assert_eq!(part2(include_str!("../input/day6_part1_example")), 6);
// }

#[test]
fn test_part2_input() {
    assert_eq!(part2(include_str!("../input/day6.txt")), 2188);
}

const UP: isize = -131;
const RIGHT: isize = 1;
const DOWN: isize = 131;
const LEFT: isize = -1;

unsafe fn go_up(
    input: &[u8],
    object_square: usize,
    history: &mut [u8; 130 * 131],
    guard_position: &mut usize,
) -> bool {
    loop {
        if *guard_position < 131 {
            return false;
        }
        let next = *guard_position - 131;
        if *input.get_unchecked(next) == b'#' || next == object_square {
            if *history.get_unchecked(next) & 1 > 0 {
                return true;
            }
            *history.get_unchecked_mut(next) |= 1;
            break;
        }
        *guard_position = next;
    }
    go_right(input, object_square, history, guard_position)
}
unsafe fn go_right(
    input: &[u8],
    object_square: usize,
    history: &mut [u8; 130 * 131],
    guard_position: &mut usize,
) -> bool {
    if *guard_position > (129 * 131) {
        return false;
    }
    let same_row = object_square / 131 == *guard_position / 131;
    loop {
        if *input.get_unchecked(*guard_position + 1) == b'#' {
            break;
        }
        let block = u8x32::from_array(
            input.get_unchecked(*guard_position..*guard_position + 32).try_into().unwrap_unchecked(),
        );
        let next_newline = 131 - (*guard_position % 131);
        let next_obstacle = block.simd_eq(u8x32::splat(b'#')).to_bitmask().trailing_zeros();
        let next_obstacle = if next_obstacle == 64 { 32 } else { next_obstacle };

        let mut next_obstacle = *guard_position + next_obstacle as usize;
        let next_newline = *guard_position + next_newline;

        let inbetween = object_square > *guard_position && object_square < next_obstacle;

        if same_row && inbetween {
            next_obstacle = object_square;
        }
        if next_newline < next_obstacle {
            return false;
        }
        *guard_position = next_obstacle - 1;

        if same_row && inbetween {
            break;
        }
    }
    if history.get_unchecked(*guard_position) & 2 > 0 {
        return true;
    }
    *history.get_unchecked_mut(*guard_position) |= 2;
    go_down(input, object_square, history, guard_position)
}
unsafe fn go_down(
    input: &[u8],
    object_square: usize,
    history: &mut [u8; 130 * 131],
    guard_position: &mut usize,
) -> bool {
    std::hint::assert_unchecked(input.len() == (131 * 130));
    loop {
        if input.len() < *guard_position + 131 {
            return false;
        }
        let next = *guard_position + 131;
        if *input.get_unchecked(next) == b'#' || next == object_square {
            if *history.get_unchecked_mut(next) & 4 > 0 {
                return true;
            }
            *history.get_unchecked_mut(next) |= 4;
            break;
        }

        *guard_position = next;
    }
    go_left(input, object_square, history, guard_position)
}
unsafe fn go_left(
    input: &[u8],
    object_square: usize,
    history: &mut [u8; 130 * 131],
    guard_position: &mut usize,
) -> bool {
    if *guard_position < 131 {
        return false;
    }
    let start_line = *guard_position / 131 * 131;
    loop {
        if *guard_position == start_line {
            return false;
        }
        let next = *guard_position - 1;
        if *input.get_unchecked(next) == b'#' || next == object_square {
            if history.get_unchecked(next) & 8 > 0 {
                return true;
            }
            *history.get_unchecked_mut(next) |= 8;
            break;
        }

        *guard_position = next;
    }
    go_up(input, object_square, history, guard_position)
}
