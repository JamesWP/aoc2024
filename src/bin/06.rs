use std::collections::{BTreeSet, HashMap, HashSet};

advent_of_code::solution!(6);

struct Grid {
    cells: Vec<char>,
    starting_location: (i32, i32),
    size: (i32, i32),
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let mut cells = Vec::new();
        let mut starting_location = (0, 0);

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                cells.push(c);
                if c == '^' {
                    starting_location = (x as i32, y as i32);
                }
            }
        }

        let size = (input.lines().next().unwrap().len() as i32, input.lines().count() as i32);

        Grid {
            cells,
            starting_location,
            size,
        }
    }
}

impl Grid {
    fn contains(&self, location: (i32, i32)) -> bool {
        location.0 >= 0 && location.0 < self.size.0 && location.1 >= 0 && location.1 < self.size.1
    }

    fn occupied(&self, location: (i32, i32)) -> bool {
        // we must first do a bounds check
        if !self.contains(location) {
            return false;
        }
        let index = (location.1 * self.size.0 + location.0) as isize;
        // we can safely unwrap here because we know the index is valid
        // This is unsafe because i want to go fast, don't ask
        let cell = unsafe {self.cells.as_ptr().offset(index).read()};
        cell == '#'
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::from(input);
    match path_to_escape(&grid, true) {
        Length::Finite(Some(path)) => Some(path.len() as u32),
        _ => panic!("nope"),
    }
}

enum Length {
    Finite(Option<Vec<(i32, i32)>>),
    Infinite,
}

fn path_to_escape(grid: &Grid, want_path: bool) -> Length {
    let mut location = grid.starting_location;
    let mut direction = (0, -1);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    let mut path: BTreeSet<((i32, i32), (i32,i32))> = BTreeSet::new();
    loop {
        if !grid.contains(location) {
            if want_path {
                break Length::Finite(Some(visited.iter().cloned().collect()));
            } else {
                break Length::Finite(None);
            }
        }

        if want_path {
            visited.insert(location);
        }

        let new_location = (location.0 + direction.0, location.1 + direction.1);

        if grid.occupied(new_location) {
            // Turn right
            direction = match direction {
                (0, -1) => (1, 0),
                (1, 0) => (0, 1),
                (0, 1) => (-1, 0),
                (-1, 0) => (0, -1),
                _ => panic!("Invalid direction"),
            };

            if path.contains(&(location, direction)) {
                break Length::Infinite;
            }
            path.insert((location, direction));
        } else {
            // Move forward
            location = new_location;
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = Grid::from(input);
    let mut count = 0;

    let cells = match path_to_escape(&grid, true) {
        Length::Finite(Some(path)) => path,
        _ => panic!("nope"),
    };

    for position in cells {

        if grid.occupied(position) {
            continue;
        }

        grid.cells[position.1 as usize * grid.size.0 as usize + position.0 as usize] = '#';

        if let Length::Infinite = path_to_escape(&grid, false) {
            count += 1;
        }

        grid.cells[position.1 as usize * grid.size.0 as usize + position.0 as usize] = '.';
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
