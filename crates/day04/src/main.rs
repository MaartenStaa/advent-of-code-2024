fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part1(input));
}

#[derive(Debug)]
enum CheckDirection {
    Up,
    Down,
    Left,
    Right,
    DiagonalUpLeft,
    DiagonalUpRight,
    DiagonalDownLeft,
    DiagonalDownRight,
}

fn part1(input: &str) -> usize {
    let mut words = 0;
    let chars: Vec<_> = input.chars().collect();

    for (y, line) in chars.split(|&c| c == '\n').enumerate() {
        let width = line.len() + 1; // Add 1 to account for the newline character

        for (x, c) in line.iter().enumerate() {
            if *c == 'X' {
                words += check_word(&chars, x, y, width, CheckDirection::Up);
                words += check_word(&chars, x, y, width, CheckDirection::Down);
                words += check_word(&chars, x, y, width, CheckDirection::Left);
                words += check_word(&chars, x, y, width, CheckDirection::Right);
                words += check_word(&chars, x, y, width, CheckDirection::DiagonalUpLeft);
                words += check_word(&chars, x, y, width, CheckDirection::DiagonalUpRight);
                words += check_word(&chars, x, y, width, CheckDirection::DiagonalDownLeft);
                words += check_word(&chars, x, y, width, CheckDirection::DiagonalDownRight);
            }
        }
    }

    words
}

fn check_word(
    input: &[char],
    x: usize,
    y: usize,
    width: usize,
    direction: CheckDirection,
) -> usize {
    let height = input.len() / width;

    if match direction {
        CheckDirection::Up if y >= 3 => {
            input[(y - 1) * width + x] == 'M'
                && input[(y - 2) * width + x] == 'A'
                && input[(y - 3) * width + x] == 'S'
        }
        CheckDirection::Down if y < height - 3 => {
            input[(y + 1) * width + x] == 'M'
                && input[(y + 2) * width + x] == 'A'
                && input[(y + 3) * width + x] == 'S'
        }
        CheckDirection::Left if x >= 3 => {
            input[y * width + x - 1] == 'M'
                && input[y * width + x - 2] == 'A'
                && input[y * width + x - 3] == 'S'
        }
        CheckDirection::Right if x < width - 3 => {
            input[y * width + x + 1] == 'M'
                && input[y * width + x + 2] == 'A'
                && input[y * width + x + 3] == 'S'
        }
        CheckDirection::DiagonalUpLeft if y >= 3 && x >= 3 => {
            input[(y - 1) * width + x - 1] == 'M'
                && input[(y - 2) * width + x - 2] == 'A'
                && input[(y - 3) * width + x - 3] == 'S'
        }
        CheckDirection::DiagonalUpRight if y >= 3 && x < width - 3 => {
            input[(y - 1) * width + x + 1] == 'M'
                && input[(y - 2) * width + x + 2] == 'A'
                && input[(y - 3) * width + x + 3] == 'S'
        }
        CheckDirection::DiagonalDownLeft if y < height - 3 && x >= 3 => {
            input[(y + 1) * width + x - 1] == 'M'
                && input[(y + 2) * width + x - 2] == 'A'
                && input[(y + 3) * width + x - 3] == 'S'
        }
        CheckDirection::DiagonalDownRight if y < height - 3 && x < width - 3 => {
            input[(y + 1) * width + x + 1] == 'M'
                && input[(y + 2) * width + x + 2] == 'A'
                && input[(y + 3) * width + x + 3] == 'S'
        }
        _ => false,
    } {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_day4_part1() {
        assert_eq!(part1(TEST_INPUT), 18);
    }
}
