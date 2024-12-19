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
        let size = (input.lines().count() as i32, input.lines().next().unwrap().len() as i32);
        let start = data.iter().position(|&c| c == 'S').unwrap() as i32;
        let end = data.iter().position(|&c| c == 'E').unwrap() as i32;
        let data = data.iter().map(|&c| c != '#').collect::<Vec<_>>();
        Maze { data, size, start, end }
    }
}

impl Maze {
    fn shortest_distance(&self) -> i32 {
        // Using recursive DFS walk through the maze, keeping track of the shortest distance found

        // set best distance to max value
        let mut best_distance = vec![i32::MAX; self.data.len()];

        self.shortest_distance_recursive(self.start, 0, 1, &mut best_distance, &mut Vec::new())
    }

    fn shortest_distance_recursive(&self, position: i32, distance: i32, direction: u8, best_distance: &mut Vec<i32>, visited: &mut Vec<i32>) -> i32 {
        // Check if we have reached the end of the maze
        if position == self.end {
            println!("Found end of maze, distance: {}", distance);
            return distance;
        }

        // Check if we have visited this position before
        if visited.contains(&position) {
            return i32::MAX;
        }

        // Check if we have found a shorter path to this position
        if distance >= best_distance[position as usize] {
            return i32::MAX;
        } else {
            best_distance[position as usize] = distance;
        }

        // Mark this position as visited
        visited.push(position);

        // Check if we can move up
        let up_position = (position - self.size.1) as usize;
        let up = if self.data[up_position] {
            let cost = if direction == 0 { 1 } else { 1001 };
            self.shortest_distance_recursive(up_position as i32, distance + cost, 0, best_distance, visited)
        } else {
            i32::MAX
        };

        // Check if we can move right
        let right_position = (position + 1) as usize;
        let right = if self.data[right_position] {
            let cost = if direction == 1 { 1 } else { 1001 };
            self.shortest_distance_recursive(right_position as i32, distance + cost, 1, best_distance, visited)
        } else {
            i32::MAX
        };

        // Check if we can move down
        let down_position = (position + self.size.1) as usize;
        let down = if self.data[down_position] {
            let cost = if direction == 2 { 1 } else { 1001 };
            self.shortest_distance_recursive(down_position as i32, distance + cost, 2, best_distance, visited)
        } else {
            i32::MAX
        };

        // Check if we can move left
        let left_position = (position - 1) as usize;
        let left = if self.data[left_position] {
            let cost = if direction == 3 { 1 } else { 1001 };
            self.shortest_distance_recursive(left_position as i32, distance + cost, 3, best_distance, visited)
        } else {
            i32::MAX
        };

        // Unmark this position as visited
        visited.pop();

        // Return the shortest distance found
        up.min(right).min(down).min(left)
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
        assert_eq!(result, None);
    }
}
