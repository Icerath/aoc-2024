use std::simd::{cmp::SimdPartialEq, u8x32, u8x64, Simd};

const INPUT_SIZE: usize = 45 * 46;

pub fn part1(input: &str) -> u32 {
    unsafe { part1_inner(input.as_bytes()) }
}
unsafe fn part1_inner(input: &[u8]) -> u32 {
    let mut remaining = input;
    let mut sum = 0;

    let mut places_visted = [false; 46 * 45];

    while remaining.len() >= 32 {
        let offset = INPUT_SIZE - remaining.len();
        let block = u8x32::from_array(remaining[..32].try_into().unwrap_unchecked());

        let mut zeros = block.simd_eq(Simd::splat(b'0')).to_bitmask();

        while zeros != 0 {
            let i = zeros.trailing_zeros() as usize + offset;
            zeros &= zeros - 1;

            sum += trail_count_from(input, i, 0, &mut places_visted);
            places_visted.fill(false);
        }
        remaining = &remaining[32..];
    }
    let offset = INPUT_SIZE - remaining.len();
    for (i, &b) in remaining.iter().enumerate() {
        let i = offset + i;
        let b'0' = b else { continue };
        sum += trail_count_from(input, i, 0, &mut places_visted);
        places_visted.fill(false);
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
    let mut arrays = [[u8x64::splat(0); 45]; 9];

    for i in 0..44 {
        let line = u8x64::from_array(input[i * 46..i * 46 + 64].try_into().unwrap_unchecked());
        let matches = simd_eq(line, b'9');
        arrays[8][i] = matches;
    }
    {
        let i = 44;
        let line = u8x64::load_or_default(&input[i * 46..]);
        let matches = simd_eq(line, b'9');
        arrays[8][i] = matches;
    }
    macro_rules! impl_digit {
        ($digit: literal) => {{
            let i = 0;
            let line = u8x64::from_array(input[0..64].try_into().unwrap_unchecked());
            let matches = simd_eq(line, $digit + b'0');
            let left_neighbors = arrays[$digit][i].rotate_elements_left::<1>();
            let right_neighbors = arrays[$digit][i].rotate_elements_right::<1>();
            let down_neighbors = arrays[$digit][i + 1];
            arrays[$digit - 1][i] = (left_neighbors + right_neighbors + down_neighbors) * matches;
        }
        for i in 1..44 {
            let line = u8x64::from_array(input[i * 46..i * 46 + 64].try_into().unwrap_unchecked());
            let matches = simd_eq(line, $digit + b'0');
            let left_neighbors = arrays[$digit][i].rotate_elements_left::<1>();
            let right_neighbors = arrays[$digit][i].rotate_elements_right::<1>();
            let up_neighbors = arrays[$digit][i - 1];
            let down_neighbors = arrays[$digit][i + 1];
            arrays[$digit - 1][i] =
                (left_neighbors + right_neighbors + up_neighbors + down_neighbors) * matches;
        }
        {
            let i = 44;
            let line = u8x64::load_or_default(&input[i * 46..]);
            let matches = simd_eq(line, $digit + b'0');
            let left_neighbors = arrays[$digit][i].rotate_elements_left::<1>();
            let right_neighbors = arrays[$digit][i].rotate_elements_right::<1>();
            let up_neighbors = arrays[$digit][i - 1];
            arrays[$digit - 1][i] = (left_neighbors + right_neighbors + up_neighbors) * matches;
        }};
    }

    impl_digit!(8);
    impl_digit!(7);
    impl_digit!(6);
    impl_digit!(5);
    impl_digit!(4);
    impl_digit!(3);
    impl_digit!(2);
    impl_digit!(1);

    let mut sum = 0;
    {
        let i = 0;
        let line = u8x64::from_array(input[0..64].try_into().unwrap_unchecked());
        let matches = simd_eq(line, b'0');
        let left_neighbors = arrays[0][i].rotate_elements_left::<1>();
        let right_neighbors = arrays[0][i].rotate_elements_right::<1>();
        let down_neighbors = arrays[0][i + 1];
        let total = (left_neighbors + right_neighbors + down_neighbors) * matches;
        sum += total[..45].iter().map(|&x| x as u32).sum::<u32>();
    }
    for i in 1..44 {
        let line = u8x64::from_array(input[i * 46..i * 46 + 64].try_into().unwrap_unchecked());
        let matches = simd_eq(line, b'0');
        let left_neighbors = arrays[0][i].rotate_elements_left::<1>();
        let right_neighbors = arrays[0][i].rotate_elements_right::<1>();
        let up_neighbors = arrays[0][i - 1];
        let down_neighbors = arrays[0][i + 1];
        let total = (left_neighbors + right_neighbors + up_neighbors + down_neighbors) * matches;
        sum += total[..45].iter().map(|&x| x as u32).sum::<u32>();
    }
    {
        let i = 44;
        let line = u8x64::load_or_default(&input[i * 46..]);
        let matches = simd_eq(line, b'0');
        let left_neighbors = arrays[0][i].rotate_elements_left::<1>();
        let right_neighbors = arrays[0][i].rotate_elements_right::<1>();
        let up_neighbors = arrays[0][i - 1];
        let total = (left_neighbors + right_neighbors + up_neighbors) * matches;
        sum += total[..45].iter().map(|&x| x as u32).sum::<u32>();
    }
    sum
}

#[inline(always)]
fn simd_eq(lhs: u8x64, val: u8) -> u8x64 {
    let mut vec = lhs.simd_eq(Simd::splat(val)).select(Simd::splat(1u8), Simd::splat(0));
    vec[63] = 0;
    vec
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
