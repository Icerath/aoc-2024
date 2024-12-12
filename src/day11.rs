#![expect(clippy::cast_possible_truncation)]

static LUT25: [u32; 10_000_000] = unsafe { std::mem::transmute(*include_bytes!("../luts/11a")) };
static LUT75: [u64; 10_000_000] = unsafe { std::mem::transmute(*include_bytes!("../luts/11b")) };

macro_rules! impl_part {
    ($input: ident, $lut: ident) => {{
        let mut input = $input.as_ptr();
        let mut sum = 0;
        let mut num = 0;
        loop {
            match input.read() {
                b'0'..=b'9' => num = (num * 10) + (input.read() - b'0') as u64,
                b' ' => sum += $lut.get_unchecked(std::mem::take(&mut num) as usize),
                b'\n' => break,
                _ => std::hint::unreachable_unchecked(),
            }
            input = input.add(1);
        }
        sum + $lut.get_unchecked(num as usize)
    }};
}

pub fn part1(input: &str) -> u32 {
    unsafe { impl_part!(input, LUT25) }
}

pub fn part2(input: &str) -> u64 {
    unsafe { impl_part!(input, LUT75) }
}

#[test]
fn test_part1_input() {
    assert_eq!(part1(include_str!("../input/day11.txt")), 194_557);
}

#[test]
fn test_part2_input() {
    assert_eq!(part2(include_str!("../input/day11.txt")), 231_532_558_973_909);
}
