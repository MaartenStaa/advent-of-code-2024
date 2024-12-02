fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part1(input));
}

fn part1(input: &str) -> usize {
    let mut safe = 0;

    #[derive(Debug)]
    enum ReportState {
        New,
        Started(isize),
        Increasing(isize),
        Decreasing(isize),
    }

    const SAFE_DIFF_RANGE: std::ops::RangeInclusive<isize> = 1..=3;

    'reports: for report in input.lines() {
        let mut report_state = ReportState::New;

        for level in report
            .split_ascii_whitespace()
            .map(|x| x.parse::<isize>().unwrap())
        {
            match report_state {
                ReportState::New => {
                    report_state = ReportState::Started(level);
                }
                ReportState::Started(prev) => {
                    let diff = (level - prev).abs();
                    if !SAFE_DIFF_RANGE.contains(&diff) {
                        continue 'reports;
                    }

                    report_state = if level > prev {
                        ReportState::Increasing(level)
                    } else {
                        ReportState::Decreasing(level)
                    };
                }
                ReportState::Increasing(prev) => {
                    if level < prev {
                        continue 'reports;
                    }
                    let diff = (level - prev).abs();
                    if !SAFE_DIFF_RANGE.contains(&diff) {
                        continue 'reports;
                    }

                    report_state = ReportState::Increasing(level);
                }
                ReportState::Decreasing(prev) => {
                    if level > prev {
                        continue 'reports;
                    }
                    let diff = (level - prev).abs();
                    if !SAFE_DIFF_RANGE.contains(&diff) {
                        continue 'reports;
                    }

                    report_state = ReportState::Decreasing(level);
                }
            }
        }

        safe += 1;
    }

    safe
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
}
