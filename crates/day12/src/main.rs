use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part1(input));
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Side {
    Top,
    Right,
    Bottom,
    Left,
}

fn part1(input: &str) -> usize {
    let mut width = 0;
    let mut height = 0;
    let grid: Vec<char> = input
        .lines()
        .flat_map(|line| {
            width = line.len();
            height += 1;
            line.chars()
        })
        .collect();

    let mut region_names = HashMap::new();
    let mut regions = HashMap::new();
    let mut region_perimeters = HashMap::new();
    let mut cell_regions = HashMap::new();
    let mut last_region_id = 0;
    for (i, &cell) in grid.iter().enumerate() {
        let x = i % width;
        let y = i / width;

        let is_new_region = !cell_regions.contains_key(&(x, y));
        if !is_new_region {
            continue;
        }

        last_region_id += 1;

        region_names.insert(last_region_id, cell);
        regions.insert(last_region_id, HashSet::new());
        region_perimeters.insert(last_region_id, HashSet::new());

        discover_region(
            &mut cell_regions,
            &mut regions,
            &mut region_perimeters,
            &grid,
            x,
            y,
            width,
            height,
            last_region_id,
        );
    }

    eprintln!();

    (1..=last_region_id)
        .map(|i| {
            let area = regions.get(&i).unwrap().len();
            let perimeter = region_perimeters.get(&i).unwrap().len();
            let name = region_names.get(&i).unwrap();

            eprintln!(
                "Region {name}: area {area}, perimeter: {perimeter} = {}",
                area * perimeter
            );

            area * perimeter
        })
        .sum()
}

#[allow(clippy::too_many_arguments)]
fn discover_region(
    cell_regions: &mut HashMap<(usize, usize), usize>,
    regions: &mut HashMap<usize, HashSet<(usize, usize)>>,
    region_perimeters: &mut HashMap<usize, HashSet<(usize, usize, Side)>>,
    grid: &[char],
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    region_id: usize,
) {
    let mut stack = vec![(x, y)];
    while let Some((x, y)) = stack.pop() {
        if x >= width || y >= height {
            continue;
        }

        if cell_regions.contains_key(&(x, y)) {
            continue;
        }

        let i = y * width + x;

        cell_regions.insert((x, y), region_id);
        regions.get_mut(&region_id).unwrap().insert((x, y));

        if x == 0 || grid[i - 1] != grid[i] {
            region_perimeters
                .get_mut(&region_id)
                .unwrap()
                .insert((x, y, Side::Left));
        }
        if y == 0 || grid[i - width] != grid[i] {
            region_perimeters
                .get_mut(&region_id)
                .unwrap()
                .insert((x, y, Side::Top));
        }
        if x == width - 1 || grid[i + 1] != grid[i] {
            region_perimeters
                .get_mut(&region_id)
                .unwrap()
                .insert((x, y, Side::Right));
        }
        if y == height - 1 || grid[i + width] != grid[i] {
            region_perimeters
                .get_mut(&region_id)
                .unwrap()
                .insert((x, y, Side::Bottom));
        }

        if x < width - 1 && grid[i + 1] == grid[i] {
            stack.push((x + 1, y));
        }
        if y < height - 1 && grid[i + width] == grid[i] {
            stack.push((x, y + 1));
        }
        if x > 0 && grid[i - 1] == grid[i] {
            stack.push((x - 1, y));
        }
        if y > 0 && grid[i - width] == grid[i] {
            stack.push((x, y - 1));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = "AAAA
BBCD
BBCC
EEEC";
    const TEST_INPUT_2: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
    const TEST_INPUT_3: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn test_day12_part1() {
        assert_eq!(part1(TEST_INPUT_1), 140);
        assert_eq!(part1(TEST_INPUT_2), 772);
        assert_eq!(part1(TEST_INPUT_3), 1930);
    }
}
