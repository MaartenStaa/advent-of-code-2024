fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part1(input));
}

fn part1(input: &str) -> usize {
    let mut rules: Vec<(usize, usize)> = vec![];

    for line in input.lines() {
        if line.is_empty() {
            break;
        }

        let (left, right) = line.split_once('|').unwrap();
        rules.push((left.parse().unwrap(), right.parse().unwrap()));
    }

    input
        .lines()
        .skip(rules.len() + 1)
        .filter_map(|line| {
            let numbers: Vec<usize> = line
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect::<Vec<_>>();

            for (left, right) in &rules {
                let left_index = numbers.iter().position(|&n| n == *left);
                let right_index = numbers.iter().position(|&n| n == *right);

                if let (Some(left_index), Some(right_index)) = (left_index, right_index) {
                    if left_index > right_index {
                        return None;
                    }
                }
            }

            // All rules match, get the middle number
            Some(numbers[numbers.len() / 2])
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_day5_part1() {
        assert_eq!(part1(TEST_INPUT), 143);
    }
}
