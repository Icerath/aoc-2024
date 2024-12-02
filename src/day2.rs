pub fn part1(input: &str) -> u32 {
    let mut safe_reports = 0;
    for report in input.trim().lines() {
        safe_reports += 1;
        let mut prev_level = None;
        let mut increasing = None;
        for level in report.split_whitespace() {
            let level = level.parse::<u32>().unwrap();
            if let Some(prev_level) = prev_level {
                if !(1..=3).contains(&level.abs_diff(prev_level)) {
                    safe_reports -= 1;
                    break;
                }
                match increasing {
                    None => increasing = Some(level.cmp(&prev_level)),
                    Some(increasing) if increasing == level.cmp(&prev_level) => {}
                    _ => {
                        safe_reports -= 1;
                        break;
                    }
                }
            }
            prev_level = Some(level);
        }
    }
    safe_reports
}

#[test]
fn test_part1_example() {
    let input = include_str!("../input/day2_part1_example");
    assert_eq!(part1(input), 2)
}

#[test]
fn test_part1_input() {
    let input = include_str!("../input/day2_part1");
    assert_eq!(part1(input), 686)
}

pub fn part2(input: &str) -> u32 {
    let mut safe_reports = 0;

    for report in input.trim().lines() {
        let safe = part2_impl(report);
        safe_reports += safe as u32;
    }
    safe_reports
}

fn part2_impl(report: &str) -> bool {
    let levels: Vec<u32> = report.split_whitespace().map(|v| v.parse().unwrap()).collect();

    for try_skip in 0..=levels.len() {
        let mut prev_level = None;
        let mut safe = true;
        let mut increasing = None;

        for (i, &level) in levels.iter().enumerate() {
            if (i + 1) == try_skip {
                continue;
            }
            if let Some(prev_level) = prev_level {
                if !(1..=3).contains(&level.abs_diff(prev_level)) {
                    safe = false;
                    break;
                }
                match increasing {
                    None => increasing = Some(level.cmp(&prev_level)),
                    Some(increasing) if increasing == level.cmp(&prev_level) => {}
                    _ => {
                        safe = false;
                        break;
                    }
                }
            }
            prev_level = Some(level);
        }
        if safe {
            return true;
        }
    }
    false
}

#[test]
fn test_part2_example() {
    let input = include_str!("../input/day2_part1_example");
    assert_eq!(part2(input), 4)
}

#[test]
fn test_part2_input() {
    let input = include_str!("../input/day2_part1");
    assert_eq!(part2(input), 717)
}
