advent_of_code::solution!(19);

enum TowelColor {
    White,
    Blue,
    Black,
    Red,
    Green,
}

impl TryFrom<char> for TowelColor {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'w' => Ok(TowelColor::White),
            'u' => Ok(TowelColor::Blue),
            'b' => Ok(TowelColor::Black),
            'r' => Ok(TowelColor::Red),
            'g' => Ok(TowelColor::Green),
            _ => Err(()),
        }
    }
}

struct Trie {
    white: Option<Box<Trie>>,
    blue: Option<Box<Trie>>,
    black: Option<Box<Trie>>,
    red: Option<Box<Trie>>,
    green: Option<Box<Trie>>,

    present: bool,
}

impl Trie {
    fn new() -> Self {
        Self {
            white: None,
            blue: None,
            black: None,
            red: None,
            green: None,
            present: false,
        }
    }

    fn insert(&mut self, s: &str) {
        let mut node = self;
        for c in s.chars() {
            let color = TowelColor::try_from(c).unwrap();
            node = match color {
                TowelColor::White => node.white.get_or_insert(Box::new(Trie::new())),
                TowelColor::Blue => node.blue.get_or_insert(Box::new(Trie::new())),
                TowelColor::Black => node.black.get_or_insert(Box::new(Trie::new())),
                TowelColor::Red => node.red.get_or_insert(Box::new(Trie::new())),
                TowelColor::Green => node.green.get_or_insert(Box::new(Trie::new())),
            };
        }
        node.present = true;
    }

    fn all_prefix_matches<'a>(&self, s: &'a str) -> Vec<&'a str> {
        let mut node = self;
        let mut matches = Vec::new();
        for (len, c) in s.chars().enumerate() {
            let color = TowelColor::try_from(c).unwrap();
            let maybe_node = match color {
                TowelColor::White => node.white.as_ref(),
                TowelColor::Blue => node.blue.as_ref(),
                TowelColor::Black => node.black.as_ref(),
                TowelColor::Red => node.red.as_ref(),
                TowelColor::Green => node.green.as_ref(),
            };

            if let Some(next_node) = maybe_node {
                node = next_node;

                if node.present {
                    matches.push(s[..len + 1].as_ref());
                }
            } else {
                break;
            }
        }
        matches
    }
}

fn is_possible(s: &str, trie: &Trie) -> bool {
    count_possible(s, trie) != 0
}

fn count_possible(s: &str, trie: &Trie) -> i64 {
    let mut score = vec![0; s.len() + 1];

    score[0] = 1;

    for idx in 0..s.chars().count() {
        for prefix in trie.all_prefix_matches(&s[idx..]) {
            if idx + prefix.len() > s.len() {
                continue;
            }
            score[idx + prefix.len()] += score[idx]
        }
    }

    score[s.len()]
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut trie = Trie::new();
    for line in input.lines().take_while(|line| !line.is_empty()) {
        for bit in line.split(",") {
            trie.insert(bit.trim());
        }
    }

    let mut count = 0;
    for line in input.lines().skip_while(|line| !line.is_empty()).skip(1) {
        if is_possible(line, &trie) {
            count += 1;
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut trie = Trie::new();
    for line in input.lines().take_while(|line| !line.is_empty()) {
        for bit in line.split(",") {
            trie.insert(bit.trim());
        }
    }

    let mut count = 0;
    for line in input.lines().skip_while(|line| !line.is_empty()).skip(1) {
        count += count_possible(line, &trie);
    }
    Some(count.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
