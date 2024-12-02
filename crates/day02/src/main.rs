fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut safe = 0;

    for report in input.lines() {
        if !is_report_safe(
            report
                .split_ascii_whitespace()
                .map(|x| x.parse::<isize>().unwrap()),
        ) {
            continue;
        }

        safe += 1;
    }

    safe
}

fn part2(input: &str) -> usize {
    let mut safe = 0;

    for report in input.lines() {
        // First check if the report is safe as it is
        let levels = report
            .split_ascii_whitespace()
            .map(|x| x.parse::<isize>().unwrap())
            .collect::<Vec<_>>();
        if is_report_safe(levels.iter().copied()) {
            safe += 1;
            continue;
        }

        // Try with the problem dampener, allowing one value to be removed
        for i in 0..levels.len() {
            let mut levels = levels.clone();
            levels.remove(i);
            if is_report_safe(levels.iter().copied()) {
                safe += 1;
                break;
            }
        }
    }

    safe
}

fn is_report_safe(levels: impl Iterator<Item = isize>) -> bool {
    #[derive(Debug)]
    enum ReportState {
        New,
        Started(isize),
        Increasing(isize),
        Decreasing(isize),
    }

    const SAFE_DIFF_RANGE: std::ops::RangeInclusive<isize> = 1..=3;

    let mut report_state = ReportState::New;

    for level in levels {
        match report_state {
            ReportState::New => {
                report_state = ReportState::Started(level);
            }
            ReportState::Started(prev) => {
                let diff = (level - prev).abs();
                if !SAFE_DIFF_RANGE.contains(&diff) {
                    return false;
                }

                report_state = if level > prev {
                    ReportState::Increasing(level)
                } else {
                    ReportState::Decreasing(level)
                };
            }
            ReportState::Increasing(prev) => {
                if level < prev {
                    return false;
                }
                let diff = (level - prev).abs();
                if !SAFE_DIFF_RANGE.contains(&diff) {
                    return false;
                }

                report_state = ReportState::Increasing(level);
            }
            ReportState::Decreasing(prev) => {
                if level > prev {
                    return false;
                }
                let diff = (level - prev).abs();
                if !SAFE_DIFF_RANGE.contains(&diff) {
                    return false;
                }

                report_state = ReportState::Decreasing(level);
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 4);
    }
}
