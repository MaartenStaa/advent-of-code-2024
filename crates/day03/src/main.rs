fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let regex = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut sum = 0;

    for cap in regex.captures_iter(input) {
        let x: usize = cap[1].parse().unwrap();
        let y: usize = cap[2].parse().unwrap();

        sum += x * y;
    }

    sum
}

fn part2(input: &str) -> usize {
    let regex = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mut sum = 0;
    let mut enabled = true;

    for cap in regex.captures_iter(input) {
        if cap[0].starts_with("don't") {
            enabled = false;
            continue;
        } else if cap[0].starts_with("do") {
            enabled = true;
            continue;
        }

        if !enabled {
            continue;
        }

        let x: usize = cap[1].parse().unwrap();
        let y: usize = cap[2].parse().unwrap();

        sum += x * y;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const TEST_INPUT2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_day3_part1() {
        assert_eq!(part1(TEST_INPUT1), 161);
    }

    #[test]
    fn test_day3_part2() {
        assert_eq!(part2(TEST_INPUT2), 48);
    }
}
