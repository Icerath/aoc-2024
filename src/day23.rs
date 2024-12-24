#![expect(clippy::cast_possible_truncation, static_mut_refs)]
use std::hint::{assert_unchecked, unreachable_unchecked};
use tinyvec::ArrayVec;

#[inline(always)]
pub fn part1(input: &str) -> u32 {
    unsafe { part1_inner(input.as_bytes()) }
}

#[inline(always)]
pub fn part2(input: &str) -> &'static str {
    unsafe { part2_inner(input.as_bytes()) }
}

const MAX_CONNECTIONS: usize = 16;

#[inline(always)]
unsafe fn part1_inner(input: &[u8]) -> u32 {
    parse(input);
    let mut sum = 0;
    for a in 494u16..520 {
        for (i, &b) in NODES.get_unchecked(a as usize).iter().enumerate() {
            if b >= 494 && b < a {
                continue;
            }
            for &c in NODES.get_unchecked(a as usize).get_unchecked(i + 1..) {
                if c >= 494 && c < a || !NODES.get_unchecked(b as usize).contains(&c) {
                    continue;
                }
                sum += 1;
            }
        }
    }
    sum
}

#[inline(always)]
unsafe fn part2_inner(input: &[u8]) -> &'static str {
    static mut STR_OUTPUT: [u8; 64] = [b','; 64];

    parse(input);

    let mut longest = ArrayVec::<[u16; MAX_CONNECTIONS]>::new();
    let mut clique = ArrayVec::<[u16; MAX_CONNECTIONS]>::new();

    let mut seen = [false; 26 * 26];
    for a in 0..(26 * 26) {
        if *seen.get_unchecked(a as usize) {
            continue;
        }
        clique.push(a);
        for &b in NODES.get_unchecked(a as usize) {
            if *seen.get_unchecked(b as usize) {
                continue;
            }
            if clique.iter().all(|&c| NODES.get_unchecked(b as usize).contains(&c)) {
                *seen.get_unchecked_mut(b as usize) = true;
                clique.push(b);
            }
        }
        if clique.len() > longest.len() {
            std::mem::swap(&mut longest, &mut clique);
        }
        clique.clear();
    }

    longest.sort_unstable_by_key(|computer| [computer / 26, computer % 26]);
    let mut str_len = 0;
    for computer in longest {
        *STR_OUTPUT.get_unchecked_mut(str_len) = (computer / 26) as u8 + b'a';
        str_len += 1;
        *STR_OUTPUT.get_unchecked_mut(str_len) = (computer % 26) as u8 + b'a';
        str_len += 2;
    }
    std::str::from_utf8_unchecked(STR_OUTPUT.get_unchecked(..str_len - 1))
}

static mut NODES: [ArrayVec<[u16; MAX_CONNECTIONS]>; 26 * 26] =
    [ArrayVec::from_array_empty([0; MAX_CONNECTIONS]); 26 * 26];

#[inline(always)]
unsafe fn parse(mut input: &[u8]) {
    NODES.fill(ArrayVec::from_array_empty([0; MAX_CONNECTIONS]));

    while !input.is_empty() {
        assert_unchecked(input.len() >= 6);
        let lhs = 26 * (input[0] - b'a') as u16 + (input[1] - b'a') as u16;
        let rhs = 26 * (input[3] - b'a') as u16 + (input[4] - b'a') as u16;

        assert_unchecked(lhs < 26 * 26);
        assert_unchecked(rhs < 26 * 26);

        let None = NODES[lhs as usize].try_push(rhs) else { unreachable_unchecked() };
        let None = NODES[rhs as usize].try_push(lhs) else { unreachable_unchecked() };

        input = &input[6..];
    }
}

pub const PART1_OUT: u32 = 1083;
pub const PART2_OUT: &str = "as,bu,cp,dj,ez,fd,hu,it,kj,nx,pp,xh,yu";
