pub fn part1(input: &str) -> u32 {
    let input = input.as_bytes();
    let mut sum = 0;
    let mut remaining = input;
    while !remaining.is_empty() {
        if !remaining.starts_with(b"mul(") {
            remaining = &remaining[1..];
            continue;
        }
        remaining = &remaining[4..];
        let Some(lhs) = parse_num(&mut remaining) else { continue };
        if remaining[0] != b',' {
            continue;
        }
        remaining = &remaining[1..];
        let Some(rhs) = parse_num(&mut remaining) else { continue };
        if remaining[0] != b')' {
            continue;
        }
        remaining = &remaining[1..];
        sum += lhs * rhs;
    }
    sum
}

fn parse_num(input: &mut &[u8]) -> Option<u32> {
    let mut num = 0u32;
    for i in 0..3 {
        match *input.first()? {
            n @ b'0'..=b'9' => num = (num * 10) + (n - b'0') as u32,
            _ => return (i != 0).then_some(num),
        }
        *input = &input[1..];
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
