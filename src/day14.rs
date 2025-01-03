#![expect(clippy::cast_possible_truncation)]

macro_rules! parse {
    ($ptr: ident) => {{
        $ptr = $ptr.add(2);
        let mut px = $ptr.read() - b'0';
        $ptr = $ptr.add(1);
        while $ptr.read() != b',' {
            px = px * 10 + ($ptr.read() - b'0');
            $ptr = $ptr.add(1);
        }
        $ptr = $ptr.add(1);
        let mut py = $ptr.read() - b'0';
        $ptr = $ptr.add(1);
        while $ptr.read() != b' ' {
            py = py * 10 + ($ptr.read() - b'0');
            $ptr = $ptr.add(1);
        }
        $ptr = $ptr.add(3);

        let vx_neg = if $ptr.read() == b'-' {
            $ptr = $ptr.add(1);
            true
        } else {
            false
        };
        let mut vx = ($ptr.read() - b'0');
        $ptr = $ptr.add(1);
        while $ptr.read() != b',' {
            vx = vx * 10 + ($ptr.read() - b'0');
            $ptr = $ptr.add(1);
        }
        $ptr = $ptr.add(1);

        let vy_neg = if $ptr.read() == b'-' {
            $ptr = $ptr.add(1);
            true
        } else {
            false
        };
        let mut vy = ($ptr.read() - b'0');
        $ptr = $ptr.add(1);
        while $ptr.read() != b'\n' {
            vy = vy * 10 + ($ptr.read() - b'0');
            $ptr = $ptr.add(1);
        }
        let vx = if vx_neg { WIDTH as u8 - vx } else { vx };
        let vy = if vy_neg { HEIGHT as u8 - vy } else { vy };
        $ptr = $ptr.add(1);
        [px, py, vx, vy]
    }};
}

pub fn part1(input: &str) -> u32 {
    let mut remaining = input.as_ptr();
    let mut quadrants = [0u32; 4];
    for _ in 0..500 {
        let [px, py, vx, vy] = unsafe { parse!(remaining) };
        let [px, py, vx, vy] = [px as u16, py as u16, vx as u16, vy as u16];
        let px = (px + vx * 100) % WIDTH as u16;
        let py = (py + vy * 100) % HEIGHT as u16;

        #[expect(non_contiguous_range_endpoints)]
        match (px, py) {
            (..50, ..51) => quadrants[0] += 1,
            (..50, 52..) => quadrants[1] += 1,
            (51.., ..51) => quadrants[2] += 1,
            (51.., 52..) => quadrants[3] += 1,
            _ => {}
        }
    }
    quadrants.into_iter().product()
}

#[expect(clippy::similar_names)]
pub fn part2(input: &str) -> i32 {
    let mut remaining = input.as_ptr();
    let mut pxs = [0u8; 500];
    let mut vxs = [0u8; 500];
    let mut pys = [0u8; 500];
    let mut vys = [0u8; 500];

    for i in 0..500 {
        let [px, py, vx, vy] = unsafe { parse!(remaining) };
        pxs[i] = px;
        vxs[i] = vx;
        pys[i] = py;
        vys[i] = vy;
    }

    let mut x_min_seconds = 0;
    let mut x_min_value = u16::MAX;

    for seconds in 0..WIDTH as u16 {
        let mut x_sum = 0;
        for i in 0..500 {
            let px = pxs[i] as u16;
            let vx = vxs[i] as u16;
            x_sum += ((px + (vx * seconds)) % WIDTH as u16).abs_diff(WIDTH as u16 / 2);
        }
        if x_sum < x_min_value {
            x_min_value = x_sum;
            x_min_seconds = seconds;
        }
    }

    let mut y_min_seconds = 0;
    let mut y_min_value = u16::MAX;

    for seconds in 0..HEIGHT as u16 {
        let mut y_sum = 0;
        for i in 0..500 {
            let py = pys[i] as u16;
            let vy = vys[i] as u16;
            y_sum += ((py + (vy * seconds)) % HEIGHT as u16).abs_diff(HEIGHT as u16 / 2);
        }
        if y_sum < y_min_value {
            y_min_value = y_sum;
            y_min_seconds = seconds;
        }
    }
    ((x_min_seconds as i32 * mod_inv(HEIGHT, WIDTH) * HEIGHT)
        + (y_min_seconds as i32 * mod_inv(WIDTH, HEIGHT) * WIDTH))
        % (HEIGHT * WIDTH)
}

// https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
const fn egcd(a: i32, b: i32) -> i32 {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1, 0);

    while r != 0 {
        let quotient = old_r / r;
        (old_r, r) = (r, old_r - quotient * r);
        (old_s, s) = (s, old_s - quotient * s);
    }
    old_s
}

const fn mod_inv(x: i32, n: i32) -> i32 {
    egcd(x, n).rem_euclid(n)
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../input/day14.txt")), 220_971_520);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("../input/day14.txt")), 6355);
}

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;
