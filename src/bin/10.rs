use std::collections::HashSet;

use memoize::memoize;

advent_of_code::solution!(10);

#[derive(Debug, Hash)]
struct Grid {
    cells: Vec<char>,
    width: i32,
    height: i32,
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let cells = input.lines().flat_map(|line| line.chars()).collect();
        let width = input.lines().count();
        let height = input.lines().next().unwrap().len() as i32;

        Grid {
            cells,
            width: width as i32,
            height: height as i32,
        }
    }
}

// test a path recursively
fn score_path(grid: &Grid, location: (i32, i32)) -> u32 {
    score_partial_path(grid, location, 0)
        .len()
        .try_into()
        .unwrap()
}

#[memoize(Ignore: grid)]
// score a partial path recursively
fn score_partial_path(grid: &Grid, location: (i32, i32), steps_taken: u32) -> HashSet<(i32, i32)> {
    let (x, y) = location;

    // check if we are allowed to step here, steps must be monotincally increasing
    let curent_cell = grid.cells[(location.0 + location.1 * grid.width) as usize];
    let curent_cell: u32 = curent_cell.to_digit(10).unwrap();

    if steps_taken != curent_cell {
        return HashSet::new();
    }

    // check if we are at the end of the path
    if curent_cell == 9 {
        return HashSet::from([(x, y)]);
    }

    // score the path recursively
    let mut paths = HashSet::new();
    for (dx, dy) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let new_location = (x + dx, y + dy);
        // check if we are out of bounds
        if new_location.0 >= grid.width || new_location.1 >= grid.height {
            continue;
        }
        if new_location.0 < 0 || new_location.1 < 0 {
            continue;
        }
        let new_steps_taken = steps_taken + 1;
        paths.extend(score_partial_path(grid, new_location, new_steps_taken));
    }

    paths
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::from(input);

    let mut total_score = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            // print!("scoring: x: {}, y: {}", x, y);
            let score = score_path(&grid, (x, y));
            // println!(" score: {}", score);
            total_score += score;
        }
    }

    Some(total_score.try_into().unwrap())
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
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
