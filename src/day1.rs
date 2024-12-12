pub fn part1(input: &str) -> u32 {
    let [mut lhs_list, mut rhs_list] = parse(input.trim());

    lhs_list.sort_unstable();
    rhs_list.sort_unstable();

    lhs_list.iter().zip(rhs_list).map(|(lhs, rhs)| lhs.abs_diff(rhs)).sum()
}

fn parse(input: &str) -> [Box<[u32]>; 2] {
    const PRECISE_RN: usize = 1000 * 15 - 2;
    const PRECISE_NL: usize = 1000 * 14 - 1;

    let input = input.trim();
    let input_bytes = input.as_bytes();
    match input.len() {
        PRECISE_NL => precise_path::<PRECISE_NL>(input_bytes.try_into().unwrap(), 14),
        PRECISE_RN => precise_path::<PRECISE_RN>(input_bytes.try_into().unwrap(), 15),
        _ => parse_generic(input),
    }
}

fn precise_path<const LEN: usize>(input: &[u8; LEN], line_len: usize) -> [Box<[u32]>; 2] {
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

fn parse_line(line: &str) -> [u32; 2] {
    match line.len() {
        13 => {
            let lhs = parse_int5(line.as_bytes()[0..5].try_into().unwrap());
            let rhs = parse_int5(line.as_bytes()[8..13].try_into().unwrap());
            [lhs, rhs]
        }
        5 => [(line.as_bytes()[0] - b'0') as u32, (line.as_bytes()[4] - b'0') as u32],
        _ => {
            let (lhs, rhs) = line.split_once("   ").unwrap();
            [lhs, rhs].map(|num| num.parse().unwrap())
        }
    }
}

#[expect(clippy::trivially_copy_pass_by_ref)]
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

// #[test]
// fn part1_example() {
//     let input = include_str!("../input/day1_part1_example");
//     assert_eq!(part1(input), 11);
// }

#[test]
fn part1_input() {
    let input = include_str!("../input/day1.txt");
    assert_eq!(part1(input), 2_086_478);
}

pub fn part2(input: &str) -> u32 {
    let mut map = [0u8; 100_000];

    let [lhs_list, rhs_list] = parse(input.trim());
    let map_ptr = map.as_mut_ptr();

    for val in rhs_list {
        unsafe { *map_ptr.add(val as usize) += 1 };
    }

    let mut score = 0;
    for val in lhs_list {
        score += val * unsafe { map_ptr.add(val as usize).read() } as u32;
    }
    score
}

// #[test]
// #[ignore]
// fn part2_example() {
//     let input = include_str!("../input/day1_part1_example");
//     assert_eq!(part2(input), 31);
// }

#[test]
fn part2_input() {
    let input = include_str!("../input/day1.txt");
    assert_eq!(part2(input), 24_941_624);
}
