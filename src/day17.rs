#![expect(static_mut_refs, clippy::cast_possible_truncation)]

use std::hint::unreachable_unchecked;

pub fn part1(input: &str) -> &'static str {
    unsafe { part1_inner(input.as_bytes()) }
}

unsafe fn parse(mut input: &[u8]) -> (u64, &[u8]) {
    input = input.get_unchecked(12..);

    let mut a = 0;
    while *input.get_unchecked(0) != b'\n' {
        a = a * 10 + (*input.get_unchecked(0) - b'0') as u64;
        input = input.get_unchecked(1..);
    }

    let program = input.get_unchecked(1 + 38..);
    (a, program)
}

unsafe fn part1_inner(input: &[u8]) -> &'static str {
    static mut OUTPUT: [u8; 64] = [b','; 64];

    let (mut a, program) = parse(input);
    let (mut b, mut c, mut instruction_ptr) = (0, 0, 0);

    let mut output_index = 0;
    macro_rules! push_output {
        ($combo: expr) => {{
            *OUTPUT.get_unchecked_mut(output_index) = ($combo) as u8 + b'0';
            output_index += 2;
        }};
    }

    while instruction_ptr < program.len() {
        let literal = || (*program.get_unchecked(instruction_ptr + 2) - b'0') as u64;
        let combo = || match literal() {
            n @ 0..=3 => n,
            4 => a,
            5 => b,
            6 => c,
            _ => unreachable_unchecked(),
        };

        match *program.get_unchecked(instruction_ptr) {
            b'0' => a >>= combo(),
            b'1' => b ^= literal(),
            b'2' => b = combo() & 7,
            b'3' if a != 0 => instruction_ptr = (literal() * 2).wrapping_sub(4) as usize,
            b'3' => {}
            b'4' => b ^= c,
            b'5' => push_output!(combo() & 7),
            b'7' => c = a >> combo(),
            _ => unreachable_unchecked(),
        }

        instruction_ptr = instruction_ptr.wrapping_add(4);
    }
    std::str::from_utf8_unchecked(OUTPUT.get_unchecked(..output_index - 1))
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../input/day17.txt")), "7,3,5,7,5,7,4,3,0");
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("../input/day17.txt")), 105_734_774_294_938);
}

pub fn part2(input: &str) -> u64 {
    unsafe { part2_inner(input.as_bytes()) }
}

unsafe fn part2_inner(input: &[u8]) -> u64 {
    let (_, program) = unsafe { parse(input) };
    recurse(program, program.len() - 2, 0).unwrap_unchecked()
}

unsafe fn recurse(program: &[u8], index: usize, mut a: u64) -> Option<u64> {
    for _ in 0..8 {
        if run(program, a) == (*program.get_unchecked(index) - b'0') as u64 {
            let Some(index) = index.checked_sub(2) else { return Some(a) };
            if let Some(a) = recurse(program, index, a << 3) {
                return Some(a);
            }
        }
        a += 1;
    }
    None
}

unsafe fn run(program: &[u8], mut a: u64) -> u64 {
    let (mut b, mut c, mut instruction_ptr) = (0, 0, 0);
    while instruction_ptr < program.len() {
        let literal = || (*program.get_unchecked(instruction_ptr + 2) - b'0') as u64;
        let combo = || match literal() {
            n @ 0..=3 => n,
            4 => a,
            5 => b,
            6 => c,
            _ => unreachable_unchecked(),
        };

        match *program.get_unchecked(instruction_ptr) {
            b'0' => a >>= combo(),
            b'1' => b ^= literal(),
            b'2' => b = combo() % 8,
            b'3' if a != 0 => instruction_ptr = (literal() * 2).wrapping_sub(4) as usize,
            b'3' => {}
            b'4' => b ^= c,
            b'5' => return combo() % 8,
            b'7' => c = a >> combo(),
            _ => unreachable_unchecked(),
        }

        instruction_ptr = instruction_ptr.wrapping_add(4);
    }
    unreachable_unchecked()
}
