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
