#![expect(clippy::cast_possible_wrap, static_mut_refs)]
use bstr::ByteSlice;
#[cfg(test)]
use std::sync::Mutex;

const LINE_LEN: usize = GRID_SIZE + 1;
const GRID_SIZE: usize = 141;
const INPUT_SIZE: usize = LINE_LEN * GRID_SIZE;

const DIRECTIONS: [isize; 4] = [1, LINE_LEN as isize, -1, -(LINE_LEN as isize)];

static mut VISITED: [[u32; 4]; INPUT_SIZE] = [[u32::MAX; 4]; INPUT_SIZE];

const TURN_COST_INCR: u32 = 1000;
const BUCKET_QUEUE_SIZE: u32 = 1001;

unsafe fn part1_inner(input: &[u8]) -> (usize, usize, u32) {
    static mut QUEUE: [Vec<(usize, u8)>; 1024] = [const { Vec::new() }; 1024];

    let start = input.find_byte(b'S').unwrap_unchecked();
    let end = input.find_byte(b'E').unwrap_unchecked();

    QUEUE.fill(const { Vec::new() });
    VISITED.fill([u32::MAX; 4]);
    let mut cost = 0;

    QUEUE[0].push((start, 0u8));
    VISITED[start][0] = 0;

    loop {
        let bucket = (cost % BUCKET_QUEUE_SIZE) as usize;
        cost += 1;

        while let Some((pos, dir)) = QUEUE.get_unchecked_mut(bucket).pop() {
            if pos == end {
                return (start, end, cost - 1);
            }

            let next_pos = pos.wrapping_add_signed(*DIRECTIONS.get_unchecked(dir as usize));

            if *input.get_unchecked(next_pos) != b'#'
                && cost < *VISITED.get_unchecked(next_pos).get_unchecked(dir as usize)
            {
                let bucket = (cost % BUCKET_QUEUE_SIZE) as usize;
                QUEUE[bucket].push((next_pos, dir));
                VISITED[next_pos][dir as usize] = cost;
            }
            for (pos, dir) in [(pos, (dir + 3) & 3), (pos, (dir + 1) & 3)] {
                let next_cost = cost + (TURN_COST_INCR - 1);
                if next_cost < *VISITED.get_unchecked(pos).get_unchecked(dir as usize) {
                    let bucket = (next_cost % BUCKET_QUEUE_SIZE) as usize;
                    QUEUE[bucket].push((pos, dir));
                    VISITED[pos][dir as usize] = next_cost;
                }
            }
        }
    }
}

unsafe fn part2_inner(input: &[u8]) -> u32 {
    static mut PATH: [u64; INPUT_SIZE / 64] = [0u64; INPUT_SIZE / 64];
    PATH.fill(0u64);

    let (start, end, cost) = part1_inner(input);
    let visited = &mut VISITED;

    let mut queue = (0u8..4)
        .filter(|&dir| visited[end][dir as usize] == cost)
        .map(|dir| (dir, end, cost))
        .collect::<Vec<_>>();

    while let Some((dir, pos, cost)) = queue.pop() {
        *PATH.get_unchecked_mut(pos / 64) |= 1 << (pos % 64);

        macro_rules! push_next {
            ($pos: expr, $dir: expr, $cost: expr) => {{
                let pos = $pos;
                let dir = $dir;
                let cost = $cost;
                if cost == *visited.get_unchecked(pos).get_unchecked(dir as usize) && pos != start {
                    queue.push((dir, pos, cost));
                    *visited.get_unchecked_mut(pos).get_unchecked_mut(dir as usize) = u32::MAX;
                }
            }};
        }
        push_next!(pos.wrapping_add_signed(-DIRECTIONS.get_unchecked(dir as usize)), dir, cost.wrapping_sub(1));
        push_next!(pos, (dir + 1) & 3, cost.wrapping_sub(TURN_COST_INCR));
        push_next!(pos, (dir + 3) & 3, cost.wrapping_sub(TURN_COST_INCR));
    }

    PATH.iter().map(|b| b.count_ones()).sum::<u32>() + 1
}

pub fn part1(input: &str) -> u32 {
    unsafe { part1_inner(input.as_bytes()).2 }
}

pub fn part2(input: &str) -> u32 {
    unsafe { part2_inner(input.as_bytes()) }
}

#[test]
fn test_part1() {
    let _guard = LOCK.lock();
    assert_eq!(part1(include_str!("../input/day16.txt")), 99448);
}

#[test]
fn test_part2() {
    let _guard = LOCK.lock();
    assert_eq!(part2(include_str!("../input/day16.txt")), 498);
}

#[cfg(test)]
static LOCK: Mutex<()> = Mutex::new(());
