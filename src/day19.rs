#![expect(static_mut_refs)]

use bstr::ByteSlice;
use std::hint::assert_unchecked;

static mut TRIE: [[u16; 6]; 2048] = [[0; 6]; 2048];
static mut NUM_OPTIONS: [u64; 128] = [0; 128];

unsafe fn both_parts<const PART1: bool>(mut input: &[u8]) -> u64 {
    TRIE.fill([0; 6]);
    let mut trie_len = 1u16;

    let mut i = 0;
    let mut node = 0;
    while *input.get_unchecked(i) != b'\n' {
        match *input.get_unchecked(i) {
            b',' => {
                TRIE.get_unchecked_mut(node)[NUM_TOWELS_SLOT] = 1;
                input = input.get_unchecked(i + 2..);
                node = 0;
                i = 0;
            }
            colour => {
                let hash = phf(colour);
                if TRIE.get_unchecked(node)[hash] == 0 {
                    TRIE.get_unchecked_mut(node)[hash] = trie_len;
                    node = trie_len as usize;
                    trie_len += 1;
                } else {
                    node = TRIE.get_unchecked(node)[hash] as usize;
                }
                i += 1;
            }
        }
    }
    TRIE.get_unchecked_mut(node)[NUM_TOWELS_SLOT] = 1;
    input = input.get_unchecked(i + 2..);
    let mut sum = 0;

    while !input.is_empty() {
        let nl = input.find_byte(b'\n').unwrap_unchecked();
        let desired = input.get_unchecked(..nl);
        input = input.get_unchecked(nl + 1..);

        assert_unchecked(desired.len() < NUM_OPTIONS.len());

        NUM_OPTIONS.get_unchecked_mut(1..desired.len() + 1).fill(0);
        *NUM_OPTIONS.get_unchecked_mut(0) = 1;

        for i in 0..desired.len() {
            if NUM_OPTIONS[i] == 0 {
                continue;
            }
            let mut node = 0;
            for j in i..desired.len() {
                node = TRIE.get_unchecked(node)[phf(desired[j])] as usize;
                let 1.. = node else { break };
                NUM_OPTIONS[j + 1] += NUM_OPTIONS[i] * TRIE.get_unchecked(node)[NUM_TOWELS_SLOT] as u64;
            }
        }

        sum += if PART1 { (NUM_OPTIONS[desired.len()] > 0) as u64 } else { NUM_OPTIONS[desired.len()] };
    }
    sum
}

pub fn part1(input: &str) -> u64 {
    unsafe { both_parts::<true>(input.as_bytes()) }
}

pub fn part2(input: &str) -> u64 {
    unsafe { both_parts::<false>(input.as_bytes()) }
}

#[test]
fn test_part1() {
    let __guard = LOCK.lock();
    assert_eq!(part1(include_str!("../input/day19.txt")), 258);
}

#[test]
fn test_part2() {
    let __guard = LOCK.lock();
    assert_eq!(part2(include_str!("../input/day19.txt")), 632_423_618_484_345);
}

#[cfg(test)]
static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

const NUM_TOWELS_SLOT: usize = 3;

#[inline(always)]
unsafe fn phf(colour: u8) -> usize {
    let ret = (((colour >> 4) ^ colour) & 7) as usize;
    assert_unchecked(ret < 6);
    assert_unchecked(ret != NUM_TOWELS_SLOT);
    ret
}
