use std::{collections::HashMap, ops::AddAssign};

advent_of_code::solution!(20);

fn dijkstra(maze: &[bool], distances: &mut [i32], size: (i32, i32), end: i32) {
    assert!(maze.len() == distances.len());

    let mut queue = std::collections::BinaryHeap::new();
    let mut visited = vec![false; maze.len()];

    queue.push(std::cmp::Reverse((0i32, end)));

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

        let neighbors = [index+1, index-1, index + size.1, index - size.1];

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

    println!("end: {}", end);
    assert!(start != -1 && end != -1);

    (maze, size, start, end)
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(calculate_cheats(input).into_iter().filter(|(_, v)| *v >= 100).count().try_into().unwrap())
}

pub fn calculate_cheats(input: &str) -> HashMap<i32, i32> {
    let (maze, size, start, end) = parse(input);

    let mut distances_to_end = vec![std::i32::MAX; maze.len()];
    dijkstra(&maze, &mut distances_to_end, size, end);

    let mut distances_to_start = vec![std::i32::MAX; maze.len()];
    dijkstra(&maze, &mut distances_to_start, size, start);

    let original_length = distances_to_start[end as usize];

    let mut shortest_distance_to_start:Vec<i32> = Vec::new();
    let mut shortest_distance_to_end:Vec<i32> = Vec::new();
    for i in 0i32..maze.len() as i32 {
        // if on edge, skip
        let (x, y) = (i % size.0, i / size.0);
        if x == 0 || x == size.0 - 1 || y == 0 || y == size.1 - 1 {
            shortest_distance_to_start.push(i32::MAX);
            shortest_distance_to_end.push(i32::MAX);
            continue;
        }

        let neighbors = [i+1, i-1, i + size.1, i - size.1];
        let min_distance_to_start = neighbors.into_iter().map(|neighbor_index| distances_to_start[neighbor_index as usize]).min().unwrap();
        let min_distance_to_end = neighbors.into_iter().map(|neighbor_index| distances_to_end[neighbor_index as usize]).min().unwrap();
        shortest_distance_to_start.push(min_distance_to_start);
        shortest_distance_to_end.push(min_distance_to_end);
    }

    // for each adjacent pair of cells
    let mut savings = HashMap::new();
    for x in 1..size.0-2 {
        for y in 1..size.1-1 {
            let a = (y * size.0 + x) as usize;
            let b = a+1;

            let distance_to_start = shortest_distance_to_start[a].min(shortest_distance_to_start[b]);
            let distance_to_end = shortest_distance_to_end[a].min(shortest_distance_to_end[b]);

            if distance_to_start == i32::MAX || distance_to_end == i32::MAX {
                continue;
            }

            if distance_to_start + distance_to_end -3 > original_length {
                continue;
            }

            let saving = original_length - (distance_to_start + distance_to_end) -3;

            println!("ShortestDist to start: {distance_to_start} end: {distance_to_end} Diff: {saving}");
            for i in 0..maze.len() {
                if i % size.0 as usize == 0 {
                    println!();
                }
                if i == a {
                    print!("1");
                } else if i == b {
                    print!("2");
                } else if maze[i] {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();

            savings.entry(saving).or_insert(0).add_assign(1);
        }
    }


    savings
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {

        let (maze, size, start, end) = parse(&advent_of_code::template::read_file("examples", DAY));
        let mut distances = vec![std::i32::MAX; maze.len()];
        dijkstra(&maze, &mut distances, size, start);
        assert_eq!(distances[end as usize], 84);


        let cheats = calculate_cheats(&advent_of_code::template::read_file("examples", DAY));
        dbg!(&cheats);
        //assert_eq!(cheats.get(&2), Some(&14));
        //assert_eq!(cheats.get(&36), Some(&1));

        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
