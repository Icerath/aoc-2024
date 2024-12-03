use tinyvec::ArrayVec;

pub fn part1(input: &str) -> u32 {
    let input = input.as_bytes();
    input
        .split(|c| *c == b'\n')
        .map(|line| {
            let record = line.split(|c| *c == b' ').map(parse_int);
            (is_record_safe(record) == 0) as u32
        })
        .sum()
}

#[inline(always)]
fn parse_int(bytes: &[u8]) -> u8 {
    unsafe {
        std::hint::assert_unchecked(bytes.len() <= 2);
    }
    match bytes.len() {
        1 => bytes[0] - b'0',
        2 => (bytes[0] - b'0') * 10 + bytes[1] - b'0',
        _ => unreachable!(),
    }
}

#[test]
fn test_part1_example() {
    let input = include_str!("../input/day2_part1_example");
    assert_eq!(part1(input), 2)
}

#[test]
fn test_part1_input() {
    let input = include_str!("../input/day2_part1");
    assert_eq!(part1(input), 686)
}

#[inline(always)]
fn remove_level(record: &[u8], level: u8) -> impl Iterator<Item = u8> + '_ {
    (0u8..).zip(record.iter().copied()).filter(move |&(i, _)| i != level).map(|(_, v)| v)
}

pub fn part2(input: &str) -> u32 {
    let input = input.as_bytes();

    input
        .split(|c| *c == b'\n')
        .map(|line| {
            let mut record = ArrayVec::<[u8; 8]>::new();
            record.truncate(0);
            line.split(|c| *c == b' ').map(parse_int).for_each(|num| unsafe {
                record.as_mut_ptr().add(record.len()).write(num);
                record.set_len(record.len() + 1);
            });
            match is_record_safe(record.iter().copied()) {
                0 => 1,
                n @ 1.. => filter_record_safe(&record, n) as u32,
            }
        })
        .sum()
}

fn filter_record_safe(numbers: &[u8], n: u8) -> bool {
    if is_record_safe(remove_level(numbers, n)) == 0 {
        return true;
    }
    if is_record_safe(remove_level(numbers, n - 1)) == 0 {
        return true;
    }
    if is_record_safe(remove_level(numbers, n + 1)) == 0 {
        return true;
    }
    false
}

fn is_record_safe(mut record: impl Iterator<Item = u8>) -> u8 {
    let current: u8 = unsafe { record.next().unwrap_unchecked() };
    let next: u8 = unsafe { record.next().unwrap_unchecked() };
    if !matches!(current.abs_diff(next), 1..=3) {
        return 1;
    }
    let direction = current < next;

    let mut current = next;
    for (index, next) in (0u8..).zip(record) {
        if !(matches!(current.abs_diff(next), 1..=3) && (current < next) == direction) {
            return index + 1;
        }
        current = next;
    }
    0
}

#[test]
fn test_part2_example() {
    let input = include_str!("../input/day2_part1_example");
    assert_eq!(part2(input), 4)
}

#[test]
fn test_part2_input() {
    let input = include_str!("../input/day2_part1");
    assert_eq!(part2(input), 717)
}
