#![allow(dead_code, unused)]
use std::{cmp::Reverse, collections::BinaryHeap};

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<u32> {
    part_one_sized(input, 71, 71, 1024)
}

pub fn part_two(input: &str) -> Option<String> {
    part_two_sized(input, 71, 71)
}

fn part_two_sized(input: &str, width: usize, height: usize) -> Option<String> {
    let num_lines = input.lines().count();

    let mut range = 0..num_lines;

    loop {
        let mid = range.start + range.len() / 2;
        let result = part_one_sized(input, width, height, mid).unwrap();
        if result != u32::MAX {
            range = mid..range.end;
        } else {
            range = range.start..mid;
        }
        if range.len() == 1 {
            return Some(input.lines().nth(range.start).unwrap().to_string());
        }
    }
}

fn part_one_sized(input: &str, width: usize, height: usize, steps: usize) -> Option<u32> {
    let grid = input
        .lines()
        .take(steps)
        .map(|line| {
            let x: usize = line.split(",").next().unwrap().parse().unwrap();
            let y: usize = line.split(",").skip(1).next().unwrap().parse().unwrap();
            (x, y)
        })
        .fold(vec![vec![0; width]; height], |mut grid, (x, y)| {
            grid[y][x] = 1;
            grid
        });

    let start = (0, 0);
    let end = (width - 1, height - 1);

    let mut distances = vec![vec![std::u32::MAX; width]; height];
    distances[start.1][start.0] = 0;
    let mut visited = vec![vec![false; width]; height];
    let mut queue: BinaryHeap<Reverse<(u32, (usize, usize))>> = BinaryHeap::new();

    queue.push(Reverse((0, start)));

    while let Some(Reverse((distance, (x, y)))) = queue.pop() {
        visited[y][x] = true;

        for direction in [0, 1, 2, 3] {
            let d = match direction {
                0 if y != 0 => Some((x, y - 1)),
                1 if x + 1 != width => Some((x + 1, y)),
                2 if y + 1 != height => Some((x, y + 1)),
                3 if x != 0 => Some((x - 1, y)),
                _ => None,
            };

            if d.is_none() {
                continue;
            }

            let (x, y) = d.unwrap();

            if grid[y][x] == 1 {
                continue;
            }

            if visited[y][x] {
                continue;
            }

            let new_distance = distance + 1;
            if new_distance < distances[y][x] {
                distances[y][x] = new_distance;
                queue.push(Reverse((new_distance, (x, y))));
            }
        }
    }

    Some(distances[end.1][end.0])
}

fn solve(width: usize, height: usize, grid: Vec<Vec<i32>>) -> Option<u32> {
    let start = (0, 0);
    let end = (width - 1, height - 1);

    let mut distances = vec![vec![std::u32::MAX; width]; height];
    distances[start.1][start.0] = 0;
    let mut visited = vec![vec![false; width]; height];
    let mut queue: BinaryHeap<Reverse<(u32, (usize, usize))>> = BinaryHeap::new();

    queue.push(Reverse((0, start)));

    while let Some(Reverse((distance, (x, y)))) = queue.pop() {
        visited[y][x] = true;

        for direction in [0, 1, 2, 3] {
            let d = match direction {
                0 if y != 0 => Some((x, y - 1)),
                1 if x + 1 != width => Some((x + 1, y)),
                2 if y + 1 != height => Some((x, y + 1)),
                3 if x != 0 => Some((x - 1, y)),
                _ => None,
            };

            if d.is_none() {
                continue;
            }

            let (x, y) = d.unwrap();

            if grid[y][x] == 1 {
                continue;
            }

            if visited[y][x] {
                continue;
            }

            let new_distance = distance + 1;
            if new_distance < distances[y][x] {
                distances[y][x] = new_distance;
                queue.push(Reverse((new_distance, (x, y))));
            }
        }
    }

    Some(distances[end.1][end.0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_sized(
            &advent_of_code::template::read_file("examples", DAY),
            7,
            7,
            12,
        );
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two_sized(&advent_of_code::template::read_file("examples", DAY), 7, 7);
        assert_eq!(result, Some("6,1".to_string()));
    }
}
