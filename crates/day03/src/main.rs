fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part1(input));
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

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 161);
    }
}
