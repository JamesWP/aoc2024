use itertools::Itertools;

advent_of_code::solution!(5);

struct Input {
    rules: Vec<(i32, i32)>,

    orderings: Vec<Vec<i32>>,
}

impl Input {
    fn parse(input: &str) -> Input {
        let first = input.split("\n\n").nth(0).unwrap();
        let rules = first
            .lines()
            .map(|line| {
                let (first_page, subsequent_page) = line.split("|").collect_tuple().unwrap();
                (
                    first_page.parse().unwrap(),
                    subsequent_page.parse().unwrap(),
                )
            })
            .collect();

        let second = input.split("\n\n").nth(1).unwrap();
        let orderings = second
            .lines()
            .map(|line| -> Vec<i32> { line.split(",").map(|i| i.parse().unwrap()).collect() })
            .collect();

        Input { rules, orderings }
    }
}

fn middle<SliceInt>(ordering: SliceInt) -> u32
where
    SliceInt: AsRef<[i32]>,
{
    let middle_index = ordering.as_ref().len() / 2;
    ordering.as_ref()[middle_index] as u32
}

fn are_rules_followed(ordering: &&Vec<i32>, input_rules: &Vec<(i32, i32)>) -> bool {
    input_rules.iter().all(|(before, after)| {
        let before = ordering.iter().position(|x| x == before);
        let after = ordering.iter().position(|x| x == after);

        match (before, after) {
            (Some(before), Some(after)) => before < after,
            _ => true,
        }
    })
}
pub fn part_one(input: &str) -> Option<u32> {
    let input = Input::parse(input);

    let correctly_ordered = input
        .orderings
        .iter()
        .filter(|ordering| are_rules_followed(ordering, &input.rules));

    Some(correctly_ordered.map(middle).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = Input::parse(input);

    let incorrectly_ordered = input
        .orderings
        .iter()
        .filter(|ordering| !are_rules_followed(ordering, &input.rules));

    let compare = |a: &i32, b: &i32| {
        let in_order = input
            .rules
            .iter()
            .find(|(before, after)| a == before && b == after)
            .is_some();

        if in_order {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Equal
        }
    };

    let correctly_ordered = incorrectly_ordered.map(|ordering| {
        let mut ordering = ordering.clone();
        ordering.sort_by(compare);
        ordering
    });

    Some(correctly_ordered.map(middle).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
