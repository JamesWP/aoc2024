advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut count = 0;

    for line in input.lines() {
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(str::parse)
            .map(Result::unwrap)
            .collect();
        if safe(levels) {
            count += 1;
        }
    }

    Some(count as u32)
}

fn safe(levels: Vec<i32>) -> bool {
    let strictly_decreasing = levels.windows(2).filter(|s| s[0] < s[1]).count() == 0;
    let strictly_increasing = levels.windows(2).filter(|s| s[0] > s[1]).count() == 0;
    let diff_bounded = levels
        .windows(2)
        .filter(|s| {
            let diff = (s[0] - s[1]).abs();

            diff > 3 || diff < 1
        })
        .count()
        == 0;

    if (strictly_decreasing || strictly_increasing) && diff_bounded {
        return true;
    }

    return false;
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut count = 0;

    for line in input.lines() {
        let mut fixable = false;
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(str::parse)
            .map(Result::unwrap)
            .collect();
        for (i, _level) in levels.iter().enumerate() {
            let mut levels = levels.clone();
            levels.remove(i);
            if safe(levels) {
                count += 1;
                fixable = true;
                break;
            }
        }
        println!("{:}, {:}", fixable, line);
    }

    Some(count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
