use std::{cmp::Reverse, collections::BinaryHeap};

const INPUT_SIZE: usize = 19999;

pub fn part1(input: &str) -> usize {
    unsafe { part1_inner(input.trim().as_bytes()) }
}

unsafe fn part1_inner(input: &[u8]) -> usize {
    let mut disk = [[0u32; 2]; INPUT_SIZE];
    let mut pos = 0;
    std::hint::assert_unchecked(input.len() == INPUT_SIZE);
    for (i, &b) in input.iter().enumerate() {
        let len = (b - b'0') as u32;
        *disk.get_unchecked_mut(i) = [pos, len];
        pos += len;
    }
    let mut result = 0;
    let mut front = 0;
    let mut back = INPUT_SIZE - 1;
    while front <= back {
        let [file_pos, file_len] = *disk.get_unchecked(front);
        let [file_pos, file_len] = [file_pos as usize, file_len as usize];

        result += (0..file_len).sum::<usize>() * front + (file_pos * front) * file_len;

        let [file_pos, file_len] = *disk.get_unchecked(front + 1);
        let [file_pos, file_len] = [file_pos as usize, file_len as usize];

        for k in 0..file_len {
            result += (file_pos + k) * back;
            disk.get_unchecked_mut(back)[1] -= 1;
            if disk.get_unchecked_mut(back)[1] == 0 {
                back -= 2;
            }
        }

        front += 2;
    }

    result / 2
}

#[test]
#[ignore]
fn test_part1_example() {
    assert_eq!(part1(include_str!("../input/day9_part1_example")), 1928);
}

#[test]
fn test_part1_input() {
    assert_eq!(part1(include_str!("../input/day9_part1")), 6_386_640_365_805);
}

pub fn part2(input: &str) -> usize {
    unsafe { part2_inner(input.as_bytes()) }
}

unsafe fn part2_inner(input: &[u8]) -> usize {
    let mut disk = [[0u32; 2]; INPUT_SIZE];
    let mut free_space = [const { BinaryHeap::new() }; 10];

    let mut front = 0;
    let mut pos = 0;
    loop {
        let len = (*input.get_unchecked(front) - b'0') as u32;
        disk[front] = [pos, len];
        pos += len;
        front += 1;
        if front == INPUT_SIZE {
            break;
        }
        let len = (*input.get_unchecked(front) - b'0') as u32;
        disk[front] = [pos, len];
        if len > 0 {
            free_space.get_unchecked_mut(len as usize).push(Reverse(pos as usize));
        }
        front += 1;
        pos += len;
    }

    let mut result = 0;
    let mut back = INPUT_SIZE - 1;
    while back > 0 {
        let [file_pos, file_len] = *disk.get_unchecked(back);
        let [file_pos, file_len] = [file_pos as usize, file_len as usize];

        let mut space_len = 0;
        let mut front = usize::MAX;
        let mut k = file_len;
        loop {
            let Some(&Reverse(space)) = free_space.get_unchecked(k).peek() else { continue };
            if space < front {
                front = space;
                space_len = k;
            }
            if k == 9 {
                break;
            }
            k += 1;
        }
        let pos = if front >= file_pos {
            file_pos
        } else {
            free_space.get_unchecked_mut(space_len).pop();
            let new_len = space_len - file_len;
            if new_len > 0 {
                free_space.get_unchecked_mut(new_len).push(Reverse(front + file_len));
            }
            front
        };

        result += (0..file_len).sum::<usize>() * back + (pos * back) * file_len;
        back -= 2;
    }
    result / 2
}

#[test]
#[ignore]
fn test_part2_example() {
    assert_eq!(part2(include_str!("../input/day9_part1_example")), 2858);
}

#[test]
fn test_part2_input() {
    assert_eq!(part2(include_str!("../input/day9_part1")), 6_423_258_376_982);
}
