use pathfinding::prelude::astar;

fn main() {
    let input = include_str!("input.txt");
    let input = parse(input);

    println!("Part 1: {}", part1(&input));
}

#[derive(Debug)]
struct Maze {
    grid: Grid,
    start: Point,
    end: Point,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct PointAndDirection {
    x: usize,
    y: usize,
    direction: Direction,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Grid {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

#[derive(Debug)]
enum Cell {
    Wall,
    Empty,
}

fn parse(input: &str) -> Maze {
    let mut cells = Vec::new();
    let mut width = 0;
    let mut height = 0;
    let mut start = None;
    let mut end = None;

    for (y, line) in input.lines().enumerate() {
        height += 1;
        width = line.len();

        for (x, c) in line.chars().enumerate() {
            let cell = match c {
                '#' => Cell::Wall,
                '.' => Cell::Empty,
                'S' => {
                    start = Some(Point { x, y });
                    Cell::Empty
                }
                'E' => {
                    end = Some(Point { x, y });
                    Cell::Empty
                }
                _ => panic!("unexpected character: {}", c),
            };

            cells.push(cell);
        }
    }

    Maze {
        grid: Grid {
            cells,
            width,
            height,
        },
        start: start.expect("no start found"),
        end: end.expect("no end found"),
    }
}

fn part1(input: &Maze) -> usize {
    // Find the cheapest path

    let start = PointAndDirection {
        x: input.start.x,
        y: input.start.y,
        direction: Direction::Right,
    };

    astar(
        &start,
        |p| {
            let mut neighbors = Vec::new();

            let dx: isize = match p.direction {
                Direction::Left => -1,
                Direction::Right => 1,
                _ => 0,
            };
            let dy: isize = match p.direction {
                Direction::Up => -1,
                Direction::Down => 1,
                _ => 0,
            };

            let front_x = p.x as isize + dx;
            let front_y = p.y as isize + dy;

            if front_x >= 0
                && front_x < input.grid.width as isize
                && front_y >= 0
                && front_y < input.grid.height as isize
            {
                if let Cell::Empty =
                    input.grid.cells[front_y as usize * input.grid.width + front_x as usize]
                {
                    neighbors.push((
                        PointAndDirection {
                            x: front_x as usize,
                            y: front_y as usize,
                            direction: p.direction,
                        },
                        1,
                    ));
                }
            }

            // Try to turn left
            let left = match p.direction {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            };
            neighbors.push((
                PointAndDirection {
                    x: p.x,
                    y: p.y,
                    direction: left,
                },
                1000,
            ));

            // Try to turn right
            let right = match p.direction {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            };
            neighbors.push((
                PointAndDirection {
                    x: p.x,
                    y: p.y,
                    direction: right,
                },
                1000,
            ));

            neighbors
        },
        |p| manhattan_distance(&Point { x: p.x, y: p.y }, &input.end),
        |p| p.x == input.end.x && p.y == input.end.y,
    )
    .unwrap()
    .1
}

fn manhattan_distance(a: &Point, b: &Point) -> usize {
    ((a.x as isize - b.x as isize).abs() + (a.y as isize - b.y as isize).abs()) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day16_part1() {
        let input = parse(
            r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#,
        );

        assert_eq!(part1(&input), 7036);
    }
}