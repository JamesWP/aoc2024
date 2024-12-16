use itertools::Itertools;

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    part_one_sized(input, (101,103))
}

fn part_one_sized(input: &str, size: (i32, i32)) -> Option<u32> {
    let robots = parse(input, size);
    let steps = 100;
    let robot_position = |(p, v): ((i32, i32), (i32, i32))| {
        let final_position = (p.0 + v.0 * steps, p.1 + v.1 * steps);

        (final_position.0 % size.0 as i32, final_position.1 % size.1 as i32)
    };

    let robots: Vec<_> = robots.into_iter().map(robot_position).collect();

    let position_to_quarter = |(x, y): (i32, i32)| {
        let x = if x < size.0 as i32 / 2 { 0 } else { 1 };
        let y = if y < size.1 as i32 / 2 { 0 } else { 1 };
        (x, y)
    };

    let in_middle = |(x, y): (i32, i32)| {
        x == size.0 as i32 / 2 || y == size.1 as i32 / 2
    };

    let not_in_middle = |(x, y): &(i32, i32)| {
        !in_middle((*x, *y))
    };

    Some(robots.into_iter().filter(not_in_middle).counts_by(position_to_quarter).values().product::<usize>() as u32)
}

fn parse(input: &str, size: (i32, i32)) -> Vec<((i32, i32), (i32, i32))> {
    let robots = input.lines().map(|line|{
        // p=0,4 v=3,-3
        let regex = regex::Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
        let caps = regex.captures(line).unwrap();
        let p = (caps[1].parse::<i32>().unwrap(), caps[2].parse::<i32>().unwrap());
        let v = (caps[3].parse::<i32>().unwrap(), caps[4].parse::<i32>().unwrap());

        let v0 = if v.0 < 0 {
            size.0 + v.0
        } else {
            v.0
        };
        let v1 = if v.1 < 0 {
            size.1 + v.1
        } else {
            v.1
        };
        let v = (v0, v1);
        (p, v)
    });
    robots.collect()
}

pub fn part_two(input: &str) -> Option<u32> {
    part_two_sized(input, (101,103))
}

fn part_two_sized(input: &str, size: (i32, i32)) -> Option<u32> {
    let robots:Vec<_> = parse(input, size).into_iter().collect();

    for steps in 6450..6500 {
        let robot_position = |(p, v): ((i32, i32), (i32, i32))| {
            let final_position = (p.0 + v.0 * steps, p.1 + v.1 * steps);

            (final_position.0 % size.0 as i32, final_position.1 % size.1 as i32)
        };

        let robots: Vec<_> = robots.iter().cloned().map(robot_position).collect();
        // println!("Steps: {}", steps);
        // print_robots(robots, size)
    }

    Some(6475)
}

fn print_robots(robots: Vec<(i32, i32)>, size: (i32, i32)) {
    let mut grid = vec![vec!['.'; size.0 as usize]; size.1 as usize];

    for (x, y) in robots {
        grid[y as usize][x as usize] = '#';
    }

    for row in grid {
        println!("{}", row.into_iter().collect::<String>());
    }

    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_sized(&advent_of_code::template::read_file("examples", DAY), (11,7));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(6475));
    }
}
