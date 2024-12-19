use std::collections::HashMap;

pub fn part1(input: &str) -> u32 {
    let mut lines = input.lines();
    let available = lines.next().unwrap().split(", ").collect::<Vec<_>>();
    _ = lines.next();

    let mut sum = 0;
    for desired in lines {
        sum += is_solvable(&available, desired) as u32;
    }
    sum
}

pub fn part2(input: &str) -> u64 {
    let mut lines = input.lines();
    let available = lines.next().unwrap().split(", ").collect::<Vec<_>>();
    _ = lines.next();

    let mut sum = 0;
    let mut cache = HashMap::new();
    for desired in lines {
        cache.clear();
        sum += sum_solvable(&available, desired, &mut cache);
    }
    sum
}

fn sum_solvable<'a>(available_towels: &[&str], desired: &'a str, cache: &mut HashMap<&'a str, u64>) -> u64 {
    if desired.is_empty() {
        return 1;
    }
    if let Some(&res) = cache.get(desired) {
        return res;
    }

    let mut sum = 0;
    for available in available_towels {
        if let Some(next_desired) = desired.strip_prefix(available) {
            let res = sum_solvable(available_towels, next_desired, cache);
            cache.insert(next_desired, res);
            sum += res;
        }
    }
    sum
}

fn is_solvable(available_towels: &[&str], desired: &str) -> bool {
    if desired.is_empty() {
        return true;
    }

    for available in available_towels {
        if let Some(next_desired) = desired.strip_prefix(available) {
            if is_solvable(available_towels, next_desired) {
                return true;
            }
        }
    }
    false
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../input/day19.txt")), 258);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("../input/day19.txt")), 632_423_618_484_345);
}
