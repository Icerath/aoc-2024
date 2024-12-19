#![expect(static_mut_refs, clippy::cast_possible_truncation)]

const GRID_SIZE: usize = 71;
const GRID_LEN: usize = GRID_SIZE * GRID_SIZE;
const END: usize = GRID_LEN - 1;
const QUEUE_CAPACITY: usize = 512;
const SKIP: u16 = 1024;

static mut QUEUE: [[u16; 2]; QUEUE_CAPACITY] = [[0; 2]; QUEUE_CAPACITY];
static mut GRID: [u16; GRID_LEN] = [u16::MAX; GRID_LEN];

pub fn part1(input: &str) -> u16 {
    unsafe { part1_inner(input.as_bytes()) }
}

unsafe fn part1_inner(input: &[u8]) -> u16 {
    parse(input);

    GRID[0] = 0;

    let mut queue = Queue { head: 0, len: 0 };
    queue.push_back([0u16, 0]);

    loop {
        let [pos, cost] = queue.pop_front();
        let pos = pos as usize;
        if pos == END {
            return cost;
        }

        macro_rules! push {
            ($contains: expr, $pos: expr) => {
                let pos;
                if $contains && {
                    pos = $pos;
                    *GRID.get_unchecked(pos) > SKIP
                } {
                    queue.push_back([pos as u16, cost + 1]);
                    *GRID.get_unchecked_mut(pos) = 0;
                }
            };
        }

        push!(pos >= GRID_SIZE, pos - GRID_SIZE);
        push!(pos < (GRID_LEN - GRID_SIZE), pos + GRID_SIZE);
        push!(pos % GRID_SIZE != 0, pos - 1);
        push!(pos % GRID_SIZE != 70, pos + 1);
    }
}

pub fn part2(input: &str) -> String {
    unsafe { part2_inner(input.as_bytes()) }
}

unsafe fn part2_inner(input: &[u8]) -> String {
    parse(input);

    let mut lo = 0;
    let mut hi = GRID_LEN as u16;

    loop {
        let mid = (lo + hi) / 2;
        if is_reachable(mid) {
            lo = mid + 1;
        } else {
            hi = mid;
        };
        if lo >= hi {
            break;
        }
    }

    let pos = GRID.iter().position(|&instant| instant == lo).unwrap();
    let [x, y] = [pos % GRID_SIZE, pos / GRID_SIZE];
    format!("{x},{y}")
}

unsafe fn is_reachable(instant: u16) -> bool {
    static mut SEEN: [u16; GRID_LEN] = [u16::MAX; GRID_LEN];
    SEEN.copy_from_slice(&GRID);
    SEEN[0] = 0;

    let mut queue = Queue { head: 0, len: 0 };
    queue.push_back([0, 0]);

    while queue.len > 0 {
        let [pos, cost] = queue.pop_front();
        let pos = pos as usize;
        if pos == END {
            return true;
        }
        macro_rules! push {
            ($contains: expr, $next: expr) => {
                let next;
                if $contains && {
                    next = $next;
                    instant < *SEEN.get_unchecked(next)
                } {
                    queue.push_back([next as u16, cost + 1]);
                    *SEEN.get_unchecked_mut(next) = 0;
                }
            };
        }

        push!(pos >= GRID_SIZE, pos - GRID_SIZE);
        push!(pos < (GRID_LEN - GRID_SIZE), pos + GRID_SIZE);
        push!(pos % GRID_SIZE != 0, pos - 1);
        push!(pos % GRID_SIZE != 70, pos + 1);
    }
    false
}

#[test]
fn test_part1() {
    let _guard = LOCK.lock();
    assert_eq!(part1(include_str!("../input/day18.txt")), 330);
}

#[test]
fn test_part2() {
    let _guard = LOCK.lock();
    assert_eq!(part2(include_str!("../input/day18.txt")), "10,38");
}

#[cfg(test)]
static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

unsafe fn parse(mut input: &[u8]) {
    GRID.fill(u16::MAX);
    for i in 0.. {
        let mut x = (input.get_unchecked(0) - b'0') as usize;
        input = input.get_unchecked(1..);

        if *input.get_unchecked(0) != b',' {
            x = x * 10 + (*input.get_unchecked(0) - b'0') as usize;
            input = input.get_unchecked(1..);
        }
        input = input.get_unchecked(1..);

        let mut y = (input.get_unchecked(0) - b'0') as usize;
        input = input.get_unchecked(1..);

        if *input.get_unchecked(0) != b'\n' {
            y = y * 10 + (*input.get_unchecked(0) - b'0') as usize;
            input = input.get_unchecked(1..);
        }
        input = input.get_unchecked(1..);

        *GRID.get_unchecked_mut(x + y * 71) = i;

        if input.is_empty() {
            break;
        }
    }
}

struct Queue {
    head: usize,
    len: usize,
}

/// This code is ripped from `std::collections::VecDeque`
impl Queue {
    #[inline(always)]
    unsafe fn pop_front(&mut self) -> [u16; 2] {
        let old_head = self.head;
        self.head = self.to_physical_idx(1);
        self.len -= 1;
        *QUEUE.get_unchecked(old_head)
    }
    #[inline(always)]
    unsafe fn push_back(&mut self, value: [u16; 2]) {
        *QUEUE.get_unchecked_mut(self.to_physical_idx(self.len)) = value;
        self.len += 1;
    }
    #[inline(always)]
    fn to_physical_idx(&self, idx: usize) -> usize {
        Self::wrap_add(self.head, idx)
    }
    #[inline(always)]
    fn wrap_add(idx: usize, addend: usize) -> usize {
        Self::wrap_index(idx.wrapping_add(addend), QUEUE_CAPACITY)
    }
    #[inline(always)]
    fn wrap_index(logical_index: usize, capacity: usize) -> usize {
        if logical_index >= capacity {
            logical_index - capacity
        } else {
            logical_index
        }
    }
}
