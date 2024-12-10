use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(8);

struct Grid {
    cells: Vec<char>,
    size: (i32, i32),
    antennas: HashMap<char, Vec<(i32, i32)>>,
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let mut cells = Vec::new();
        let mut antennas = HashMap::new();

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                cells.push(c);
                if c.is_alphanumeric() {
                    antennas
                        .entry(c)
                        .or_insert_with(Vec::new)
                        .push((x as i32, y as i32));
                }
            }
        }

        let size = (
            input.lines().next().unwrap().len() as i32,
            input.lines().count() as i32,
        );

        Grid {
            cells,
            size,
            antennas,
        }
    }
}

fn antinodes((a, b): ((i32, i32), (i32, i32))) -> ((i32, i32), (i32, i32)) {
    // ab = -a + b
    let ab = (b.0 - a.0, b.1 - a.1);
    // antinode1 = a - ab
    let antinode1 = (a.0 - ab.0, a.1 - ab.1);
    // antinode2 = b + ab
    let antinode2 = (b.0 + ab.0, b.1 + ab.1);

    (antinode1, antinode2)
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    // If equal, return any of them
    if a == b {
        return a;
    }

    // Swap a with b, if b is greater than a
    if b > a {
        std::mem::swap(&mut a, &mut b);
    }

    while b > 0 {
        // This is the trickiest part
        // We swap a with b, and b with a%b, till b becomes 0
        let temp = a;
        a = b;
        b = temp % b;
    }

    // Now, a%b = 0, hence return it
    return a;
}

fn antinodes_with_harmonics((a, b): ((i32, i32), (i32, i32)), size: (i32, i32)) -> Vec<(i32, i32)> {
    let ab = (b.0 - a.0, b.1 - a.1);
    let gcd = gcd(ab.0.abs(), ab.1.abs());
    let ab = (ab.0 / gcd, ab.1 / gcd);

    let mut antinodes = Vec::new();
    for i in 0.. {
        let antinode1 = (a.0 - ab.0 * i, a.1 - ab.1 * i);
        let antinode2 = (a.0 + ab.0 * i, a.1 + ab.1 * i);

        let tot_nodes = antinodes.len();

        if antinode1.0 >= 0 && antinode1.0 < size.0 && antinode1.1 >= 0 && antinode1.1 < size.1 {
            antinodes.push(antinode1);
        }

        if antinode2.0 >= 0 && antinode2.0 < size.0 && antinode2.1 >= 0 && antinode2.1 < size.1 {
            antinodes.push(antinode2);
        }

        if tot_nodes == antinodes.len() {
            break;
        }
    }

    antinodes
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::from(input);

    let antennas = grid.antennas.iter().collect::<Vec<_>>();

    let in_grid = |(x, y): &(i32, i32)| x >= &0 && x < &grid.size.0 && y >= &0 && y < &grid.size.1;

    let mut all_antinodes: HashSet<(i32, i32)> = HashSet::new();
    for (_antenna, positions) in antennas.iter() {
        let antinodes = positions
            .iter()
            .permutations(2)
            .map(|iter| (iter[0].clone(), iter[1].clone()))
            .map(antinodes)
            .flat_map(|(a, b)| vec![a, b])
            .filter(in_grid);
        all_antinodes.extend(antinodes);
    }

    Some(all_antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::from(input);

    let antennas = grid.antennas.iter().collect::<Vec<_>>();

    let in_grid = |(x, y): &(i32, i32)| x >= &0 && x < &grid.size.0 && y >= &0 && y < &grid.size.1;

    let mut all_antinodes: HashSet<(i32, i32)> = HashSet::new();
    for (_antenna, positions) in antennas.iter() {
        let antinodes = positions
            .iter()
            .permutations(2)
            .map(|iter| (iter[0].clone(), iter[1].clone()))
            .map(|antennas| antinodes_with_harmonics(antennas, grid.size))
            .flatten()
            .filter(in_grid);
        all_antinodes.extend(antinodes);
    }

    Some(all_antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }

    #[test]
    fn test_simple_2() {
        let input =
r"T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........
";
        let result = part_two(input);
        assert_eq!(result, Some(9));
    }

    #[test]
    fn test_simple_3() {
        // 1, 2,
        // 3, 1,
        let antinodes = antinodes_with_harmonics(((1,2), (3,1)), (9,9));

        assert_eq!(4, antinodes.len());
    }
}
