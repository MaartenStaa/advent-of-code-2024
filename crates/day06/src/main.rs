use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part1(input));
}

#[derive(Debug)]
struct Field {
    grid: Vec<bool>,
    guard: Guard,
}

#[derive(Debug, Clone)]
struct Guard {
    x: isize,
    y: isize,
    direction: Direction,
}

impl Guard {
    fn has_block_in_front(&self, grid: &[bool], width: usize, height: usize) -> bool {
        match self.direction {
            Direction::North if self.y == 0 => false,
            Direction::North => grid[(self.y as usize - 1) * width + self.x as usize],
            Direction::East if self.x as usize == width - 1 => false,
            Direction::East => grid[self.y as usize * width + self.x as usize + 1],
            Direction::South if self.y as usize == height - 1 => false,
            Direction::South => grid[(self.y as usize + 1) * width + self.x as usize],
            Direction::West if self.x as usize == 0 => false,
            Direction::West => grid[self.y as usize * width + self.x as usize - 1],
        }
    }

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
    }

    fn step_forward(&mut self) {
        match self.direction {
            Direction::North => self.y -= 1,
            Direction::East => self.x += 1,
            Direction::South => self.y += 1,
            Direction::West => self.x -= 1,
        }
    }
}

#[derive(Debug, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn part1(input: &str) -> usize {
    let Field { grid, mut guard } = parse_input(input);
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let mut unique_positions = HashSet::new();

    loop {
        unique_positions.insert((guard.x, guard.y));

        if guard.has_block_in_front(&grid, width, height) {
            guard.turn_right()
        } else {
            guard.step_forward()
        }

        if guard.y < 0 || guard.y as usize >= height || guard.x < 0 || guard.x as usize >= width {
            break;
        }
    }

    unique_positions.len()
}

fn parse_input(input: &str) -> Field {
    let mut grid = Vec::new();
    let mut guard = None;

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => grid.push(false),
                '#' => grid.push(true),
                '^' => {
                    guard = Some(Guard {
                        x: x as isize,
                        y: y as isize,
                        direction: Direction::North,
                    });
                    grid.push(false);
                }
                '>' => {
                    guard = Some(Guard {
                        x: x as isize,
                        y: y as isize,
                        direction: Direction::East,
                    });
                    grid.push(false);
                }
                'v' => {
                    guard = Some(Guard {
                        x: x as isize,
                        y: y as isize,
                        direction: Direction::South,
                    });
                    grid.push(false);
                }
                '<' => {
                    guard = Some(Guard {
                        x: x as isize,
                        y: y as isize,
                        direction: Direction::West,
                    });
                    grid.push(false);
                }
                _ => panic!("Invalid character: {}", c),
            }
        }
    }

    Field {
        grid,
        guard: guard.unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_day6_part1() {
        assert_eq!(part1(TEST_INPUT), 41);
    }
}
