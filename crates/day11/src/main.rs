use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let input = parse(input);

    println!("Part 1: {}", run_simulation(input.clone(), 25));
    println!("Part 2: {}", run_simulation(input.clone(), 75));
}

fn parse(input: &str) -> Vec<usize> {
    input
        .split_ascii_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn run_simulation(numbers: Vec<usize>, n: usize) -> usize {
    // Turn into a map
    let mut numbers = {
        let mut map = HashMap::new();
        for n in numbers {
            *map.entry(n).or_insert(0) += 1;
        }
        map
    };

    for _ in 0..n {
        blink(&mut numbers);
    }

    numbers.values().sum()
}

fn blink(numbers: &mut HashMap<usize, usize>) {
    for (n, count) in numbers.iter().map(|(&k, &v)| (k, v)).collect::<Vec<_>>() {
        let entry = numbers.entry(n).or_insert(0);
        if *entry == count {
            numbers.remove(&n);
        } else {
            *entry -= count;
        }

        if n == 0 {
            *numbers.entry(1).or_insert(0) += count;
            continue;
        }

        let num_digits = n.checked_ilog10().unwrap_or(0) + 1;
        if num_digits % 2 == 0 {
            let pow = 10usize.pow(num_digits / 2);
            let left = n / pow;
            let right = n % pow;

            *numbers.entry(left).or_insert(0) += count;
            *numbers.entry(right).or_insert(0) += count;
        } else {
            *numbers.entry(n * 2024).or_insert(0) += count;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "125 17";

    #[test]
    fn test_day11_part1() {
        assert_eq!(run_simulation(parse(TEST_INPUT), 25), 55312);
    }

    #[test]
    fn edgecase() {
        let mut map = HashMap::new();
        map.insert(2, 2);
        map.insert(4048, 1);

        blink(&mut map);

        assert_eq!(map, [(4048, 2), (40, 1), (48, 1)].into_iter().collect());
    }
}
