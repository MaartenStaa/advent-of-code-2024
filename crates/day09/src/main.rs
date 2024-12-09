fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part1(input));
}

fn part1(input: &str) -> usize {
    let mut blocks = Vec::with_capacity(input.len());
    let mut file_id = 0;
    let mut is_file = true;

    for c in input.chars() {
        if c == '\n' {
            continue;
        }
        let n = c.to_digit(10).unwrap() as usize;
        if is_file {
            blocks.extend(std::iter::repeat(Some(file_id)).take(n));

            file_id += 1;
            is_file = false;
        } else {
            blocks.extend(std::iter::repeat(None).take(n));

            is_file = true;
        }
    }

    let mut search_empty_from = 0;
    for i in (0..blocks.len()).rev() {
        if blocks[i].is_some() {
            // Find the first empty spot
            let empty_slot = match blocks
                .iter()
                .skip(search_empty_from)
                .position(|x| x.is_none())
            {
                Some(x) => x,
                None => break,
            } + search_empty_from;
            search_empty_from = empty_slot;

            blocks.swap_remove(empty_slot);
        }
    }

    blocks
        .iter()
        .enumerate()
        .map(|(i, x)| i * x.expect("All Nones should have been removed"))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2333133121414131402";

    #[test]
    fn test_day9_part1() {
        assert_eq!(part1(TEST_INPUT), 1928);
    }
}
