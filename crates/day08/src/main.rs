use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part1(input));
}

#[derive(Debug)]
struct Antenna {
    char: char,
    point: Point,
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

fn part1(input: &str) -> usize {
    let grid_width = input.lines().next().unwrap().len();
    let grid_height = input.lines().count();

    let antennas: Vec<_> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, char)| {
                if char == '.' {
                    None
                } else {
                    Some(Antenna {
                        char,
                        point: Point { x, y },
                    })
                }
            })
        })
        .collect();
    let mut antenna_map: HashMap<_, Vec<_>> = HashMap::new();
    for antenna in antennas {
        antenna_map
            .entry(antenna.char)
            .or_insert_with(Vec::new)
            .push(antenna.point);
    }

    let antinode_points: HashSet<_> = antenna_map
        .values()
        .filter(|points| points.len() > 1)
        .flat_map(|points| {
            points.iter().enumerate().flat_map(|(i, point_a)| {
                points.iter().skip(i + 1).flat_map(move |point_b| {
                    let mut antinode_points = Vec::new();
                    let x_diff = point_b.x as isize - point_a.x as isize;
                    let y_diff = point_b.y as isize - point_a.y as isize;

                    let antinode1_x = point_a.x as isize - x_diff;
                    let antinode1_y = point_a.y as isize - y_diff;
                    if antinode1_x >= 0
                        && antinode1_y >= 0
                        && antinode1_x < grid_width as isize
                        && antinode1_y < grid_height as isize
                    {
                        antinode_points.push(Point {
                            x: antinode1_x as usize,
                            y: antinode1_y as usize,
                        });
                    }

                    let antinode2_x = point_b.x as isize + x_diff;
                    let antinode2_y = point_b.y as isize + y_diff;
                    if antinode2_x >= 0
                        && antinode2_y >= 0
                        && antinode2_x < grid_width as isize
                        && antinode2_y < grid_height as isize
                    {
                        antinode_points.push(Point {
                            x: antinode2_x as usize,
                            y: antinode2_y as usize,
                        });
                    }

                    antinode_points.into_iter()
                })
            })
        })
        .collect();

    antinode_points.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_day8_part1() {
        assert_eq!(part1(TEST_INPUT), 14);
    }
}
