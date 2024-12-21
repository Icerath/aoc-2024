macro_rules! test_day {
    ($day: ident) => {
        mod $day {
            use crate::$day;
            #[test]
            fn part1() {
                let _guard = super::LOCK.lock();
                assert_eq!($day::part1(include_str!("../input/day20.txt")), $day::PART1_OUT);
            }

            #[test]
            fn part2() {
                let _guard = super::LOCK.lock();
                assert_eq!($day::part2(include_str!("../input/day20.txt")), $day::PART2_OUT);
            }
        }
    };
}

test_day!(day20);

#[cfg(test)]
static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
