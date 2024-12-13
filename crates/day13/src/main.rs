use pathfinding::prelude::astar;
use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    let input = parse(input);

    println!("Part 1: {}", part1(&input));
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
struct Offset {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, PartialEq)]
struct Input {
    a: Offset,
    b: Offset,
    prize: Offset,
}

fn parse(input: &str) -> Vec<Input> {
    let button_re = Regex::new(r"Button [A|B]: X\+(\d+), Y\+(\d+)").unwrap();
    let prize_re = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    input
        .split("\n\n")
        .map(|section| {
            let mut a = None;
            let mut b = None;
            let mut prize = None;

            for line in section.lines() {
                if let Some(caps) = button_re.captures(line) {
                    let x = caps.get(1).unwrap().as_str().parse().unwrap();
                    let y = caps.get(2).unwrap().as_str().parse().unwrap();

                    if line.starts_with("Button A") {
                        a = Some(Offset { x, y });
                    } else {
                        b = Some(Offset { x, y });
                    }
                } else if let Some(caps) = prize_re.captures(line) {
                    let x = caps.get(1).unwrap().as_str().parse().unwrap();
                    let y = caps.get(2).unwrap().as_str().parse().unwrap();

                    prize = Some(Offset { x, y });
                }
            }

            Input {
                a: a.unwrap(),
                b: b.unwrap(),
                prize: prize.unwrap(),
            }
        })
        .collect()
}

const BUTTON_A_COST: usize = 3;
const BUTTON_B_COST: usize = 1;

fn solve(input: &Input) -> Option<usize> {
    astar(
        &Offset { x: 0, y: 0 },
        |&current| {
            let mut neighbors = vec![];

            for &(offset, cost) in &[(input.a, BUTTON_A_COST), (input.b, BUTTON_B_COST)] {
                let new_offset = Offset {
                    x: current.x + offset.x,
                    y: current.y + offset.y,
                };

                if new_offset.x <= input.prize.x && new_offset.y <= input.prize.y {
                    neighbors.push((new_offset, cost));
                }
            }

            neighbors
        },
        |&current| manhattan_distance(current, input.prize),
        |&current| current == input.prize,
    )
    .map(|(_, cost)| cost)
}

fn part1(input: &[Input]) -> usize {
    input.iter().filter_map(solve).sum()
}

fn manhattan_distance(start: Offset, goal: Offset) -> usize {
    ((goal.x as isize - start.x as isize).abs() + (goal.y as isize - start.y as isize).abs())
        as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_day13_parse() {
        assert_eq!(
            parse(TEST_INPUT),
            vec![
                Input {
                    a: Offset { x: 94, y: 34 },
                    b: Offset { x: 22, y: 67 },
                    prize: Offset { x: 8400, y: 5400 },
                },
                Input {
                    a: Offset { x: 26, y: 66 },
                    b: Offset { x: 67, y: 21 },
                    prize: Offset { x: 12748, y: 12176 },
                },
                Input {
                    a: Offset { x: 17, y: 86 },
                    b: Offset { x: 84, y: 37 },
                    prize: Offset { x: 7870, y: 6450 },
                },
                Input {
                    a: Offset { x: 69, y: 23 },
                    b: Offset { x: 27, y: 71 },
                    prize: Offset { x: 18641, y: 10279 },
                },
            ]
        );
    }

    #[test]
    fn test_day13_part1() {
        let input = parse(TEST_INPUT);

        assert_eq!(solve(&input[0]), Some(280));
        assert_eq!(solve(&input[1]), None);
        assert_eq!(solve(&input[2]), Some(200));
        assert_eq!(solve(&input[3]), None);
    }
}
