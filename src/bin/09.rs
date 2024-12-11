advent_of_code::solution!(9);

#[derive(Debug, Clone, Copy)]
enum Block {
    File{ id: usize, size: usize },
    FreeSpace { size: usize},
}


impl From<(usize, char)> for Block {
    fn from((index, c): (usize, char)) -> Self {
        let file = index %2 == 0;
        let size = c.to_digit(10).unwrap() as usize;
        if file {
            Block::File { id: index/2, size }
        } else {
            Block::FreeSpace { size }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut blocks: Vec<Block> = input.char_indices().map(Block::from).collect();
    assert!(blocks.len() %2 ==1);

    let mut i_destination = 1;
    assert!(matches!(blocks[i_destination], Block::FreeSpace { .. }));

    let mut i_source = blocks.len() -1;
    assert!(matches!(blocks[i_source], Block::File { .. }));

    while i_source > i_destination {
        dbg!(i_source, i_destination);
        let destination_size = match &blocks[i_destination] {
            Block::FreeSpace{size} => *size,
            _ => unreachable!(),
        };
        let (source_size, source_id) = match &blocks[i_source] {
            Block::File { size, id } => (*size, *id),
            _ => unreachable!(),
        };

        let transfer_size = std::cmp::min(destination_size, source_size);

        blocks[i_destination] = Block::File { id: source_id, size: transfer_size };
        blocks[i_source] = Block::FreeSpace { size: source_size - transfer_size };

        if source_size - transfer_size == 0 {
            i_source -= 2;
        }
        if destination_size - transfer_size == 0 {
            i_destination += 2;
        }
    }

    let blocks = blocks;

    let checksum = blocks.into_iter().fold((0,0), |acc, block| {
        let (block_idx, checksum) = acc;
        let size = match block {
            Block::File { size, .. } => size,
            Block::FreeSpace { size } => size,
        };
        let block_id = match block {
            Block::File { id, .. } => id,
            Block::FreeSpace { .. } => 0,
        };

        let block_sum: usize = (block_idx..block_idx + size).map(|block_idx|block_idx*block_id).sum();

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
