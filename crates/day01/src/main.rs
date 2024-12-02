fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part1(input));
}

fn part1(input: &str) -> usize {
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
}
