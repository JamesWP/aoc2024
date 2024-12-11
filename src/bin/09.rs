use std::fmt::Write;

advent_of_code::solution!(9);

#[derive(Debug, Clone, Copy)]
enum Block {
    File { id: usize, size: usize },
    FreeSpace { size: usize },
}

impl From<(usize, char)> for Block {
    fn from((index, c): (usize, char)) -> Self {
        let file = index % 2 == 0;
        let size = c.to_digit(10).unwrap() as usize;
        if file {
            Block::File {
                id: index / 2,
                size,
            }
        } else {
            Block::FreeSpace { size }
        }
    }
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Block::File { id, size } => {
                for _ in 0..*size {
                    f.write_str(&(id%10).to_string()).unwrap();
                }
                Ok(())
            }
            Block::FreeSpace { size } => {
                for _ in 0..*size {
                    f.write_char('.').unwrap();
                }
                Ok(())
            },
        }
    }
}

fn debug(blocks: &[Block]) {
    // for block in blocks {
    //     print!("{:}", block);
    // }
    // println!();
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut blocks: Vec<Block> = input.trim().char_indices().map(Block::from).collect();
    let orig_blocks = blocks.clone();
    assert!(blocks.len() % 2 == 1);

    let mut defrag_blocks: Vec<Block> = Vec::new();

    let mut left = 0;
    let mut right = blocks.len() - 1;

    while left < right {
        debug(&orig_blocks);
        debug(&defrag_blocks);
        let free_space = match &blocks[left] {
            Block::File { .. } => {
                defrag_blocks.push(blocks[left].clone());
                left += 1;
                continue;
            }
            Block::FreeSpace { size } => *size,
        };
        // We have some free space on the left

        let (file_id, file_size) = match &blocks[right] {
            Block::File { id, size } => (*id, *size),
            Block::FreeSpace { .. } => {
                right -= 1;
                continue;
            }
        };
        // We have a file on the right

        // Transfer some bytes over
        let transfer_size = std::cmp::min(file_size, free_space);
        if file_size - transfer_size > 0 {
            // remember if we're partially though when we're done
            blocks[right] = Block::File {
                id: file_id,
                size: file_size - transfer_size,
            };
        } else {
            right -= 1;
        }

        defrag_blocks.push(Block::File {
            id: file_id,
            size: transfer_size,
        });

        if free_space - transfer_size > 0 {
            blocks[left] = Block::FreeSpace {
                size: free_space - transfer_size,
            };
        } else {
            left += 1;
        }
    }
    match &blocks[right] {
        Block::File { id, size } => {
            defrag_blocks.push(Block::File{id: *id, size: *size});
        },
        _ => ()
    }

    let blocks = defrag_blocks;

    let checksum = blocks.into_iter().fold((0, 0), |acc, block| {
        let (block_idx, checksum) = acc;
        let size = match block {
            Block::File { size, .. } => size,
            Block::FreeSpace { size } => size,
        };
        let block_id = match block {
            Block::File { id, .. } => id,
            Block::FreeSpace { .. } => 0,
        };

        let block_sum: usize = (block_idx..block_idx + size)
            .map(|block_idx| block_idx * block_id)
            .sum();

        (block_idx + size, checksum + block_sum)
    });

    Some(checksum.1.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
