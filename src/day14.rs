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

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../input/day14.txt")), 220_971_520);
}
