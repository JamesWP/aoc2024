use itertools::Itertools;

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u32> {
    // Button A: X+94, Y+34
    // Button B: X+22, Y+67
    // Prize: X=8400, Y=5400

    let regex = r"Button A: X\+(?<ax>\d+), Y\+(?<ay>\d+)\nButton B: X\+(?<bx>\d+), Y\+(?<by>\d+)\nPrize: X=(?<px>\d+), Y=(?<py>\d+)".to_string();
    #[cfg(windows)]
    let regex = regex.replace("\\n", "\\r\\n");
    let regex = regex::Regex::new(&regex).unwrap();
    let caps = regex.captures_iter(input);
    let games = caps.map(|cap| {
        let ax = cap.name("ax").unwrap().as_str().parse::<u32>().unwrap();
        let ay = cap.name("ay").unwrap().as_str().parse::<u32>().unwrap();
        let bx = cap.name("bx").unwrap().as_str().parse::<u32>().unwrap();
        let by = cap.name("by").unwrap().as_str().parse::<u32>().unwrap();
        let px = cap.name("px").unwrap().as_str().parse::<u32>().unwrap();
        let py = cap.name("py").unwrap().as_str().parse::<u32>().unwrap();
        let a = (ax, ay);
        let b = (bx, by);
        let p = (px, py);
        (a, b, p)
    });

    let min_win_score = |(a, b, p): ((u32, u32), (u32, u32), (u32, u32))| {
        let min_score = (0..=100)
            .cartesian_product(0..=100)
            .filter_map(|(ba, bb)| {
                let end_x = ba * a.0 + bb * b.0;
                let end_y = ba * a.1 + bb * b.1;
                if end_x == p.0 && end_y == p.1 {
                    Some(ba * 3 + bb * 1)
                } else {
                    None
                }
            })
            .min();
        min_score
    };

    Some(
        games
            .filter_map(min_win_score)
            .sum::<u32>()
            .try_into()
            .unwrap(),
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
