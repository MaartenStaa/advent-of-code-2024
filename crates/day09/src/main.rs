fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
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

    checksum(blocks)
}

fn part2(input: &str) -> usize {
    enum Block {
        File { id: usize, length: usize },
        Empty { length: usize },
    }

    let mut blocks = Vec::new();
    let mut file_id = 0;
    let mut is_file = true;

    for c in input.chars() {
        if c == '\n' {
            continue;
        }
        let n = c.to_digit(10).unwrap() as usize;
        if is_file {
            blocks.push(Block::File {
                id: file_id,
                length: n,
            });

            file_id += 1;
            is_file = false;
        } else {
            blocks.push(Block::Empty { length: n });

            is_file = true;
        }
    }

    for i in (0..blocks.len()).rev() {
        if let Block::File { length, .. } = &blocks[i] {
            let file_length = *length;
            let (empty_slot, empty_slot_length) =
                match blocks.iter().enumerate().find_map(|(i, block)| {
                    if let Block::Empty { length } = block {
                        if *length >= file_length {
                            Some((i, length))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }) {
                    Some(x) => x,
                    None => continue,
                };
            if empty_slot > i {
                continue;
            }

            let empty_left = *empty_slot_length - file_length;
            if empty_left == 0 {
                blocks.swap(i, empty_slot);
            } else {
                let file = std::mem::replace(
                    &mut blocks[i],
                    Block::Empty {
                        length: file_length,
                    },
                );
                blocks.insert(empty_slot, file);
                blocks[empty_slot + 1] = Block::Empty { length: empty_left };
            }
        }
    }

    let blocks = blocks
        .into_iter()
        .flat_map(|x| match x {
            Block::File { id, length } => vec![Some(id); length],
            Block::Empty { length } => vec![None; length],
        })
        .collect();

    checksum(blocks)
}

fn checksum(blocks: Vec<Option<usize>>) -> usize {
    blocks
        .iter()
        .enumerate()
        .map(|(i, x)| x.map(|x| i * x).unwrap_or(0))
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

    #[test]
    fn test_day9_part2() {
        assert_eq!(part2(TEST_INPUT), 2858);
    }
}
