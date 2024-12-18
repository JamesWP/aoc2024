use std::fmt::{Display, Formatter};

advent_of_code::solution!(15);

struct Grid {
    grid: Vec<char>,
    size: (i32, i32),
    loc: i32,

    instructions: Vec<char>,
}

impl Grid {
    fn boxes(&self) -> impl Iterator<Item = (i32, i32)> + '_ {
        self.grid.iter().enumerate().filter_map(move |(i, &c)| {
            if c == 'O' {
                Some((i as i32 % self.size.0, i as i32 / self.size.0))
            } else {
                None
            }
        })
    }
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let line_not_empty = |line: &&str| !line.is_empty();

        let grid: Vec<_> = input
            .lines()
            .take_while(|line| !line.is_empty())
            .flat_map(|line| line.chars())
            .collect();
        let instructions = input
            .lines()
            .skip_while(line_not_empty)
            .flat_map(|line| line.chars())
            .collect();

        // dbg!(&instructions);

        let loc = grid.iter().position(|&c| c == '@').unwrap() as i32;

        Grid {
            grid,
            loc,
            size: (
                input
                    .lines()
                    .take_while(|line| !line.is_empty())
                    .count()
                    .try_into()
                    .unwrap(),
                input.lines().next().unwrap().len().try_into().unwrap(),
            ),
            instructions,
        }
    }
}

impl Grid {
    fn push(&mut self, direction: char) {
        let loc = self.loc;
        assert!(self.grid[loc as usize] == '@');
        let direction = match direction {
            '^' => -self.size.0,
            'v' => self.size.0,
            '<' => -1,
            '>' => 1,
            d => panic!("Invalid direction, {}", d),
        };

        let pushing = self.grid[(loc + direction) as usize] == 'O';
        let moving = self.grid[(loc + direction) as usize] == '.';

        let is_not_wall = |i: &i32| self.grid[(loc + i * direction) as usize] != '#';
        let is_not_space = |i: &i32| self.grid[(loc + i * direction) as usize] != '.';

        if pushing {
            let dist_to_wall = 1 + (1..).take_while(is_not_wall).count() as i32;
            let dist_to_space = 1 + (1..dist_to_wall).take_while(is_not_space).count() as i32;

            let found_space = self.grid[(loc + dist_to_space * direction) as usize] == '.';
            let found_wall = self.grid[(loc + dist_to_wall * direction) as usize] == '#';
            assert!(found_wall);
            let push_success = found_space && dist_to_space < dist_to_wall;

            if push_success {
                // move `amount` boxes by one space

                // @OOO..#
                // .@OOO.#
                let space_loc = loc + (dist_to_space * direction);
                self.grid[space_loc as usize] = 'O';
                self.grid[(loc + direction) as usize] = '@';
                self.grid[loc as usize] = '.';

                self.loc = loc + direction;
            }
        } else if moving {
            self.grid[(loc + direction) as usize] = '@';
            self.grid[loc as usize] = '.';

            self.loc = loc + direction;
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                write!(f, "{}", self.grid[(x + y * self.size.0) as usize])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut grid = Grid::from(input);
    println!("{}", &grid);

    for instruction in grid.instructions.clone() {
        //println!("{}", instruction);
        grid.push(instruction);
        //println!("{}", &grid);
    }
    let gps_coords = |(left, top)| 100 * top + left;
    Some(grid.boxes().map(gps_coords).sum())
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
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
