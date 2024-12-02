use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut a = Vec::with_capacity(input.lines().count());
    let mut b = Vec::with_capacity(a.capacity());

    for line in input.lines() {
        let mut parts = line.split_ascii_whitespace();
        let Some(left) = parts.next() else {
            panic!("Invalid input: {}", line);
        };
        let Some(right) = parts.next() else {
            panic!("Invalid input: {}", line);
        };

        let left: usize = left.parse().expect("Invalid input");
        let right: usize = right.parse().expect("Invalid input");

        a.push(left);
        b.push(right);
    }

    (a, b)
}

fn part1(input: &str) -> usize {
    let (mut a, mut b) = parse_input(input);

    a.sort();
    b.sort();

    let mut sum = 0;
    for (left, right) in a.iter().zip(b.iter()) {
        if left > right {
            sum += left - right;
        } else {
            sum += right - left;
        }
    }

    sum
}

fn part2(input: &str) -> usize {
    let (a, b) = parse_input(input);

    let b = {
        let mut map = HashMap::new();
        for &value in b.iter() {
            *map.entry(value).or_insert(0) += 1;
        }
        map
    };

    let mut similarity = 0;
    for value in a {
        similarity += value * b.get(&value).copied().unwrap_or(0);
    }

    similarity
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 31);
    }
}
