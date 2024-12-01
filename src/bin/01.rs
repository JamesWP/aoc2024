advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left_list, mut right_list) = get_lists(input);

    left_list.sort();
    right_list.sort();

    Some(left_list.iter().zip(right_list).map(|(left, right)|{
        left.abs_diff(right)
    }).sum())
}

fn get_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    input.lines().for_each(|line|{
        let mut parts= line.split_whitespace().map(|v|v.parse::<i32>().unwrap());
    
        let left = parts.next().unwrap();
        let right = parts.next().unwrap();
        left_list.push(left);
        right_list.push(right); 
    });
    (left_list, right_list)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut similarity = 0;
    let (mut left_list, mut right_list) = get_lists(input);

    for item in left_list {
        for other in &right_list {
            if *other == item {
                similarity += other;
            }
        }
    }

    Some(similarity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
