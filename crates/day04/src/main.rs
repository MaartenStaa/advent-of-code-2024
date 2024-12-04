fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
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

        for (x, &c) in line.iter().enumerate() {
            if c == 'X' {
                words += check_word(&chars, x, y, width, "MAS", CheckDirection::Up);
                words += check_word(&chars, x, y, width, "MAS", CheckDirection::Down);
                words += check_word(&chars, x, y, width, "MAS", CheckDirection::Left);
                words += check_word(&chars, x, y, width, "MAS", CheckDirection::Right);
                words += check_word(&chars, x, y, width, "MAS", CheckDirection::DiagonalUpLeft);
                words += check_word(&chars, x, y, width, "MAS", CheckDirection::DiagonalUpRight);
                words += check_word(&chars, x, y, width, "MAS", CheckDirection::DiagonalDownLeft);
                words += check_word(
                    &chars,
                    x,
                    y,
                    width,
                    "MAS",
                    CheckDirection::DiagonalDownRight,
                );
            }
        }
    }

    words
}

fn part2(input: &str) -> usize {
    let mut words = 0;
    let chars: Vec<_> = input.chars().collect();

    for (y, line) in chars.split(|&c| c == '\n').enumerate() {
        let width = line.len() + 1; // Add 1 to account for the newline character

        for (x, &c) in line.iter().enumerate() {
            if c == 'M' {
                if check_word(&chars, x, y, width, "AS", CheckDirection::DiagonalUpRight) == 1
                    && line[x + 2] == 'M'
                    && check_word(
                        &chars,
                        x + 2,
                        y,
                        width,
                        "AS",
                        CheckDirection::DiagonalUpLeft,
                    ) == 1
                {
                    words += 1;
                }

                if check_word(&chars, x, y, width, "AS", CheckDirection::DiagonalDownRight) == 1 {
                    if line[x + 2] == 'M'
                        && check_word(
                            &chars,
                            x + 2,
                            y,
                            width,
                            "AS",
                            CheckDirection::DiagonalDownLeft,
                        ) == 1
                    {
                        words += 1;
                    }

                    if chars[(y + 2) * width + x] == 'M'
                        && check_word(
                            &chars,
                            x,
                            y + 2,
                            width,
                            "AS",
                            CheckDirection::DiagonalUpRight,
                        ) == 1
                    {
                        words += 1;
                    }
                }

                if check_word(&chars, x, y, width, "AS", CheckDirection::DiagonalDownLeft) == 1
                    && chars[(y + 2) * width + x] == 'M'
                    && check_word(
                        &chars,
                        x,
                        y + 2,
                        width,
                        "AS",
                        CheckDirection::DiagonalUpLeft,
                    ) == 1
                {
                    words += 1;
                }
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
    word: &str,
    direction: CheckDirection,
) -> usize {
    let height = input.len() / width;

    if match direction {
        CheckDirection::Up if y >= word.len() => word
            .chars()
            .enumerate()
            .all(|(i, c)| input[(y - i - 1) * width + x] == c),
        CheckDirection::Down if y < height - word.len() => word
            .chars()
            .enumerate()
            .all(|(i, c)| input[(y + i + 1) * width + x] == c),
        CheckDirection::Left if x >= word.len() => word
            .chars()
            .enumerate()
            .all(|(i, c)| input[y * width + x - i - 1] == c),
        CheckDirection::Right if x < width - word.len() => word
            .chars()
            .enumerate()
            .all(|(i, c)| input[y * width + x + i + 1] == c),
        CheckDirection::DiagonalUpLeft if y >= word.len() && x >= word.len() => word
            .chars()
            .enumerate()
            .all(|(i, c)| input[(y - i - 1) * width + x - i - 1] == c),
        CheckDirection::DiagonalUpRight if y >= word.len() && x < width - word.len() => word
            .chars()
            .enumerate()
            .all(|(i, c)| input[(y - i - 1) * width + x + i + 1] == c),
        CheckDirection::DiagonalDownLeft if y < height - word.len() && x >= word.len() => word
            .chars()
            .enumerate()
            .all(|(i, c)| input[(y + i + 1) * width + x - i - 1] == c),
        CheckDirection::DiagonalDownRight if y < height - word.len() && x < width - word.len() => {
            word.chars()
                .enumerate()
                .all(|(i, c)| input[(y + i + 1) * width + x + i + 1] == c)
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

    #[test]
    fn test_day4_part2() {
        assert_eq!(part2(TEST_INPUT), 9);
    }
}
