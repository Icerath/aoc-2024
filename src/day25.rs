pub fn part1(input: &str) -> u32 {
    let mut input = input.as_bytes();
    let mut locks = vec![];
    let mut keys = vec![];
    loop {
        let mut bits = 0;
        for i in 0..30 {
            bits |= ((input[i + 6] == b'#') as u32) << i;
        }
        match input[0] {
            b'#' => locks.push(bits),
            _ => keys.push(bits),
        }
        input = match input.get(43..) {
            Some(input) => input,
            None => break,
        };
    }
    let mut num_matching = 0;
    for lock in &locks {
        for key in &keys {
            num_matching += (lock & key == 0) as u32;
        }
    }
    num_matching
}

pub const PART1_OUT: u32 = 3255;
