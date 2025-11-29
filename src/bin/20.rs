use std::{collections::HashMap, ops::AddAssign};

use itertools::Itertools;

advent_of_code::solution!(20);

fn dijkstra(maze: &[bool], distances: &mut [i32], size: (i32, i32), end: i32) {
    assert!(maze.len() == distances.len());

    let mut queue = std::collections::BinaryHeap::new();
    let mut visited = vec![false; maze.len()];

    queue.push(std::cmp::Reverse((0i32, end)));
    distances[end as usize] = 0;

    while let Some(std::cmp::Reverse((distance, index))) = queue.pop() {
        visited[index as usize] = true;

        // println!("distance: {}, index: {}", distance, index);
        // for i in 0..maze.len() {
        //     if i % size.0 as usize == 0 {
        //         println!();
        //     }
        //     if visited[i] {
        //         print!("-");
        //     } else {
        //         print!("{}", if maze[i] { '#' } else { '.' });
        //     }
        // }
        // println!();

        let neighbors = [index + 1, index - 1, index + size.1, index - size.1];

        for neighbor_index in neighbors.into_iter() {
            if maze[neighbor_index as usize] {
                continue;
            }
            if visited[neighbor_index as usize] {
                continue;
            }
            if distances[neighbor_index as usize] > distance + 1 {
                distances[neighbor_index as usize] = distance + 1;
                queue.push(std::cmp::Reverse((distance + 1, neighbor_index)));
            }
        }
    }
}

fn parse(input: &str) -> (Vec<bool>, (i32, i32), i32, i32) {
    let mut maze = Vec::with_capacity(input.len());
    let mut size = (0, 0);

    let mut start = -1;
    let mut end = -1;

    for line in input.lines() {
        size.0 = line.len() as i32;
        for c in line.chars() {
            start = if c == 'S' { maze.len() as i32 } else { start };
            end = if c == 'E' { maze.len() as i32 } else { end };
            maze.push(c == '#');
        }
        size.1 += 1;
    }

    // println!("end: {}", end);
    assert!(start != -1 && end != -1);

    (maze, size, start, end)
}

pub fn part_one(input: &str) -> Option<i32> {
    Some(
        calculate_cheats(input, 2)
            .into_iter()
            .filter(|(k, _)| *k >= 100)
            .map(|(k, v)| v)
            .sum(),
    )
}
fn cheat_saving(c1: i32, c2: i32, d_start: &[i32], d_end: &[i32], start: i32) -> i32 {
    let post_cheat_distance = d_end[c2 as usize];
    let pre_cheat_distance = d_start[c1 as usize];
    let cheat_dist = post_cheat_distance + pre_cheat_distance + 2;
    let nocheat_dist = d_end[start as usize];
    nocheat_dist - cheat_dist
}

fn cheats(start: i32, len: usize, size: (i32, i32)) -> impl Iterator<Item = i32> {
    let len_limit = len as i32;
    let (startx, starty): (i32, i32) = (start % size.1, start / size.1);

    let xs = startx - len_limit..=startx + len_limit;
    let ys = starty - len_limit..=starty + len_limit;
    xs.cartesian_product(ys)
        .filter(move |(x, y)| {
            let length = startx.abs_diff(*x) + starty.abs_diff(*y);
            length <= len_limit as u32 && length > 0
        })
        .map(move |(x, y)| y * size.1 + x)
}

pub fn calculate_cheats(input: &str, cheat_len: usize) -> HashMap<i32, i32> {
    let (maze, size, start, end) = parse(input);

    let mut d_end = vec![std::i32::MAX; maze.len()];
    dijkstra(&maze, &mut d_end, size, end);

    let mut d_start = vec![std::i32::MAX; maze.len()];
    dijkstra(&maze, &mut d_start, size, start);

    // for each wall in maze
    let mut savings = HashMap::new();

    let width = size.1 as usize;
    let height = size.0 as usize;

    let valid = |pos| {
        if pos >= maze.len() {
            return false;
        }

        let x = pos % width;
        let y = pos / width;
        if x == 0 || x == width - 1 {
            return false;
        }
        if y == 0 || y == height - 1 {
            return false;
        }
        true
    };

    // cheat starts on that position
    for pos in 0..(maze.len()) {
        if !valid(pos) || maze[pos] {
            continue;
        }
        let cheat_start = pos as i32;
        // for each direction of cheat
        for cheat_end in cheats(cheat_start, cheat_len, size) {
            if !valid(cheat_end as usize) || maze[cheat_end as usize] {
                continue;
            }
            let saving = cheat_saving(cheat_start, cheat_end, &d_start, &d_end, start);

            if saving < 1 {
                continue;
            }

            let total = savings.entry(saving).or_default();
            *total += 1;
        }
        // calculate savings
        // accumulate count of cheats
    }

    savings
}

pub fn part_two(input: &str) -> Option<i32> {
    Some(
        calculate_cheats(input, 20)
            .into_iter()
            .filter(|(k, _)| *k >= 100)
            .map(|(_k, v)| v)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dijstra() {
        let (maze, size, start, end) = parse(&advent_of_code::template::read_file("examples", DAY));
        let mut d_start = vec![std::i32::MAX; maze.len()];
        dijkstra(&maze, &mut d_start, size, start);
        assert_eq!(d_start[end as usize], 84);
        assert_eq!(d_start[start as usize], 0);

        let mut d_end = vec![std::i32::MAX; maze.len()];
        dijkstra(&maze, &mut d_end, size, end);

        let c1 = start + 6 + size.1 * 4;
        let c2 = end;
        assert_eq!(cheat_saving(c1, c2, &d_start, &d_end, start), 64);

        let c1 = start + 7 + size.1 * 4;
        let c2 = c1 + size.1 * 2;
        assert_eq!(cheat_saving(c1, c2, &d_start, &d_end, start), 38);
    }

    #[test]
    fn test_cheats() {
        /*
        [.....] 0-4
        [.....] 5-9
        [..c..] 10-12-14
        [.....] 15-19
        [.....] 20-24
        */
        let cheats = cheats(2 + 5 + 5, 1, (5, 5));
        // cheats.for_each(|c| println!("{c}"));
        assert_eq!(cheats.count(), 4);
    }

    #[test]
    fn test_calculate_cheats() {
        let cheats = calculate_cheats(&advent_of_code::template::read_file("examples", DAY), 2);
        // dbg!(&cheats);
        assert_eq!(cheats.get(&2), Some(&14));
        assert_eq!(cheats.get(&36), Some(&1));
    }
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two_decomp() {
        let input: &str = &advent_of_code::template::read_file("examples", DAY);
        calculate_cheats(input, 20)
            .into_iter()
            .filter(|(k, _)| *k >= 50)
            .for_each(|(k, v)| println!("there are {v} cheats saving {k}"));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
