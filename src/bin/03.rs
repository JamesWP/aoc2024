advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let regex: regex::Regex = regex::Regex::new(
        r"(mul\((?<lhs>\d{1,3}),(?<rhs>\d{1,3})\))|(?<do>do\(\))|(?<dont>don't\(\))",
    )
    .unwrap();
    let mut count = 0;
    regex.captures_iter(input).for_each(|capture| {
        let lhs = capture.name("lhs");
        let rhs = capture.name("rhs");

        //dbg!(lhs, rhs);
        count += match (lhs, rhs) {
            (Some(lhs), Some(rhs)) => {
                lhs.as_str().parse::<u32>().unwrap() * rhs.as_str().parse::<u32>().unwrap()
            }
            _ => 0,
        };
    });

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let regex: regex::Regex = regex::Regex::new(
        r"(mul\((?<lhs>\d{1,3}),(?<rhs>\d{1,3})\))|(?<do>do\(\))|(?<dont>don't\(\))",
    )
    .unwrap();
    let mut count = 0;
    let mut enabled = true;
    regex.captures_iter(input).for_each(|capture| {
        let lhs = capture.name("lhs");
        let rhs = capture.name("rhs");
        let do_ = capture.name("do");
        let dont = capture.name("dont");

        count += match (lhs, rhs, do_, dont) {
            (Some(lhs), Some(rhs), _, _) => {
                if !enabled {
                    0
                } else {
                    lhs.as_str().parse::<u32>().unwrap() * rhs.as_str().parse::<u32>().unwrap()
                }
            }
            (_, _, Some(_), _) => {
                enabled = true;
                0
            }
            (_, _, _, Some(_)) => {
                enabled = false;
                0
            }
            _ => 0,
        };
    });

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
