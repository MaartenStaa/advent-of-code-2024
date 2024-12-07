use itertools::Itertools;
use rayon::prelude::*;

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[derive(Debug)]
struct Equation {
    expected: usize,
    components: Vec<usize>,
}

#[derive(Copy, Debug, Clone)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

impl Equation {
    fn from_str(input: &str) -> Self {
        let (expected, components) = input.split_once(": ").unwrap();
        Self {
            expected: expected.parse().unwrap(),
            components: components
                .split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect(),
        }
    }

    fn solve(&self, allow_concatenation: bool) -> Option<Vec<Operator>> {
        'outer: for operators in (0..self.components.len())
            .map(|_| {
                if allow_concatenation {
                    vec![Operator::Add, Operator::Multiply, Operator::Concatenate]
                } else {
                    vec![Operator::Add, Operator::Multiply]
                }
            })
            .multi_cartesian_product()
        {
            let mut result = self.components[0];
            for (i, component) in self.components.iter().skip(1).enumerate() {
                match operators[i] {
                    Operator::Add => result += component,
                    Operator::Multiply => result *= component,
                    Operator::Concatenate => {
                        let component_digits = component.checked_ilog10().unwrap_or(0) + 1;
                        result = result * 10usize.pow(component_digits) + component;
                    }
                }

                if result > self.expected {
                    continue 'outer;
                }
            }

            if result == self.expected {
                return Some(operators);
            }
        }

        None
    }
}

fn part1(input: &str) -> usize {
    let equations: Vec<Equation> = input.lines().map(Equation::from_str).collect();
    equations
        .par_iter()
        .filter_map(|equation| equation.solve(false).map(|_| equation.expected))
        .sum()
}

fn part2(input: &str) -> usize {
    let equations: Vec<Equation> = input.lines().map(Equation::from_str).collect();
    equations
        .par_iter()
        .filter_map(|equation| equation.solve(true).map(|_| equation.expected))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_day7_part1() {
        assert_eq!(part1(TEST_INPUT), 3749);
    }

    #[test]
    fn test_day7_part2() {
        assert_eq!(part2(TEST_INPUT), 11387);
    }
}
