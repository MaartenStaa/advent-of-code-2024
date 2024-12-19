use cached::proc_macro::cached;
use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    let (patterns, desired_designs) = parse(input);

    println!("Part 1: {}", part1(&patterns, &desired_designs));
    println!("Part 2: {}", part2(&patterns, &desired_designs));
}

fn parse(input: &str) -> (Vec<String>, Vec<String>) {
    let (patterns_str, desired_designs_str) = input.split_once("\n\n").unwrap();

    let patterns = patterns_str.split(", ").map(|s| s.to_string()).collect();
    let desired_designs = desired_designs_str.lines().map(|s| s.to_string()).collect();

    (patterns, desired_designs)
}

fn part1(patterns: &[String], desired_designs: &[String]) -> usize {
    let mut patterns_regex = "^(".to_string();
    for (i, pattern) in patterns.iter().enumerate() {
        if i != 0 {
            patterns_regex.push('|');
        }
        patterns_regex.push_str(pattern);
    }
    patterns_regex.push_str(")*$");

    let patterns_regex = Regex::new(&patterns_regex).unwrap();

    desired_designs
        .iter()
        .filter(|design| patterns_regex.is_match(design))
        .count()
}

fn part2(patterns: &[String], desired_designs: &[String]) -> usize {
    desired_designs
        .iter()
        .map(|design| num_possible_combinations(design, patterns))
        .sum()
}

#[cached(
    key = "(String, Vec<String>)",
    convert = r#"{ (design.to_string(), patterns.to_vec()) }"#
)]
fn num_possible_combinations(design: &str, patterns: &[String]) -> usize {
    if design.is_empty() {
        return 1;
    }

    patterns
        .iter()
        .filter(|pattern| design.starts_with(pattern.as_str()))
        .map(|pattern| num_possible_combinations(&design[pattern.len()..], patterns))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn test_day19_part1() {
        let (pat, des) = parse(TEST_INPUT);
        assert_eq!(part1(&pat, &des), 6);
    }

    #[test]
    fn test_day19_part2() {
        let (pat, des) = parse(TEST_INPUT);
        assert_eq!(part2(&pat, &des), 16);
    }
}
