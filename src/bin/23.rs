use std::{cmp::max, collections::{HashMap, HashSet}};

use itertools::Itertools;

advent_of_code::solution!(23);

fn parse_name(computer_name: &str) -> i32 {
    computer_name.chars().fold(0, |acc, c| (acc<<8) + (c as i32)-('a' as i32))
}

fn unparse_name(computer_name: i32) -> String {
    let mut s = Default::default();
    let mut c = computer_name;

    let chr = (c & 0xFF) + ('a' as i32);
    c = c>>8;
    s = format!("{}{}", chr as u8 as char, s);

    let chr = (c & 0xFF) + ('a' as i32);
    c = c>>8;
    s = format!("{}{}", chr as u8 as char, s);

    assert!(c == 0);

    s
}

fn starts_with_t(computer:i32) -> bool {
    unparse_name(computer).chars().next().unwrap() == 't'
}

pub fn part_one(input: &str) -> Option<u32> {

    let connections = input.lines().map(|line|{
        let parts = line.split("-").collect::<Vec<&str>>();
        let parts: Vec<i32> = parts.into_iter().map(parse_name).collect();
        (parts[0], parts[1])
    });

    let mut neighbours: HashMap<i32, Vec<i32>> = Default::default();
    for (from, to) in connections {
        neighbours.entry(from).or_default().push(to);
        neighbours.entry(to).or_default().push(from);
    }
    let neighbours = neighbours;
    let nodes: HashSet<i32> = neighbours.keys().cloned().collect();
    let blank = vec![];
    let n = |computer| {
        neighbours.get(computer).unwrap_or(&blank).as_slice()
    };

    let mut trios : HashSet<[i32;3]> = Default::default();
    // for each item in the network
    for node in nodes.iter() {
        // do the neigbours of my neihbours include node?
        let my_neighbours = n(node);
        for neighbour in my_neighbours {
            let my_neighbours_neighbours = n(neighbour);
            for neighbour_neighbour in my_neighbours_neighbours{
                let my_neighbours_neighbours_neighbours = n(neighbour_neighbour);
            if my_neighbours_neighbours_neighbours.contains(node) {
                let mut trio = [*node, *neighbour, *neighbour_neighbour];
                trio.sort();
                trios.insert(trio);
            }
            }
        }
    }
    let mut count = 0;
    for [a,b,c] in trios.iter() {
        if starts_with_t(*a) || starts_with_t(*b) || starts_with_t(*c) {
            count +=1;
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<String> {
    let connections = input.lines().map(|line|{
        let parts = line.split("-").collect::<Vec<&str>>();
        let parts: Vec<i32> = parts.into_iter().map(parse_name).collect();
        (parts[0], parts[1])
    });

    let mut neighbours: HashMap<i32, Vec<i32>> = Default::default();
    for (from, to) in connections {
        neighbours.entry(from).or_default().push(to);
        neighbours.entry(to).or_default().push(from);
    }
    let neighbours = neighbours;
    let nodes: HashSet<i32> = neighbours.keys().cloned().collect();
    let blank = vec![];
    let ns = |computer| {
        neighbours.get(computer).unwrap_or(&blank).as_slice()
    };

    let mut largest_party: HashSet<i32> = Default::default();
    // for each node := n
    for n in nodes.iter() {
        let n_ns = ns(n);
        // - if list of neighbors n smaller than largest party seen skip
        if n_ns.len() < largest_party.len() {
            continue
        }
        // - for each neighbor of n := nn
        for nn in n_ns {
            let nn_ns = ns(nn);
            // - - if list of neighbors nn smaller than largest party seen skip
            if nn_ns.len() < largest_party.len() {
                continue
            }
            // - - begin a lan party with n and nn
            let mut party : HashSet<i32> = Default::default();
            party.insert(*n);
            party.insert(*nn);

            // - - for each neighbour of nn := nnn
            for nnn in nn_ns {
                // - - - if nnn connected to all nodes in lan party
                let nnn_ns = ns(nnn);
                if nnn_ns.len() < party.len() {
                    continue
                }
                let nnn_ns_set = nnn_ns.iter().cloned().collect();
                if party.is_subset(&nnn_ns_set) {
                    // - - - add nnn to lan party
                    party.insert(*nnn);
                }
            }

            // - - update largest seen party
            if party.len()> largest_party.len() {
                largest_party = party;
            }
        }
    }
    // convert node names to strings
    // sort
    // join with ','
    Some(largest_party.into_iter().map(unparse_name).sorted().join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert!(!starts_with_t(parse_name("ab")));
        assert!(starts_with_t(parse_name("tb")));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}
