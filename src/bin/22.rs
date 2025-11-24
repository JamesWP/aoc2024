use std::{
    collections::{HashMap, HashSet},
    default,
    ops::DerefMut,
    slice::Windows,
};

advent_of_code::solution!(22);

fn prune(input: u64) -> u64 {
    input % 16777216
}

fn mix(left: u64, right: u64) -> u64 {
    left ^ right
}

fn generate_next_secret(old_secret: u64, _idx: u64) -> u64 {
    let r1 = prune(mix(old_secret * 64, old_secret));
    let r2 = prune(mix(r1 / 32, r1));
    let r3 = prune(mix(r2 * 2048, r2));

    r3
}

fn get_2000th_secret(starting_secret: u64) -> u64 {
    let nums = 0..2000_u64;
    let mut secret = starting_secret;
    // print!("Secret Starting: {:8}", starting_secret);
    nums.for_each(|num| {
        // let old_secret = secret;
        secret = generate_next_secret(secret, num);
        // if num < 100 {print!(", {:2}", ((secret as i64%10) - (old_secret as i64%10))); }
    });
    //println!();

    secret
}

pub fn part_one(input: &str) -> Option<u64> {
    let nums = input
        .lines()
        .map(str::parse::<u64>)
        .map(Result::unwrap)
        .map(get_2000th_secret);
    Some(nums.sum())
}

fn sequence<'a>(secret: u64) -> Vec<(u32, u8)> {
    let mut sequence = [0; 2000];
    let mut sequence_deltas = [0_i8; 1999];
    sequence[0] = secret as i64;
    for idx in 1..2000 {
        sequence[idx] = generate_next_secret(sequence[idx - 1] as u64, idx as u64) as i64;
        let delta = (sequence[idx] % 10) - (sequence[idx - 1] % 10);
        sequence_deltas[idx - 1] = (delta + 9) as i8;
    }

    let mut seen: HashSet<u32> = Default::default();
    let mut deltas = vec![];

    let prices = sequence.iter().skip(4).map(|x| (x % 10) as u8);

    let windows: Vec<u32> = sequence_deltas
        .windows(4)
        .map(|window| {
            let a = window[0] as i32;
            let b = window[1] as i32;
            let c = window[2] as i32;
            let d = window[3] as i32;
            let score = a << (8 + 8 + 8) | b << (8 + 8) | c << 8 | d;
            // if a==-2+9&&b==1+9&&c==-1+9&&d==3+9 {
            // println!("Scoring: {a}, {b}, {c}, {d} = {score}");
            // }
            score as u32
        })
        .collect();

    for (window, price) in windows.into_iter().zip(prices) {
        if !seen.contains(&window) {
            seen.insert(window);
            deltas.push((window, price));
            // if window ==0x70a080c {
            // println!("Scoring: {window} = {price}");
            // }
        }
    }

    deltas
}
/*
global_dict := map last4 -> bananas

for each number
    generate the sequence including only first occurence
    walk over the sequence
    increment global_dict entry by bananas


find dict entry with highest count
    [-1,-3,5,6]: 12
*/

pub fn part_two(input: &str) -> Option<u32> {
    let nums: Vec<u64> = input
        .lines()
        .map(str::parse::<u64>)
        .map(Result::unwrap)
        .collect();

    // dbg!(&nums);

    let mut delta_score: HashMap<u32, u32> = Default::default();

    for num in nums {
        for (deltas, bananas) in sequence(num) {
            let val = delta_score.entry(deltas).or_insert(0);
            *val += bananas as u32;
        }
    }
    for score in delta_score.keys() {
        // println!("{:08x} = {}", score, delta_score.get(score).unwrap_or(&0));
    }
    // println!("Val: {}", delta_score.get(&0x70a080c).unwrap_or(&0));
    delta_score.values().max().copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_123() {
        // part_one("1\n2\n3\n2024");
        let a = sequence(2024);

        for (a, b) in a {
            // println!("Val: {}", delta_score.get(&0x70a080c).unwrap_or(&0));
            if a == 0x70a080c {
                // println!("B: {b}");
            }
        }
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two("1\n2\n3\n2024\n");
        assert_eq!(result, Some(23));
    }
}
