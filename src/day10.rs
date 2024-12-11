use std::simd::{cmp::SimdPartialEq, u8x64, Simd};

pub fn part1(input: &str) -> u32 {
    let input = input.as_bytes();
    let mut sum = 0;
    for (i, &b) in input.iter().enumerate() {
        if b != b'0' {
            continue;
        }
        let mut places_visted = [false; 46 * 45];
        sum += trail_count_from(input, i, 0, &mut places_visted);
    }
    sum
}

fn trail_count_from(input: &[u8], position: usize, mut current: u8, places_visted: &mut [bool]) -> u32 {
    if places_visted[position] || input[position].wrapping_sub(b'0') != current {
        return 0;
    }
    places_visted[position] = true;
    if current == 9 {
        return 1;
    }
    current += 1;
    let mut sum = 0;
    sum += trail_count_from(input, position + 1, current, places_visted);
    if position != 0 {
        sum += trail_count_from(input, position - 1, current, places_visted);
    }
    if position >= LINE_LEN {
        sum += trail_count_from(input, position - LINE_LEN, current, places_visted);
    }
    if position < (input.len() - LINE_LEN) {
        sum += trail_count_from(input, position + LINE_LEN, current, places_visted);
    }
    sum
}

#[test]
#[ignore]
fn test_part1_example() {
    assert_eq!(part1(include_str!("../input/day10_part1_example")), 36);
}

#[test]
fn test_part1_input() {
    assert_eq!(part1(include_str!("../input/day10_part1")), 552);
}

pub fn part2(input: &str) -> u32 {
    unsafe { part2_inner(input.as_bytes()) }
}

unsafe fn part2_inner(input: &[u8]) -> u32 {
    let mut arrays = [[u8x64::splat(0); 45]; 10];

    for i in 0..44 {
        let line = u8x64::from_array(input[i * 46..i * 46 + 64].try_into().unwrap_unchecked());
        let mut matches = simd_eq(line, b'9');
        matches[63] = 0;
        arrays[9][i] = matches;
    }
    for i in 44..45 {
        let line = u8x64::load_or_default(&input[i * 46..]);
        let matches = simd_eq(line, b'9');
        arrays[9][i] = matches;
    }
    macro_rules! impl_digit {
        ($digit: literal) => {
            for i in 0..1 {
                let line = u8x64::from_array(input[0..64].try_into().unwrap_unchecked());
                let mut matches = simd_eq(line, $digit + b'0');
                matches[63] = 0;
                let left_neighbors = arrays[$digit + 1][i].rotate_elements_left::<1>();
                let right_neighbors = arrays[$digit + 1][i].rotate_elements_right::<1>();
                let down_neighbors = arrays[$digit + 1][i + 1];
                arrays[$digit][i] = (left_neighbors + right_neighbors + down_neighbors) * matches;
            }
            for i in 1..44 {
                let line = u8x64::from_array(input[i * 46..i * 46 + 64].try_into().unwrap_unchecked());
                let mut matches = simd_eq(line, $digit + b'0');
                matches[63] = 0;
                let left_neighbors = arrays[$digit + 1][i].rotate_elements_left::<1>();
                let right_neighbors = arrays[$digit + 1][i].rotate_elements_right::<1>();
                let up_neighbors = arrays[$digit + 1][i - 1];
                let down_neighbors = arrays[$digit + 1][i + 1];
                arrays[$digit][i] =
                    (left_neighbors + right_neighbors + up_neighbors + down_neighbors) * matches;
            }
            for i in 44..45 {
                let line = u8x64::load_or_default(&input[i * 46..]);
                let matches = simd_eq(line, $digit + b'0');
                let left_neighbors = arrays[$digit + 1][i].rotate_elements_left::<1>();
                let right_neighbors = arrays[$digit + 1][i].rotate_elements_right::<1>();
                let up_neighbors = arrays[$digit + 1][i - 1];
                arrays[$digit][i] = (left_neighbors + right_neighbors + up_neighbors) * matches;
            }
        };
    }

    impl_digit!(8);
    impl_digit!(7);
    impl_digit!(6);
    impl_digit!(5);
    impl_digit!(4);
    impl_digit!(3);
    impl_digit!(2);
    impl_digit!(1);
    impl_digit!(0);

    arrays[0].iter().map(|&x| x.to_array()[..45].iter().map(|&x| x as u32).sum::<u32>()).sum()
}

fn simd_eq(lhs: u8x64, val: u8) -> u8x64 {
    lhs.simd_eq(Simd::splat(val)).select(Simd::splat(1u8), Simd::splat(0))
}

#[test]
#[ignore]
fn test_part2_example() {
    assert_eq!(part2(include_str!("../input/day10_part1_example")), 81);
}

#[test]
fn test_part2_input() {
    assert_eq!(part2(include_str!("../input/day10_part1")), 1225);
}

const LINE_LEN: usize = 46;