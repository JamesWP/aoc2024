
advent_of_code::solution!(7);

fn possible(line: &str) -> (bool, bool) {
    let mut numbers: Vec<i64> = line
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split(" ")
        .map(str::parse)
        .map(Result::unwrap)
        .collect();
    let test_result: i64 = the_test_value(line);

    (can_operations_make(test_result, &mut numbers[..], false),
    can_operations_make(test_result, &mut numbers[..], true))
}

fn can_operations_make(result: i64, numbers: &mut [i64], can_cons: bool) -> bool {
    if numbers.len() == 1 {
        return result == numbers[0];
    }

    let (first, rest) = numbers.split_first_mut().unwrap();
    let first = *first;

    if first > result {
        return false;
    }

    let rest_base = rest[0];

    rest[0] = rest_base + first;
    if can_operations_make(result, rest, can_cons) {
        rest[0] = rest_base;
        return true;
    }
    rest[0] = rest_base * first;
    if can_operations_make(result, rest, can_cons) {
        rest[0] = rest_base;
        return true;
    }
    if can_cons {
        let new_rest: i64 = format!("{first}{rest_base}").parse().unwrap();
        rest[0] = new_rest;
        if can_operations_make(result, rest, can_cons) {
            rest[0] = rest_base;
            return true;
        }
    }
    rest[0] = rest_base;

    return false;
}

fn the_test_value(line: &str) -> i64 {
    line.split(":").nth(0).unwrap().parse().unwrap()
}

pub fn part_one(input: &str) -> Option<u64> {

    // Too low: 1836066741
    Some(input.lines().filter(|line: &&str| possible(*line).0).map(the_test_value).sum::<i64>()as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(input.lines().filter(|line: &&str| possible(*line).1).map(the_test_value).sum::<i64>()as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 85592486: 8 551 4 28 153
    /// 159536: 3 1 175 2 767
    /// 10539: 2 384 25 795 94

    #[test]
    fn extra() {
        let mut numbers = vec![8, 551, 4, 28, 153];
        assert_eq!(can_operations_make(85592486, &mut numbers[..], false), false);

        let mut numbers = vec![3, 1, 175, 2, 767];
        assert_eq!(can_operations_make(159536, &mut numbers[..], false), false);

        let mut numbers = vec![2, 384, 25, 795, 94];
        assert_eq!(can_operations_make(10539, &mut numbers[..], false), true);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
