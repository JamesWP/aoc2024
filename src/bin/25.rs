use itertools::Itertools;

advent_of_code::solution!(25);

#[derive(Debug, Copy, Clone)]
enum Schematic {
    Lock,
    Key,
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut lines = input.lines().chain("\n".lines());

    let mut keys = vec![];
    let mut locks = vec![];

    let mut nums = [0; 5];
    let mut schematic = None;

    for line in lines {
        if line.len() == 0 {
            nums.iter_mut().for_each(|v| *v -= 1);

            match schematic {
                Some(Schematic::Lock) => locks.push(nums),
                Some(Schematic::Key) => keys.push(nums),
                _ => todo!(),
            }
            nums = [0; 5];
            schematic = None;
            continue;
        }

        let pins = line.trim().chars().take(5).map(|v| match v {
            '.' => 0,
            '#' => 1,
            _ => todo!(),
        });

        let pins: Vec<i32> = pins.collect();
        assert!(pins.len() == 5);

        if schematic.is_none() {
            match pins.iter().sum::<i32>() {
                5 => schematic = Some(Schematic::Lock),
                _ => schematic = Some(Schematic::Key),
            }
            // dbg!(&pins);
            // dbg!(&schematic);
        }

        for pin in 0..5 {
            nums[pin] += pins[pin];
        }
    }

    fn key_fits_lock((key, lock): &(&[i32; 5], &[i32; 5])) -> bool {
        key.iter()
            .zip(lock.iter())
            .map(|(k, l)| k + l)
            .all(|sum| sum <= 5)
    }

    let fits = keys
        .iter()
        .cartesian_product(locks.iter())
        .filter(key_fits_lock)
        .count();

    Some(fits)
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
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
