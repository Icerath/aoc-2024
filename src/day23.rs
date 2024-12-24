use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

pub fn part1(input: &str) -> u32 {
    part1_inner(input.as_bytes())
}

pub fn part2(input: &str) -> String {
    part2_inner(input.as_bytes())
}

fn part1_inner(input: &[u8]) -> u32 {
    let nodes = parse(input);
    let mut seen = HashSet::default();
    let mut sum = 0;
    for (&a, neighbours) in &nodes {
        seen.insert(a);
        for (i, b) in neighbours.iter().enumerate() {
            if seen.contains(b) {
                continue;
            }
            for c in &neighbours[i..] {
                if seen.contains(c) || (!nodes[b].contains(c)) {
                    continue;
                }
                sum += [&a, b, c].iter().any(|x| x.to_ne_bytes()[0] == b't') as u32;
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
