use std::collections::HashSet;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u32> {
    let plants: Vec<char> = input.lines().flat_map(|l| l.chars()).collect();
    let size: (i32, i32) = (
        input.lines().next().unwrap().len().try_into().unwrap(),
        input.lines().count().try_into().unwrap(),
    );

    let mut visited: HashSet<i32> = HashSet::new();
    let mut regions: Vec<(char, (i32, i32))> = Vec::new();
    for (idx, _) in (0i32..).zip(&plants) {
        let r = visit(&mut visited, idx, size, &plants[..]);
        if r != (0, 0) {
            regions.push((*plants.get(idx as usize).unwrap(), r));
        }
    }

    // dbg!(&regions);

    Some(
        regions
            .iter()
            .map(|(_region, (region_size, perimiter))| region_size * perimiter)
            .sum::<i32>()
            .try_into()
            .unwrap(),
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn visit(visited: &mut HashSet<i32>, idx: i32, size: (i32, i32), plants: &[char]) -> (i32, i32) {
    if visited.contains(&idx) {
        return (0, 0);
    }

    visited.insert(idx);

    let plant = *plants.get(idx as usize).unwrap();

    let x = idx % size.0;
    let y = idx / size.0;

    // collect neighbors
    let mut neighbors = Vec::new();

    // has left neighbor
    let has_left = x > 0;
    if has_left {
        let left = idx - 1;
        neighbors.push(left);
    }

    // has right neighbor
    let has_right = x < size.0 - 1;
    if has_right {
        let right = idx + 1;
        neighbors.push(right);
    }

    // has top neighbor
    let has_top = y > 0;
    if has_top {
        let top = idx - size.0;
        neighbors.push(top);
    }

    // has bottom neighbor
    let has_bottom = y < size.1 - 1;
    if has_bottom {
        let bottom = idx + size.0;
        neighbors.push(bottom);
    }

    let is_same_plant = |n: &i32| *plants.get(*n as usize).unwrap() == plant;
    let not_same_plant = |n: &i32| !is_same_plant(n);
    // let is_already_visited = |n: &i32| visited.contains(n);
    //let not_already_visited = |n: &i32| !is_already_visited(n);

    // count perimiter if on edge of 'plant'
    let mut perimiter: i32 = neighbors
        .iter()
        .cloned()
        .filter(not_same_plant)
        .count()
        .try_into()
        .unwrap();

    // count edge of grid
    if !has_left {
        perimiter += 1;
    }
    if !has_right {
        perimiter += 1;
    }
    if !has_top {
        perimiter += 1;
    }
    if !has_bottom {
        perimiter += 1;
    }

    // count region size
    let mut area = 1;

    // visit neighbors
    for neighbor in neighbors.iter().cloned().filter(is_same_plant) {
        let (a, p) = visit(visited, neighbor, size, plants);
        area += a;
        perimiter += p;
    }

    (area, perimiter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
