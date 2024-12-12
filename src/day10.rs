#![expect(clippy::cast_possible_truncation)]
use std::simd::{cmp::SimdPartialEq, u8x32, u8x64, Simd};

pub fn part1(input: &str) -> u32 {
    unsafe { part1_inner(input.as_bytes()) }
}

unsafe fn part1_inner(input: &[u8]) -> u32 {
    let line_len = line_len(input);
    let mut remaining = input;
    let mut places_visited = vec![false; input.len()];
    let mut sum = 0;
    macro_rules! loop_body {
        ($block: expr) => {
            let offset = (input.len() - remaining.len()) as u16;
            let mut zeros = $block.simd_eq(Simd::splat(b'0')).to_bitmask();
            while zeros != 0 {
                let i = zeros.trailing_zeros() as u16 + offset;
                zeros &= zeros - 1;
                sum += trail(input, i, &mut places_visited, line_len);
                places_visited.fill(false);
            }
        };
    }
    while remaining.len() >= 32 {
        let block = u8x32::from_array(remaining.get_unchecked(..32).try_into().unwrap_unchecked());
        loop_body!(block);
        remaining = remaining.get_unchecked(32..);
    }
    loop_body!(u8x32::load_or_default(remaining));
    sum
}

unsafe fn trail(input: &[u8], initial_position: u16, places_visited: &mut [bool], line_len: usize) -> u32 {
    let mut stack_positions = [0u16; 256];
    let mut stack_digits = [0u8; 256];
    let mut stack_len = 1;
    stack_positions[0] = initial_position;
    stack_digits[0] = b'0';

    let mut sum = 0;
    while stack_len > 0 {
        stack_len -= 1;
        if *stack_digits.get_unchecked(stack_len) == b'9' {
            sum += 1;
            continue;
        }
        let digit = *stack_digits.get_unchecked(stack_len) + 1;
        macro_rules! push {
            ($position: expr) => {
                let position = $position;
                if *input.get_unchecked(position) == digit && !*places_visited.get_unchecked(position) {
                    *places_visited.get_unchecked_mut(position) = true;
                    *stack_positions.get_unchecked_mut(stack_len) = position as u16;
                    *stack_digits.get_unchecked_mut(stack_len) = digit;
                    stack_len += 1;
                }
            };
        }
        let position = *stack_positions.get_unchecked(stack_len) as usize;
        push!(position + 1);
        if position != 0 {
            push!(position - 1);
        }
        if position >= line_len {
            push!(position - line_len);
        }
        if position < (input.len() - line_len) {
            push!(position + line_len);
        }
    }
    sum
}

// #[test]
// #[ignore]
// fn test_part1_example() {
//     assert_eq!(part1(include_str!("../input/day10_part1_example")), 36);
// }

#[test]
fn test_part1_input() {
    assert_eq!(part1(include_str!("../input/day10.txt")), 552);
}

pub fn part2(input: &str) -> u32 {
    unsafe { part2_inner(input.as_bytes()) }
}

unsafe fn part2_inner(input: &[u8]) -> u32 {
    let line_len = line_len(input);
    let mut arrays = [[u8x64::splat(0); 64]; 9];

    for i in 0..line_len - 2 {
        let line = u8x64::from_array(
            input.get_unchecked(i * line_len..i * line_len + 64).try_into().unwrap_unchecked(),
        );
        let matches = simd_eq(line, b'9');
        *arrays[8].get_unchecked_mut(i) = matches;
    }
    {
        let i = line_len - 2;
        let line = u8x64::load_or_default(input.get_unchecked(i * line_len..));
        let matches = simd_eq(line, b'9');
        *arrays[8].get_unchecked_mut(i) = matches;
    }
    macro_rules! impl_digit {
        ($($digit: literal,)+) => {
            $(impl_digit!($digit);)+
        };
        ($digit: literal) => {{
            let i = 0;
            let line = u8x64::from_array(input.get_unchecked(0..64).try_into().unwrap_unchecked());
            let matches = simd_eq(line, $digit + b'0');
            let left_neighbors = arrays[$digit].get_unchecked(i).rotate_elements_left::<1>();
            let right_neighbors = arrays[$digit].get_unchecked(i).rotate_elements_right::<1>();
            let down_neighbors = *arrays[$digit].get_unchecked(i + 1);
            *arrays[$digit - 1].get_unchecked_mut(i) =
                (left_neighbors + right_neighbors + down_neighbors) * matches;
        }
        for i in 1..line_len - 2 {
            let line = u8x64::from_array(
                input.get_unchecked(i * line_len..i * line_len + 64).try_into().unwrap_unchecked(),
            );
            let matches = simd_eq(line, $digit + b'0');
            let left_neighbors = arrays[$digit].get_unchecked(i).rotate_elements_left::<1>();
            let right_neighbors = arrays[$digit].get_unchecked(i).rotate_elements_right::<1>();
            let up_neighbors = *arrays[$digit].get_unchecked(i - 1);
            let down_neighbors = *arrays[$digit].get_unchecked(i + 1);
            *arrays[$digit - 1].get_unchecked_mut(i) =
                (left_neighbors + right_neighbors + up_neighbors + down_neighbors) * matches;
        }
        {
            let i = line_len - 2;
            let line = u8x64::load_or_default(input.get_unchecked(i * line_len..));
            let matches = simd_eq(line, $digit + b'0');
            let left_neighbors = arrays[$digit].get_unchecked(i).rotate_elements_left::<1>();
            let right_neighbors = arrays[$digit].get_unchecked(i).rotate_elements_right::<1>();
            let up_neighbors = *arrays[$digit].get_unchecked(i - 1);
            *arrays[$digit - 1].get_unchecked_mut(i) =
                (left_neighbors + right_neighbors + up_neighbors) * matches;
        }};
    }

    impl_digit!(8, 7, 6, 5, 4, 3, 2, 1,);

    let mut sum = 0;
    {
        let i = 0;
        let line = u8x64::from_array(input.get_unchecked(0..64).try_into().unwrap_unchecked());
        let matches = simd_eq(line, b'0');
        let left_neighbors = arrays[0].get_unchecked(i).rotate_elements_left::<1>();
        let right_neighbors = arrays[0].get_unchecked(i).rotate_elements_right::<1>();
        let down_neighbors = *arrays[0].get_unchecked(i + 1);
        let total = (left_neighbors + right_neighbors + down_neighbors) * matches;
        sum += total.as_array().get_unchecked(..line_len - 1).iter().map(|&x| x as u32).sum::<u32>();
    }
    for i in 1..line_len - 2 {
        let line = u8x64::from_array(
            input.get_unchecked(i * line_len..i * line_len + 64).try_into().unwrap_unchecked(),
        );
        let matches = simd_eq(line, b'0');
        let left_neighbors = arrays[0].get_unchecked(i).rotate_elements_left::<1>();
        let right_neighbors = arrays[0].get_unchecked(i).rotate_elements_right::<1>();
        let up_neighbors = arrays[0].get_unchecked(i - 1);
        let down_neighbors = arrays[0].get_unchecked(i + 1);
        let total = (left_neighbors + right_neighbors + up_neighbors + down_neighbors) * matches;
        sum += total.as_array().get_unchecked(..line_len - 1).iter().map(|&x| x as u32).sum::<u32>();
    }
    {
        let i = line_len - 2;
        let line = u8x64::load_or_default(input.get_unchecked(i * line_len..));
        let matches = simd_eq(line, b'0');
        let left_neighbors = arrays[0].get_unchecked(i).rotate_elements_left::<1>();
        let right_neighbors = arrays[0].get_unchecked(i).rotate_elements_right::<1>();
        let up_neighbors = *arrays[0].get_unchecked(i - 1);
        let total = (left_neighbors + right_neighbors + up_neighbors) * matches;
        sum += total.as_array().get_unchecked(..line_len - 1).iter().map(|&x| x as u32).sum::<u32>();
    }
    sum
}

// #[test]
// #[ignore]
// fn test_part2_example() {
//     assert_eq!(part2(include_str!("../input/day10_part1_example")), 81);
// }

#[test]
fn test_part2_input() {
    assert_eq!(part2(include_str!("../input/day10.txt")), 1225);
}

#[inline(always)]
unsafe fn line_len(input: &[u8]) -> usize {
    1 + u8x64::from_array(input.get_unchecked(..64).try_into().unwrap_unchecked())
        .simd_eq(u8x64::splat(b'\n'))
        .to_bitmask()
        .trailing_zeros() as usize
}

#[inline(always)]
fn simd_eq(lhs: u8x64, val: u8) -> u8x64 {
    let mut vec = lhs.simd_eq(Simd::splat(val)).select(Simd::splat(1u8), Simd::splat(0));
    vec[63] = 0;
    vec
}
