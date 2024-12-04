use bstr::ByteSlice;

pub fn part1(input: &str) -> u32 {
    let input = input.as_bytes();
    let mut sum = 0;
    let mut remaining = input;
    while remaining.len() >= 8 {
        unsafe {
            if !remaining.starts_with(b"mul(") {
                remaining = remaining.get_unchecked(1..);
                continue;
            }
            remaining = remaining.get_unchecked(4..);
            std::hint::assert_unchecked(remaining.len() >= 4);
            let Some(lhs) = parse_num(&mut remaining) else { continue };
            if *remaining.get_unchecked(0) != b',' {
                continue;
            }
            remaining = remaining.get_unchecked(1..);
            let Some(rhs) = parse_num(&mut remaining) else { continue };
            if *remaining.get_unchecked(0) != b')' {
                continue;
            }
            remaining = remaining.get_unchecked(1..);
            sum += lhs * rhs;
        }
    }
    sum
}

#[inline(always)]
fn parse_num(input: &mut &[u8]) -> Option<u32> {
    let mut num = 0u32;
    for i in 0..3 {
        match *input.first()? {
            n @ b'0'..=b'9' => num = (num * 10) + (n - b'0') as u32,
            _ => return (i != 0).then_some(num),
        }
        *input = unsafe { input.get_unchecked(1..) };
    }
    Some(num)
}

#[test]
fn test_part1_example() {
    let input = include_str!("../input/day3_part1_example");
    assert_eq!(part1(input), 161);
}

#[test]
fn test_part1_input() {
    let input = include_str!("../input/day3_part1");
    assert_eq!(part1(input), 171_183_089);
}

pub fn part2(input: &str) -> u32 {
    let input = input.as_bytes();
    let mut sum = 0;
    let mut remaining = input;
    while remaining.len() >= 8 {
        unsafe {
            if remaining.starts_with(b"do()") {
                remaining = &remaining[4..];
            } else if remaining.starts_with(b"don't()") {
                remaining = remaining.get_unchecked(7..);
                let Some(skip) = remaining.find(b"do()") else { break };
                remaining = remaining.get_unchecked(skip..);
            }

            if !remaining.starts_with(b"mul(") {
                remaining = remaining.get_unchecked(1..);
                continue;
            }
            remaining = remaining.get_unchecked(4..);
            let Some(lhs) = parse_num(&mut remaining) else { continue };
            if remaining[0] != b',' {
                continue;
            }
            remaining = remaining.get_unchecked(1..);
            let Some(rhs) = parse_num(&mut remaining) else { continue };
            if *remaining.get_unchecked(0) != b')' {
                continue;
            }
            remaining = remaining.get_unchecked(1..);
            sum += lhs * rhs;
        }
    }
    sum
}

#[test]
fn test_part2_example() {
    let example = include_str!("../input/day3_part2_example");
    assert_eq!(part2(example), 48);
}

#[test]
fn test_part2_input() {
    let example = include_str!("../input/day3_part1");
    assert_eq!(part2(example), 63_866_497);
}
