pub const EXAMPLE_1: &str = "
3   4
4   3
2   5
1   3
3   9
3   3";

pub fn part1(input: &str) -> u32 {
    let [mut lhs_list, mut rhs_list] = parse(input.trim());

    lhs_list.sort_unstable();
    rhs_list.sort_unstable();

    lhs_list
        .iter()
        .zip(rhs_list)
        .map(|(lhs, rhs)| lhs.abs_diff(rhs))
        .sum()
}

pub fn parse(input: &str) -> [Box<[u32]>; 2] {
    let input = input.trim();
    let input_bytes = input.as_bytes();
    match input.len() {
        PRECISE_RN_PATH_LEN => rn_precise_path(input_bytes.try_into().unwrap()),
        PRECISE_NL_PATH_LEN => nl_precise_path(input_bytes.try_into().unwrap()),
        _ => parse_generic(input),
    }
}

const PRECISE_RN_PATH_LEN: usize = 1000 * 15 - 2;
fn rn_precise_path(input: &[u8; PRECISE_RN_PATH_LEN]) -> [Box<[u32]>; 2] {
    precise_path_n(input, 15)
}

const PRECISE_NL_PATH_LEN: usize = 1000 * 14 - 1;
fn nl_precise_path(input: &[u8; PRECISE_NL_PATH_LEN]) -> [Box<[u32]>; 2] {
    precise_path_n(input, 14)
}

fn precise_path_n<const LEN: usize>(input: &[u8; LEN], line_len: usize) -> [Box<[u32]>; 2] {
    let mut lhs_list = Box::new_uninit_slice(1000);
    let mut rhs_list = Box::new_uninit_slice(1000);

    for len in 0..1000 {
        let i = len * line_len;
        let lhs = parse_int5(input[i..i + 5].try_into().unwrap());
        let rhs = parse_int5(input[i + 8..i + 13].try_into().unwrap());
        unsafe { lhs_list.as_mut_ptr().cast::<u32>().add(len).write(lhs) };
        unsafe { rhs_list.as_mut_ptr().cast::<u32>().add(len).write(rhs) };
    }
    [lhs_list, rhs_list].map(|list| unsafe { list.assume_init() })
}

fn parse_generic(input: &str) -> [Box<[u32]>; 2] {
    let mut lhs_list = Vec::with_capacity(1000);
    let mut rhs_list = Vec::with_capacity(1000);

    for line in input.lines() {
        let [lhs, rhs] = parse_line(line);
        lhs_list.push(lhs);
        rhs_list.push(rhs);
    }
    [lhs_list.into(), rhs_list.into()]
}

pub fn parse_line(line: &str) -> [u32; 2] {
    match line.len() {
        13 => {
            let lhs = parse_int5(line.as_bytes()[0..5].try_into().unwrap());
            let rhs = parse_int5(line.as_bytes()[8..13].try_into().unwrap());
            [lhs, rhs]
        }
        5 => [
            (line.as_bytes()[0] - b'0') as u32,
            (line.as_bytes()[4] - b'0') as u32,
        ],
        _ => {
            let (lhs, rhs) = line.split_once("   ").unwrap();
            [lhs, rhs].map(|num| num.parse().unwrap())
        }
    }
}

fn parse_int5(bytes: &[u8; 5]) -> u32 {
    let zero = b'0' as u32;
    let offset = zero * 10000 + zero * 1000 + zero * 100 + zero * 10 + zero;

    (bytes[0] as u32 * 10000
        + (bytes[1]) as u32 * 1000
        + (bytes[2]) as u32 * 100
        + (bytes[3]) as u32 * 10
        + (bytes[4]) as u32)
        - offset
}

#[test]
fn part1_example() {
    assert_eq!(part1(EXAMPLE_1), 11);
}

#[test]
fn part1_input() {
    let input = include_str!("../input/day1_part1");
    assert_eq!(part1(input), 2086478);
}

pub fn part2(input: &str) -> u32 {
    // I hate this with a passion but it's fast :/
    static mut MAP: [u8; 100_000] = [0; 100_000];

    let [lhs_list, rhs_list] = parse(input.trim());
    let map_ptr = (&raw mut MAP).cast::<u8>();

    for val in rhs_list {
        unsafe { *map_ptr.add(val as usize) += 1 };
    }

    let mut score = 0;
    for val in lhs_list {
        score += val * unsafe { map_ptr.add(val as usize).read() } as u32;
    }
    score
}

#[test]
fn part2_example() {
    assert_eq!(part2(EXAMPLE_1), 31);
}

#[test]
fn part2_input() {
    let input = include_str!("../input/day1_part1");
    assert_eq!(part2(input), 24941624);
}
