#![expect(clippy::cast_possible_truncation, static_mut_refs, clippy::items_after_statements)]
use std::hint::{assert_unchecked, unreachable_unchecked};

#[inline(always)]
pub fn part1(input: &str) -> u64 {
    unsafe { part1_inner(input.as_bytes()) }
}

#[inline(always)]
pub fn part2(input: &str) -> &'static str {
    unsafe { part2_inner(input.as_bytes()) }
}

macro_rules! parse_gate_connections {
    ($input: ident) => {{
        assert_unchecked($input.len() >= 18);
        let lhs = compress_gate($input);
        let op = *$input.get_unchecked(4);
        $input = match op {
            b'O' => &$input.get_unchecked(7..),
            b'A' | b'X' => &$input.get_unchecked(8..),
            _ => unreachable_unchecked(),
        };
        let rhs = compress_gate($input);
        let output = compress_gate(&$input.get_unchecked(7..));
        $input = $input.get_unchecked(11..);
        (lhs, op, rhs, output)
    }};
}

static mut IDS: [Option<u16>; ZEDS[0] as usize + 1] = [None; ZEDS[0] as usize + 1];
static mut GATES: [Option<bool>; ZEDS[0] as usize + 1] = [None; ZEDS[0] as usize + 1];
static mut OPS: [(u16, u8, u16); ZEDS[0] as usize + 1] = [(0, 0, 0); ZEDS[0] as usize + 1];

#[inline(always)]
unsafe fn part1_inner(mut input: &[u8]) -> u64 {
    let mut num_gates = 0;
    while *input.get_unchecked(0) != b'\n' {
        assert_unchecked(input.len() >= 8);
        *IDS.get_unchecked_mut(compress_gate(input) as usize) = Some(num_gates as u16);
        *GATES.get_unchecked_mut(num_gates) = Some(input[5] == b'1');
        num_gates += 1;
        input = &input[7..];
    }
    input = input.get_unchecked(1..);

    let mut id = num_gates as u16;
    while !input.is_empty() {
        let (lhs, op, rhs, output) = parse_gate_connections!(input);
        let lhs = *IDS.get_unchecked_mut(lhs as usize).get_or_insert(id);
        let rhs = *IDS.get_unchecked_mut(rhs as usize).get_or_insert(id + 1);
        let output = *IDS.get_unchecked_mut(output as usize).get_or_insert(id + 2);
        id += 3;

        *OPS.get_unchecked_mut(output as usize) = (lhs, op, rhs);
    }
    let mut result = 0u64;
    for key in ZEDS {
        let Some(key) = *IDS.get_unchecked(key as usize) else { continue };
        result <<= 1;
        result |= dfs(key) as u64;
    }

    result
}

unsafe fn dfs(key: u16) -> bool {
    if let Some(value) = *GATES.get_unchecked(key as usize) {
        return value;
    }
    let (lhs, op, rhs) = *OPS.get_unchecked(key as usize);
    let value = match op {
        b'O' => dfs(lhs) | dfs(rhs),
        b'A' => dfs(lhs) & dfs(rhs),
        b'X' => dfs(lhs) ^ dfs(rhs),
        _ => unreachable_unchecked(),
    };
    *GATES.get_unchecked_mut(key as usize).insert(value)
}

pub const PART1_OUT: u64 = 42_049_478_636_360;
pub const PART2_OUT: &str = "cph,gws,hgj,nnt,npf,z13,z19,z33";

static mut XORS: [bool; ZEDS[0] as usize + 1] = [false; ZEDS[0] as usize + 1];
static mut ORS: [bool; ZEDS[0] as usize + 1] = [false; ZEDS[0] as usize + 1];

#[inline(always)]
unsafe fn part2_inner(mut input: &[u8]) -> &'static str {
    static mut GATES: [(u16, u8, u16, u16); 1024] = [(0, 0, 0, 0); 1024];
    while *input.get_unchecked(7) != b'\n' {
        input = input.get_unchecked(7..);
    }
    input = input.get_unchecked(8..);

    let mut num_gates = 0;
    while !input.is_empty() {
        let (lhs, op, rhs, output) = parse_gate_connections!(input);
        *GATES.get_unchecked_mut(num_gates) = (lhs, op, rhs, output);
        num_gates += 1;
        macro_rules! insert {
            ($lookup: ident) => {{
                *$lookup.get_unchecked_mut(lhs as usize) = true;
                *$lookup.get_unchecked_mut(rhs as usize) = true;
            }};
        }
        match op {
            b'X' => insert!(XORS),
            b'O' => insert!(ORS),
            _ => {}
        };
    }

    let mut swapped = [0u16; 8];
    let mut swapped_len = 0u16;
    macro_rules! push_swapped {
        ($output: expr) => {{
            *swapped.get_unchecked_mut(swapped_len as usize) = $output;
            swapped_len += 1;
        }};
    }
    for &(lhs, op, rhs, output) in GATES.get_unchecked(..num_gates) {
        match op {
            b'O' if first_char(output) == b'z' && output != Z45 => push_swapped!(output),
            b'A' if lhs != X00 && rhs != X00 && !ORS.get_unchecked(output as usize) => push_swapped!(output),
            b'X' if first_char(lhs) == b'x' || first_char(rhs) == b'x' => {
                if lhs != X00 && rhs != X00 && !XORS.get_unchecked(output as usize) {
                    push_swapped!(output);
                }
            }
            b'X' if first_char(output) != b'z' => push_swapped!(output),
            _ => {}
        }
    }
    let mut swapped: Vec<_> = swapped.into_iter().map(gate_string).collect();
    swapped.sort_unstable();
    static mut OUTPUT: [u8; 31] = [b','; 31];
    for (i, s) in swapped.iter().enumerate() {
        *OUTPUT.get_unchecked_mut(i * 4) = s[0];
        *OUTPUT.get_unchecked_mut(i * 4 + 1) = s[1];
        *OUTPUT.get_unchecked_mut(i * 4 + 2) = s[2];
    }
    std::str::from_utf8_unchecked(&OUTPUT)
}

const X00: u16 = unsafe { compress_gate(b"x00") };
const Z45: u16 = unsafe { compress_gate(b"z45") };

fn gate_string(key: u16) -> [u8; 3] {
    let c1 = key / (36 * 36);
    let c1 = b'a' + c1 as u8;
    let remaining = key % (36 * 36);
    let c2 = (remaining / 36) as u8;
    let c2 = if c2 < 26 { b'a' + c2 } else { b'0' + (c2 - 26) };
    let c3 = (remaining % 36) as u8;
    let c3 = if c3 < 26 { b'a' + c3 } else { b'0' + (c3 - 26) };
    [c1, c2, c3]
}

#[inline(always)]
const unsafe fn compress_gate(c: &[u8]) -> u16 {
    assert_unchecked(c.len() >= 3);
    let c1 = c[0] - b'a';
    let c2 = if c[1].is_ascii_digit() { c[1] - b'0' + 26 } else { c[1] - b'a' };
    let c3 = if c[2].is_ascii_digit() { c[2] - b'0' + 26 } else { c[2] - b'a' };
    c1 as u16 * 36 * 36 + c2 as u16 * 36 + c3 as u16
}

const fn first_char(gate: u16) -> u8 {
    (gate / (36 * 36)) as u8 + b'a'
}

const ZEDS: [u16; 64] = [
    33581, 33580, 33579, 33578, 33551, 33550, 33549, 33548, 33547, 33546, 33545, 33544, 33543, 33542, 33515,
    33514, 33513, 33512, 33511, 33510, 33509, 33508, 33507, 33506, 33479, 33478, 33477, 33476, 33475, 33474,
    33473, 33472, 33471, 33470, 33443, 33442, 33441, 33440, 33439, 33438, 33437, 33436, 33435, 33434, 33407,
    33406, 33405, 33404, 33403, 33402, 33401, 33400, 33399, 33398, 33371, 33370, 33369, 33368, 33367, 33366,
    33365, 33364, 33363, 33362,
];
