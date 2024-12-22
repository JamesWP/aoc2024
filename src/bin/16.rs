use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

advent_of_code::solution!(16);

struct Maze {
    data: Vec<bool>,
    size: (i32, i32),
    start: i32,
    end: i32,
}

impl From<&str> for Maze {
    fn from(input: &str) -> Self {
        let data = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .flatten()
            .collect::<Vec<_>>();
        let size = (
            input.lines().count() as i32,
            input.lines().next().unwrap().len() as i32,
        );
        let start = data.iter().position(|&c| c == 'S').unwrap() as i32;
        let end = data.iter().position(|&c| c == 'E').unwrap() as i32;
        let data = data.iter().map(|&c| c != '#').collect::<Vec<_>>();
        Maze {
            data,
            size,
            start,
            end,
        }
    }
}

impl Maze {
    fn neighbors(&self, pos: i32, direction: i8) -> [Option<(i32, i8)>; 3] {
        // we can either turn left, right or go straight
        let left = (direction - 1).rem_euclid(4);
        let right = (direction + 1).rem_euclid(4);
        let straight = pos
            + match direction {
                0 => -self.size.1,
                1 => 1,
                2 => self.size.1,
                3 => -1,
                _ => unreachable!(),
            };

        let mut out = [
            Some((pos, left)),
            Some((pos, right)),
            Some((straight, direction)),
        ];

        out.iter_mut().for_each(|a| {
            if let Some((pos, _direction)) = a {
                if !self.data[*pos as usize] {
                    *a = None;
                }
            }
        });

        out
    }

    fn shortest_distance(&self) -> i32 {
        // using dijkstra's algorithm, calculate the shortest path from each node to the end
        let mut distances = vec![vec![i32::MAX; 4]; self.data.len()];
        let mut queue: BinaryHeap<Reverse<(i32, i32, i8)>> = BinaryHeap::new();

        distances[self.start as usize][1] = 0;
        queue.push(std::cmp::Reverse((0, self.start, 1)));

        while let Some(std::cmp::Reverse((distance, pos, direction))) = queue.pop() {
            // println!("{} {}x{} {}", distance, pos / self.size.1, pos % self.size.1, direction);
            // self.print_maze(pos, direction);

            let neighbors = self.neighbors(pos, direction);
            for (neighbor_pos, neighbour_direction) in neighbors.into_iter().filter_map(|a| a) {
                assert!((neighbor_pos == pos) ^ (neighbour_direction == direction));
                let new_distance = distance
                    + if direction == neighbour_direction {
                        1
                    } else {
                        1000
                    };
                if new_distance < distances[neighbor_pos as usize][neighbour_direction as usize] {
                    distances[neighbor_pos as usize][neighbour_direction as usize] = new_distance;
                    queue.push(std::cmp::Reverse((
                        new_distance,
                        neighbor_pos,
                        neighbour_direction,
                    )));
                }
            }
        }

        distances[self.end as usize].iter().copied().min().unwrap()
    }

    fn print_maze(&self, pos: i32, direction: i8) {
        for (i, row) in self.data.chunks(self.size.1 as usize).enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if i == pos as usize / self.size.1 as usize
                    && j == pos as usize % self.size.1 as usize
                {
                    print!("\x1b[31m");
                    print!(
                        "{}",
                        match direction {
                            0 => '^',
                            1 => '>',
                            2 => 'v',
                            3 => '<',
                            _ => unreachable!(),
                        }
                    );
                } else {
                    print!("\x1b[0m");
                    print!("{}", if *cell { '.' } else { '#' });
                }
            }
            println!();
        }
        println!();
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let maze = Maze::from(input);
    Some(maze.shortest_distance().try_into().unwrap())
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
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(64));
        assert_eq!(result, None);
    }
}
