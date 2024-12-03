use tinyvec::ArrayVec;

pub fn part1(input: &str) -> u32 {
    let input = input.as_bytes();
    let mut safe_records = 0u32;
    let mut i = 0;

    while i < input.len() {
        let mut numbers = ArrayVec::<[u8; 8]>::new();

        while i < input.len() && input[i] != b'\n' {
            let is_single_digit = i + 1 == input.len() || matches!(input[i + 1], b' ' | b'\n');
            let num = if is_single_digit {
                input[i] - b'0'
            } else {
                (input[i] - b'0') * 10 + input[i + 1] - b'0'
            };
            i += (!is_single_digit) as usize + 1;

            unsafe {
                *numbers.as_mut_ptr().add(numbers.len()) = num;
                numbers.set_len(numbers.len() + 1);
            }
            if i < input.len() && input[i] == b' ' {
                i += 1;
            }
        }
        // skip newline
        i += 1;

        safe_records += (is_record_safe(numbers.iter().copied()) == 0) as u32;
    }

    safe_records
}

#[test]
fn test_part1_example() {
    let input = include_str!("../input/day2_part1_example");
    assert_eq!(part1(input), 2);
}

#[test]
fn test_part1_input() {
    let input = include_str!("../input/day2_part1");
    assert_eq!(part1(input), 686);
}

#[inline(always)]
fn remove_level(record: &[u8], level: u8) -> impl Iterator<Item = u8> + '_ {
    (0u8..).zip(record.iter().copied()).filter(move |&(i, _)| i != level).map(|(_, v)| v)
}

pub fn part2(input: &str) -> u32 {
    let input = input.as_bytes();
    let mut safe_records = 0u32;
    let mut i = 0;

    while i < input.len() {
        let mut numbers = ArrayVec::<[u8; 8]>::new();

        while i < input.len() && input[i] != b'\n' {
            let is_single_digit = i + 1 == input.len() || matches!(input[i + 1], b' ' | b'\n');
            let num = if is_single_digit {
                input[i] - b'0'
            } else {
                (input[i] - b'0') * 10 + input[i + 1] - b'0'
            };
            i += (!is_single_digit) as usize + 1;

            unsafe {
                *numbers.as_mut_ptr().add(numbers.len()) = num;
                numbers.set_len(numbers.len() + 1);
            }
            if i < input.len() && input[i] == b' ' {
                i += 1;
            }
        }
        // skip newline
        i += 1;

        safe_records += match is_record_safe(numbers.iter().copied()) {
            0 => 1,
            n @ 1.. => filter_record_safe(&numbers, n) as u32,
        };
    }

    safe_records
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
    assert_eq!(part2(input), 4);
}

#[test]
fn test_part2_input() {
    let input = include_str!("../input/day2_part1");
    assert_eq!(part2(input), 717);
}
