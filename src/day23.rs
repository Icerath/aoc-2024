use std::hint::{assert_unchecked, unreachable_unchecked};

use rustc_hash::FxHashMap as HashMap;
use tinyvec::ArrayVec;

pub fn part1(input: &str) -> u32 {
    unsafe { part1_inner(input.as_bytes()) }
}

pub fn part2(input: &str) -> String {
    part2_inner(input.as_bytes())
}

// FIXME: choose a logical number
const MAX_CONNECTIONS: usize = 32;

unsafe fn part1_inner(mut input: &[u8]) -> u32 {
    let mut nodes = vec![ArrayVec::<[u16; MAX_CONNECTIONS]>::new(); 26 * 26];
    let mut edges = vec![[false; 26 * 26]; 26 * 26];
    while !input.is_empty() {
        assert_unchecked(input.len() >= 6);
        let lhs = 26 * (input[0] - b'a') as u16 + (input[1] - b'a') as u16;
        let rhs = 26 * (input[3] - b'a') as u16 + (input[4] - b'a') as u16;

        assert_unchecked(lhs < 26 * 26);
        assert_unchecked(rhs < 26 * 26);

        let None = nodes[lhs as usize].try_push(rhs) else { unreachable_unchecked() };
        let None = nodes[rhs as usize].try_push(lhs) else { unreachable_unchecked() };

        edges[lhs as usize][rhs as usize] = true;
        edges[rhs as usize][lhs as usize] = true;

        input = &input[6..];
    }
    let mut sum = 0;
    for a in 494u16..520 {
        let neighbours = &nodes[a as usize];
        for (i, &b) in neighbours.iter().enumerate() {
            if b >= 494 && b < a {
                continue;
            }
            for &c in &neighbours[i..] {
                if c >= 494 && c < a || !edges[b as usize][c as usize] {
                    continue;
                }
                sum += 1;
            }
        }
    }
    sum
}

fn part2_inner(input: &[u8]) -> String {
    let nodes = parse(input);

    let mut clique = vec![];
    let mut longest = vec![];
    for (&a, neighbours) in &nodes {
        clique.push(a);
        for b in neighbours {
            if clique.iter().all(|c| nodes[b].contains(c)) {
                clique.push(*b);
            }
        }
        if clique.len() > longest.len() {
            std::mem::swap(&mut longest, &mut clique);
        }
        clique.clear();
    }

    longest.sort_unstable_by_key(|x| x.to_ne_bytes());
    let mut result = vec![];
    for n in longest {
        result.extend(n.to_ne_bytes());
        result.push(b',');
    }
    result.pop();
    String::from_utf8(result).unwrap()
}

pub const PART1_OUT: u32 = 1083;
pub const PART2_OUT: &str = "as,bu,cp,dj,ez,fd,hu,it,kj,nx,pp,xh,yu";

fn parse(input: &[u8]) -> HashMap<u16, Vec<u16>> {
    let input = &input[..input.len() - 1];
    let mut nodes = HashMap::<u16, Vec<_>>::default();

    for line in input.split(|&b| b == b'\n') {
        let lhs = u16::from_ne_bytes([line[0], line[1]]);
        let rhs = u16::from_ne_bytes([line[3], line[4]]);

        nodes.entry(lhs).or_default().push(rhs);
        nodes.entry(rhs).or_default().push(lhs);
    }
    nodes
}
