#![expect(clippy::cast_sign_loss)]
use bstr::ByteSlice;

pub fn part1(input: &str) -> u32 {
    let mut remaining = input.as_bytes();
    let mut quadrants = [0u32; 4];
    loop {
        if remaining.is_empty() {
            break;
        }
        remaining = &remaining[2..];
        let comma = remaining.find_byte(b',').unwrap();
        let px: i32 = remaining[..comma].to_str().unwrap().parse().unwrap();
        remaining = &remaining[comma + 1..];
        let space = remaining.find_byte(b' ').unwrap();
        let py: i32 = remaining[..space].to_str().unwrap().parse().unwrap();
        remaining = &remaining[space + 3..];
        let comma = remaining.find_byte(b',').unwrap();
        let vx: i32 = remaining[..comma].to_str().unwrap().parse().unwrap();
        remaining = &remaining[comma + 1..];
        let nl = remaining.find_byte(b'\n').unwrap();
        let vy: i32 = remaining[..nl].to_str().unwrap().parse().unwrap();
        remaining = &remaining[nl + 1..];

        let px = (px + vx * 100).rem_euclid(101);
        let py = (py + vy * 100).rem_euclid(103);

        #[expect(non_contiguous_range_endpoints)]
        match (px, py) {
            (..50, ..51) => quadrants[0] += 1,
            (..50, 52..) => quadrants[1] += 1,
            (51.., ..51) => quadrants[2] += 1,
            (51.., 52..) => quadrants[3] += 1,
            _ => {}
        }
    }
    quadrants.into_iter().product()
}

pub fn part2(input: &str) -> i32 {
    let mut remaining = input.as_bytes();
    let mut robots = vec![];
    loop {
        if remaining.is_empty() {
            break;
        }
        remaining = &remaining[2..];
        let comma = remaining.find_byte(b',').unwrap();
        let px: i32 = remaining[..comma].to_str().unwrap().parse().unwrap();
        remaining = &remaining[comma + 1..];
        let space = remaining.find_byte(b' ').unwrap();
        let py: i32 = remaining[..space].to_str().unwrap().parse().unwrap();
        remaining = &remaining[space + 3..];
        let comma = remaining.find_byte(b',').unwrap();
        let vx: i32 = remaining[..comma].to_str().unwrap().parse().unwrap();
        remaining = &remaining[comma + 1..];
        let nl = remaining.find_byte(b'\n').unwrap();
        let vy: i32 = remaining[..nl].to_str().unwrap().parse().unwrap();
        remaining = &remaining[nl + 1..];

        robots.push([px, py, vx, vy]);
    }
    for i in 0..(101 * 103) {
        // FIXME: u128s are slow. Use something else.
        let mut grid = [0u128; 103];
        for [px, py, vx, vy] in &mut robots {
            grid[*py as usize] |= 1u128 << *px;
            *px = (*px + *vx).rem_euclid(101);
            *py = (*py + *vy).rem_euclid(103);
        }
        // FIXME: Currently this looks for 16 consecutive robots on a 16 alignment. This works for 2 inputs I've tested but I'm not satisfied.
        if grid.iter().any(|row| (0..101).step_by(0xF).map(|x| 0xFFFF << x).any(|mask| row & mask == mask)) {
            return i;
        }
    }
    unreachable!()
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../input/day14.txt")), 220_971_520);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("../input/day14.txt")), 6355);
}
