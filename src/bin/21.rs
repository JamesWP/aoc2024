use std::collections::HashSet;

use memoize::memoize;

advent_of_code::solution!(21);

fn arrow_location(arrow: char) -> Option<(i32, i32)> {
    //        +---+---+
    //        | ^ | A |
    //    +---+---+---+
    //    | < | v | > |
    //    +---+---+---+

    Some(match arrow {
        '^' => (1, 0),
        'A' => (2, 0),
        '<' => (0, 1),
        'v' => (1, 1),
        '>' => (2, 1),
        _ => return None,
    })
}

fn valid_arrow_location(x: i32, y: i32) -> bool {
    if y == 0 {
        x == 1 || x == 2
    } else if y == 1 {
        x == 0 || x == 1 || x == 2
    } else {
        false
    }
}

fn num_location(num: char) -> Option<(i32, i32)> {
    // +---+---+---+
    // | 7 | 8 | 9 |
    // +---+---+---+
    // | 4 | 5 | 6 |
    // +---+---+---+
    // | 1 | 2 | 3 |
    // +---+---+---+
    //     | 0 | A |
    //     +---+---+

    Some(match num {
        '7' => (0, 0),
        '8' => (1, 0),
        '9' => (2, 0),
        '4' => (0, 1),
        '5' => (1, 1),
        '6' => (2, 1),
        '1' => (0, 2),
        '2' => (1, 2),
        '3' => (2, 2),
        '0' => (1, 3),
        'A' => (2, 3),
        _ => return None,
    })
}

fn valid_num_location(x: i32, y: i32) -> bool {
    if y == 0 || y == 1 || y == 2 {
        x == 0 || x == 1 || x == 2
    } else if y == 3 {
        x == 1 || x == 2
    } else {
        false
    }
}

#[memoize]
fn arrow_path_coord(start: (i32, i32), finish: (i32, i32)) -> HashSet<String> {
    // compute all paths from start to finish

    if !valid_arrow_location(start.0, start.1) {
        return HashSet::new();
    }

    if start == finish {
        return HashSet::from_iter(["A".to_string()]);
    }

    let mut options = HashSet::new();

    // path where horizontal is preferred
    if start.0 != finish.0 {
        let dir = if start.0 < finish.0 { 1 } else { -1 };

        let paths = arrow_path_coord((start.0 + dir, start.1), finish);

        let paths = paths.into_iter().map(|path| {
            let movement = if start.0 < finish.0 { ">" } else { "<" };
            format!("{}{}", movement, path)
        });

        options.extend(paths);
    }

    // path where vertical is preferred
    if start.1 != finish.1 {
        let dir = if start.1 < finish.1 { 1 } else { -1 };

        let paths = arrow_path_coord((start.0, start.1 + dir), finish);

        let paths = paths.into_iter().map(|path| {
            let movement = if start.1 < finish.1 { "v" } else { "^" };
            format!("{}{}", movement, path)
        });

        options.extend(paths);
    }

    options
}

#[memoize]
fn numeric_path_coord(start: (i32, i32), finish: (i32, i32)) -> HashSet<String> {
    // compute all paths from start to finish

    if !valid_num_location(start.0, start.1) {
        return HashSet::new();
    }

    if start == finish {
        return HashSet::from_iter(["A".to_string()]);
    }

    let mut options = HashSet::new();

    // path where horizontal is preferred
    if start.0 != finish.0 {
        let dir = if start.0 < finish.0 { 1 } else { -1 };

        let paths = numeric_path_coord((start.0 + dir, start.1), finish);

        let paths = paths.into_iter().map(|path| {
            let movement = if start.0 < finish.0 { ">" } else { "<" };
            format!("{}{}", movement, path)
        });

        options.extend(paths);
    }

    // path where vertical is preferred
    if start.1 != finish.1 {
        let dir = if start.1 < finish.1 { 1 } else { -1 };

        let paths = numeric_path_coord((start.0, start.1 + dir), finish);

        let paths = paths.into_iter().map(|path| {
            let movement = if start.1 < finish.1 { "v" } else { "^" };
            format!("{}{}", movement, path)
        });

        options.extend(paths);
    }

    options
}

fn arrow_path(start: char, finish: char) -> HashSet<String> {
    arrow_path_coord(arrow_location(start).unwrap(), arrow_location(finish).unwrap())
}

fn numeric_path(start: char, finish: char) -> HashSet<String> {
    numeric_path_coord(num_location(start).unwrap(), num_location(finish).unwrap())
}


#[memoize]
fn path_length(code: String, level: usize, start_level: usize) -> i64 {
    if level==0 {
        return 1;
    }

    let mut tot = 0;

    let mut last = 'A';
    for elem in code.chars() {
        let paths = if level == start_level {
            numeric_path(last, elem)
        } else {
            arrow_path(last, elem)
        };

        let smallest_path_length = paths.into_iter().map(|p| path_length(p, level-1, start_level)).min().unwrap();

        tot += smallest_path_length;

        last = elem;
    }

    tot
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(|line| {
        let num: i64 = line[0..line.len()-1].parse().unwrap();
        print!("{}: ({})", line, num);
        let length = path_length(line.to_string(), 4, 4);
        println!("len: {}", length);
        length * num
    }).sum::<i64>().try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(input.lines().map(|line| {
        let num: i64 = line[0..line.len()-1].parse().unwrap();
        print!("{}: ({})", line, num);
        let length = path_length(line.to_string(), 27, 27);
        println!("len: {}", length);
        length * num
    }).sum::<i64>().try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arrow_location() {
        assert_eq!(arrow_location('^'), Some((1, 0)));
        assert_eq!(arrow_location('A'), Some((2, 0)));
        assert_eq!(arrow_location('<'), Some((0, 1)));
        assert_eq!(arrow_location('v'), Some((1, 1)));
        assert_eq!(arrow_location('>'), Some((2, 1)));
    }

    #[test]
    fn test_arrow_path() {
        assert!(arrow_path('^', 'v').contains(&"vA".to_string()));
        assert!(arrow_path('^', '>').contains(&">vA".to_string()));
        assert!(arrow_path('^', '<').contains(&"v<A".to_string()));
        assert!(arrow_path('^', 'A').contains(&">A".to_string()));
        assert!(arrow_path('A', '^').contains(&"<A".to_string()));
        assert!(arrow_path('A', 'v').contains(&"v<A".to_string()));
        assert!(arrow_path('A', '>').contains(&"vA".to_string()));
        assert!(arrow_path('A', '<').contains(&"v<<A".to_string()));
        assert!(arrow_path('A', 'A').contains(&"A".to_string()));
    }

    #[test]
    fn test_num_path() {
        assert!(numeric_path('1', '0').contains(&">vA".to_string()));
        assert!(numeric_path('0', '1').contains(&"^<A".to_string()));
        assert!(numeric_path('A', '4').contains(&"^^<<A".to_string()));
        assert!(numeric_path('4', 'A').contains(&">>vvA".to_string()));
        assert!(numeric_path('4', 'A').contains(&"v>>vA".to_string()));
        assert!(numeric_path('4', 'A').contains(&">v>vA".to_string()));
        assert!(numeric_path('8', '9').contains(&">A".to_string()));
        assert!(numeric_path('3', '9').contains(&"^^A".to_string()));
    }

    #[test]
    fn test_example() {
        assert_eq!(path_length("029A", 4, 4), 68);
        assert_eq!(path_length("379A", 4, 4), 64);
    }
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
