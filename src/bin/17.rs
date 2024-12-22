use std::ptr::addr_of_mut;

advent_of_code::solution!(17);

struct Computer {
    register_a: i64,
    register_b: i64,
    register_c: i64,

    output: Vec<i64>,
}

impl Computer {

    fn combo(&self, n: i64) -> i64 {
        match n {
            0..=3 => n,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => panic!("Invalid combo arg"),
        }
    }
    fn output_string(&self) -> String {
        self.output.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",")
    }
    fn run_test_program() -> String {
        /*
            Register A: 729
            Register B: 0
            Register C: 0

            Program: 0,1,5,4,3,0
        */
        let mut computer = Computer {
            register_a: 729,
            register_b: 0,
            register_c: 0,

            output: Vec::new(),
        };

        loop {
            computer.register_a /= 2i64.pow(computer.combo(1).try_into().unwrap());
            computer.output.push(computer.combo(4).rem_euclid(8));
            if computer.register_a != 0 {
                continue;
            }
            break;
        }

        computer.output_string()
    }

    fn run_program() -> String {
        // Register A: 64196994
        // Register B: 0
        // Register C: 0

        // Program: 2,4,1,1,7,5,1,5,4,0,0,3,5,5,3,0
        // Original output: 6,4,6,0,4,5,7,2,7

        let mut register_a = 64196994;

        let mut output = Vec::new();

        loop {
            // 2,4 | bst combo:4 | b = combo(4)%8 | b = a%8 | b = a&0x7 | b = a&111
            let mut register_b = register_a & 0x7;

            // 1,1 | bxl literal:1 | b = b^1
            register_b ^= 1;

            // 7,5 | cdv combo:5 | c = a/(2^combo(5)) | c = a/(2^b) | c = a >> b
            let register_c = register_a >> register_b;

            // 1,5 | bxl literal:5 | b = b^5
            register_b ^= 5;

            // 4,0 | bxc ignored:0 | b = b^c
            register_b ^= register_c;

            // 0,3 | adv combo:3 | a = a/(2^combo(3)) | a = a/(2^3) | a = a/8 | a /= 8 | a = a>>3 | a >>= 3
            register_a >>= 3;

            // 5,5 | out combo:5 | output b%8 | output b&0x7 | output b&111
            output.push(register_b & 0x7);

            // 3,0 | jnz literal:0 | if a!=0 goto 0
            if register_a != 0 {
                continue;
            }
            break;
        }

        output.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",")
    }
}

// Nope 6,3,4,3,6,3,0,3,4,3,5,3,7,3,2,3,7,3

pub fn part_one(input: &str) -> Option<String> {
    Some(Computer::run_program())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(Computer::run_test_program(), "4,6,3,5,6,3,5,2,1,0".to_string());
        assert_eq!(Computer::run_program(), "6,4,6,0,4,5,7,2,7".to_string());
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
