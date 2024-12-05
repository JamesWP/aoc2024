use std::collections::HashMap;

advent_of_code::solution!(4);

#[derive(Clone, Copy)]
struct Cell(char);

impl PartialEq<char> for Cell {
    fn eq(&self, other: &char) -> bool {
        match self {
            Cell(letter) => letter == other,
        }
    }
}

struct Grid {
    cells: HashMap<(i32, i32), Cell>,
    cells_by_letter: HashMap<char, Vec<(i32, i32)>>,
}

impl Grid {
    fn parse(input: &str) -> Grid {
        let mut cells = HashMap::new();
        for (y, line) in input.lines().enumerate() {
            for (x, letter) in line.chars().enumerate() {
                cells.insert((x as i32, y as i32), Cell(letter));
            }
        }

        let mut cells_by_letter = HashMap::new();
        for (pos, cell) in &cells {
            match cell {
                Cell(letter) => {
                    cells_by_letter
                        .entry(*letter)
                        .or_insert(Vec::new())
                        .push(*pos);
                }
            }
        }

        Grid {
            cells,
            cells_by_letter,
        }
    }

    fn default_cell() -> Cell {
        Cell('.')
    }

    fn get(&self, x: i32, y: i32) -> Cell {
        let c = self.cells.get(&(x, y));
        let c = c.copied();
        c.unwrap_or(Self::default_cell())
    }

    fn get_by_letter(&self, letter: char) -> Vec<(i32, i32)> {
        let c = self.cells_by_letter.get(&letter);
        let c = c.cloned();
        c.unwrap_or(Vec::new())
    }
}

fn pt1_pattern(grid: &Grid, ix: i32, y: i32, dx: i32, dy: i32) -> i32 {
    let x = grid.get(ix, y);
    let m = grid.get(ix + dx, y + dy);
    let a = grid.get(ix + 2 * dx, y + 2 * dy);
    let s = grid.get(ix + 3 * dx, y + 3 * dy);

    if x == 'X' && m == 'M' && a == 'A' && s == 'S' {
        return 1;
    } else {
        return 0;
    }
}

fn pt2_pattern(grid: &Grid, x: i32, y: i32, dx1: i32, dy1: i32, dx2: i32, dy2: i32) -> bool {
    let a = grid.get(x + dx1, y + dy1);
    let b = grid.get(x + dx2, y + dy2);

    (a == 'M' && b == 'S') || (a == 'S' && b == 'M')
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::parse(input);
    let mut count = 0;

    for (x, y) in grid.get_by_letter('X') {
        count += pt1_pattern(&grid, x, y, 1, 0); // Right
        count += pt1_pattern(&grid, x, y, -1, 0); // Left
        count += pt1_pattern(&grid, x, y, 0, 1); // Down
        count += pt1_pattern(&grid, x, y, 0, -1); // Up
        count += pt1_pattern(&grid, x, y, 1, 1); // Down Right
        count += pt1_pattern(&grid, x, y, -1, -1); // Up Left
        count += pt1_pattern(&grid, x, y, 1, -1); // Up Right
        count += pt1_pattern(&grid, x, y, -1, 1); // Down Left
    }
    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::parse(input);
    let mut count = 0;

    // M .     S .
    //  A   or  A
    // . S     . M

    // AND

    // . M     . S
    //  A   or  A
    // S .     M .

    for (x, y) in grid.get_by_letter('A') {
        if pt2_pattern(&grid, x, y, -1, -1, 1, 1) && pt2_pattern(&grid, x, y, 1, -1, -1, 1) {
            // Up Left - Down Right
            // Up Right - Down Left
            count += 1;
        }
    }

    Some(count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
