macro_rules! parse {
    ($remaining: ident) => {{
        $remaining = $remaining.add(2);
        let mut px = 0;
        while $remaining.read() != b',' {
            px = px * 10 + ($remaining.read() - b'0');
            $remaining = $remaining.add(1);
        }
        $remaining = $remaining.add(1);
        let mut py = 0;
        while $remaining.read() != b' ' {
            py = py * 10 + ($remaining.read() - b'0');
            $remaining = $remaining.add(1);
        }
        $remaining = $remaining.add(3);

        let vx_sign = if $remaining.read() == b'-' {
            $remaining = $remaining.add(1);
            -1
        } else {
            1
        };
        let mut vx = 0;
        while $remaining.read() != b',' {
            vx = vx * 10 + ($remaining.read() - b'0') as i32;
            $remaining = $remaining.add(1);
        }
        $remaining = $remaining.add(1);

        let vy_sign = if $remaining.read() == b'-' {
            $remaining = $remaining.add(1);
            -1
        } else {
            1
        };
        let mut vy = 0;
        while $remaining.read() != b'\n' {
            vy = vy * 10 + ($remaining.read() - b'0') as i32;
            $remaining = $remaining.add(1);
        }
        $remaining = $remaining.add(1);
        [px as i32, py as i32, vx * vx_sign, vy * vy_sign]
    }};
}

pub fn part1(input: &str) -> u32 {
    let mut remaining = input.as_ptr();
    let mut quadrants = [0u32; 4];
    for _ in 0..500 {
        let [px, py, vx, vy] = unsafe { parse!(remaining) };
        let px = (px + vx * 100).rem_euclid(101);
        let py = (py + vy * 100).rem_euclid(103);

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

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

pub fn part2(input: &str) -> i32 {
    let mut remaining = input.as_ptr();
    let mut robots = [[0i32; 4]; 500];
    for i in 0..500 {
        robots[i] = unsafe { parse!(remaining) };
    }

    let mut x_min_seconds = 0;
    let mut x_min_value = u32::MAX;
    for seconds in 0..WIDTH {
        let mut x_value = 0;
        for [px, _py, vx, _vy] in &robots {
            x_value += (px + (vx * seconds)).rem_euclid(WIDTH).abs_diff(WIDTH / 2);
        }
        if x_value < x_min_value {
            x_min_value = x_value;
            x_min_seconds = seconds;
        }
    }

    let mut y_min_seconds = 0;
    let mut y_min_value = u32::MAX;

    for seconds in 0..HEIGHT {
        let mut y_value = 0;
        for [_px, py, _vx, vy] in &robots {
            y_value += (py + (vy * seconds)).rem_euclid(HEIGHT).abs_diff(HEIGHT / 2);
        }
        if y_value < y_min_value {
            y_min_value = y_value;
            y_min_seconds = seconds;
        }
    }
    big_brain(x_min_seconds, y_min_seconds)
}

fn big_brain(x: i32, y: i32) -> i32 {
    const MOD_INV_X: i32 = mod_inv(HEIGHT, WIDTH) * HEIGHT; //5253
    const MOD_INV_Y: i32 = mod_inv(WIDTH, HEIGHT) * WIDTH; // 5151
    ((x * MOD_INV_X) + (y * MOD_INV_Y)) % (HEIGHT * WIDTH)
}

#[expect(clippy::many_single_char_names)]
// https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
const fn egcd(a: i32, b: i32) -> i32 {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);

    while r != 0 {
        let quotient = old_r / r;
        (old_r, r) = (r, old_r - quotient * r);
        (old_s, s) = (s, old_s - quotient * s);
        (old_t, t) = (t, old_t - quotient * t);
    }
    old_s
}

const fn mod_inv(x: i32, n: i32) -> i32 {
    (egcd(x, n) % n + n) % n
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../input/day14.txt")), 220_971_520);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("../input/day14.txt")), 6355);
}
