use itertools::Itertools;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<usize> {
    let input = input.trim();
    let input: Vec<_> = input.chars().map(|c| c.to_digit(10).unwrap()).collect();

    let mut blocks: Vec<i64> = Vec::new();

    let mut free = false;
    let mut file_id = 0;

    for size in input.into_iter() {
        if free {
            blocks.extend((0..size).map(|_| -1));
            free = false;
        } else {
            blocks.extend((0..size).map(|_| file_id.clone()));
            file_id += 1;
            free = true;
        }
    }

    // println!(
    //     "{}",
    //     blocks
    //         .iter()
    //         .map(|i| if *i >= 0 {
    //             i.to_string().chars().next().unwrap()
    //         } else {
    //             '.'
    //         })
    //         .collect::<String>()
    // );

    let mut l_idx = 0;
    let mut r_idx = blocks.len() - 1;

    while l_idx < r_idx {
        if blocks[l_idx] >= 0 {
            l_idx += 1;
            continue;
        }
        if blocks[r_idx] < 0 {
            r_idx -= 1;
            continue;
        }
        blocks[l_idx] = blocks[r_idx];
        blocks[r_idx] = -1;
        l_idx += 1;
        r_idx -= 1;
    }

    // println!(
    //     "{}",
    //     blocks
    //         .iter()
    //         .map(|i| if *i >= 0 {
    //             i.to_string().chars().next().unwrap()
    //         } else {
    //             '.'
    //         })
    //         .collect::<String>()
    // );

    let checksum = blocks
        .into_iter()
        .enumerate()
        .map(|(i, b)| if b > 0 { b * (i as i64) } else { 0 })
        .sum::<i64>();

    Some(checksum.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = input.trim();
    let input: Vec<_> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    let mut files: Vec<(usize, usize, usize)> = Vec::new();

    let mut free = false;
    let mut file_id = 0;
    let mut beginning = 0;
    for size in input.into_iter() {
        if free {
            free = false;
        } else {
            files.push((file_id, beginning, size));
            file_id += 1;
            free = true;
        }
        beginning += size;
    }

    let total_size = beginning;
    // dbg!(total_size);
    let mut blocks: Vec<i64> = Vec::with_capacity(total_size);
    blocks.resize(total_size, -1);

    for (file_id, beginning, size) in files.iter().cloned() {
        let file = &mut blocks[beginning..beginning + size];
        file.fill(file_id as i64);
    }

    // println!(
    //     "{}",
    //     blocks
    //         .iter()
    //         .map(|i| if *i >= 0 {
    //             i.to_string().chars().next().unwrap()
    //         } else {
    //             '.'
    //         })
    //         .collect::<String>()
    // );

    for (file_id, beginning, size) in files.into_iter().rev() {
        use itertools::FoldWhile::{Continue, Done};

        let mut file_final_location: usize = beginning.try_into().unwrap();
        // place file at beginning..beginning+size
        // find if there is space in 0..beginning which will fit size bytes
        let gap = blocks
            .iter()
            .take(beginning)
            .cloned()
            .enumerate()
            .fold_while((0, 0), |(acc_idx, count), (idx, v)| {
                if v == -1 {
                    if count + 1 == size {
                        // we found a gap with enough space
                        Done((acc_idx, count + 1))
                    } else {
                        // gap is not done yet
                        Continue((acc_idx, count + 1))
                    }
                } else {
                    // gap is done, start over
                    Continue((idx + 1, 0))
                }
            });

        match gap {
            Done((idx, count)) => {
                assert!(size <= count);
                file_final_location = idx;

                // remove from original location
                let file = &mut blocks[beginning..beginning + size];
                file.fill(-1);
            }
            _ => {}
        }

        // place file there
        let file = &mut blocks[file_final_location..file_final_location + size];
        file.fill(file_id as i64);

        // println!(
        //     "{}",
        //     blocks
        //         .iter()
        //         .map(|i| if *i >= 0 {
        //             i.to_string().chars().next().unwrap()
        //         } else {
        //             '.'
        //         })
        //         .collect::<String>()
        // );
    }

    let checksum = blocks
        .into_iter()
        .enumerate()
        .map(|(i, b)| if b > 0 { b * (i as i64) } else { 0 })
        .sum::<i64>();

    Some(checksum.try_into().unwrap())
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
        assert_eq!(result, Some(2858));
    }
}
