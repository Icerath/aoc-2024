pub fn part1(input: &str) -> u32 {
    let mut input = input.as_bytes();
    let mut locks = vec![];
    let mut keys = vec![];
    loop {
        let counts: [u8; 5] = std::array::from_fn(|i| {
            (6..36).step_by(6).map(|offset| (input[i + offset] == b'#') as u8).sum::<u8>()
        });
        match input[0] {
            b'#' => locks.push(counts.map(|i| 5 - i)),
            _ => keys.push(counts),
        }
        input = match input.get(43..) {
            Some(input) => input,
            None => break,
        };
    }
    let mut num_matching = 0;
    for lock in &locks {
        for key in &keys {
            num_matching += (std::iter::zip(lock, key).all(|(lock, key)| key <= lock)) as u32;
        }
    }
    num_matching
}

pub const PART1_OUT: u32 = 3255;
