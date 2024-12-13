const INPUT_SIZE: usize = LINE_WIDTH * GRID_WIDTH;
const LINE_WIDTH: usize = GRID_WIDTH + 1;
const GRID_WIDTH: usize = 140;

unsafe fn part1_inner(input: &[u8]) -> u32 {
    let mut checked = vec![false; input.len()];
    let mut queue = Vec::with_capacity(128);

    let mut sum = 0;
    for i in 0..INPUT_SIZE - 1 {
        if checked[i] || i % LINE_WIDTH == 140 {
            continue;
        }
        checked[i] = true;

        let mut area = 0;
        let mut perimeter = 0;

        queue.push(i);
        while let Some(pos) = queue.pop() {
            area += 1;

            macro_rules! loop_body {
                ($pos: expr, $same: expr) => {{
                    let pos = $pos;
                    if !($same && input[pos] == input[i]) {
                        perimeter += 1;
                    } else if !checked[pos] {
                        queue.push(pos);
                        checked[pos] = true;
                    }
                }};
            }

            let x = pos % LINE_WIDTH;
            loop_body!(pos.wrapping_sub(1), x.wrapping_sub(1) < GRID_WIDTH);
            loop_body!(pos.wrapping_sub(LINE_WIDTH), pos > GRID_WIDTH);
            loop_body!(pos + 1, x + 1 < GRID_WIDTH);
            loop_body!(pos + LINE_WIDTH, pos < INPUT_SIZE - LINE_WIDTH);
        }
        sum += area * perimeter;
    }

    sum
}

unsafe fn part2_inner(input: &[u8]) -> u32 {
    let mut checked = vec![false; input.len()];
    let mut queue = Vec::with_capacity(128);

    let mut sum = 0;
    for i in 0..INPUT_SIZE - 1 {
        if checked[i] || i % LINE_WIDTH == 140 {
            continue;
        }
        checked[i] = true;

        let mut area = 0;
        let mut corners = 0;

        queue.push(i);
        while let Some(pos) = queue.pop() {
            let [x, y] = [pos % LINE_WIDTH, pos / LINE_WIDTH];
            area += 1;

            let adj_eq = [
                x != 0 && input[pos - 1] == input[i],
                y != 0 && input[pos - LINE_WIDTH] == input[i],
                x + 1 < GRID_WIDTH && input[pos + 1] == input[i],
                y + 1 < GRID_WIDTH && input[pos + LINE_WIDTH] == input[i],
            ];

            macro_rules! count_corners {
                ($i: literal => $j: literal, $diag: expr) => {
                    let is_corner = (!adj_eq[$i] && !adj_eq[$j])
                        || adj_eq[$i] && adj_eq[$j] && {
                            let pos = $diag;
                            !(pos < input.len() && input[pos] == input[i])
                        };
                    corners += is_corner as u32;
                };
            }
            count_corners!(0 => 1, pos.wrapping_sub(LINE_WIDTH + 1));
            count_corners!(1 => 2, pos.wrapping_sub(LINE_WIDTH - 1));
            count_corners!(2 => 3, pos + LINE_WIDTH + 1);
            count_corners!(3 => 0, pos + LINE_WIDTH - 1);

            macro_rules! push_if_different {
                ($pos: expr, $same: expr) => {{
                    let pos = $pos;
                    if ($same && input[pos] == input[i]) && !checked[pos] {
                        queue.push(pos);
                        checked[pos] = true;
                    }
                }};
            }

            push_if_different!(pos.wrapping_sub(1), x.wrapping_sub(1) < GRID_WIDTH);
            push_if_different!(pos.wrapping_sub(LINE_WIDTH), pos > GRID_WIDTH);
            push_if_different!(pos + 1, x + 1 < GRID_WIDTH);
            push_if_different!(pos + LINE_WIDTH, pos < INPUT_SIZE - LINE_WIDTH);
        }
        sum += area * corners;
    }

    sum
}

pub fn part1(input: &str) -> u32 {
    unsafe { part1_inner(input.as_bytes()) }
}

pub fn part2(input: &str) -> u32 {
    unsafe { part2_inner(input.as_bytes()) }
}

#[test]
fn test_part1_input() {
    assert_eq!(part1(include_str!("../input/day12.txt")), 1_374_934);
}

#[test]
fn test_part2_input() {
    assert_eq!(part2(include_str!("../input/day12.txt")), 841_078);
}