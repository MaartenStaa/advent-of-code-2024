use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn parse(input: &str) -> (Vec<u32>, usize, usize) {
    let mut width = 0;
    let mut height = 0;

    let grid: Vec<u32> = input
        .lines()
        .flat_map(|line| {
            width = line.len();
            height += 1;
            line.chars().map(|c| c.to_digit(10).unwrap())
        })
        .collect();

    (grid, width, height)
}

fn part1(input: &str) -> usize {
    let (grid, width, height) = parse(input);

    grid.iter()
        .enumerate()
        .filter(|(_, &v)| v == 0)
        .map(|(i, &v)| trail(v, i, &grid, width, height).len())
        .sum()
}

fn part2(input: &str) -> usize {
    let (grid, width, height) = parse(input);

    grid.iter()
        .enumerate()
        .filter(|(_, &v)| v == 0)
        .map(|(i, &v)| trail2(v, i, &grid, width, height))
        .sum()
}

fn trail(
    current_value: u32,
    i: usize,
    grid: &[u32],
    width: usize,
    height: usize,
) -> HashSet<usize> {
    let x = i % width;
    let y = i / width;

    eprintln!("trail {current_value} @ {x}x{y}");
    if current_value == 9 {
        return HashSet::from_iter([i]);
    }

    let next = current_value + 1;

    let mut terminal_positions = HashSet::new();

    // Check all four directions
    // Up
    if y > 0 && grid[i - width] == next {
        terminal_positions.extend(trail(next, i - width, grid, width, height));
    }
    // Right
    if x < width - 1 && grid[i + 1] == next {
        terminal_positions.extend(trail(next, i + 1, grid, width, height));
    }
    // Down
    if y < height - 1 && grid[i + width] == next {
        terminal_positions.extend(trail(next, i + width, grid, width, height));
    }
    // Left
    if x > 0 && grid[i - 1] == next {
        terminal_positions.extend(trail(next, i - 1, grid, width, height));
    }

    terminal_positions
}

fn trail2(current_value: u32, i: usize, grid: &[u32], width: usize, height: usize) -> usize {
    let x = i % width;
    let y = i / width;

    eprintln!("trail {current_value} @ {x}x{y}");
    if current_value == 9 {
        return 1;
    }

    let next = current_value + 1;

    let mut count = 0;

    // Check all four directions
    // Up
    if y > 0 && grid[i - width] == next {
        count += trail2(next, i - width, grid, width, height);
    }
    // Right
    if x < width - 1 && grid[i + 1] == next {
        count += trail2(next, i + 1, grid, width, height);
    }
    // Down
    if y < height - 1 && grid[i + width] == next {
        count += trail2(next, i + width, grid, width, height);
    }
    // Left
    if x > 0 && grid[i - 1] == next {
        count += trail2(next, i - 1, grid, width, height);
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_day10_part1() {
        assert_eq!(part1(TEST_INPUT), 36);
    }

    #[test]
    fn test_day10_part2() {
        assert_eq!(part2(TEST_INPUT), 81);
    }
}
