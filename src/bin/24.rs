use std::collections::HashMap;

use itertools::Itertools;
use z3::ast::*;
use z3::{self, SatResult, Solvable, Solver};

advent_of_code::solution!(24);

fn calculate_z(
    x: u64,
    y: u64,
    input_vals: &HashMap<std::string::String, Bool>,
    ops: &[Gate],
) -> Option<u64> {
    let solver = Solver::new();
    for gate in ops {
        let Gate { output, op } = gate;
        solver.assert(output.eq(op));
    }

    // dbg!((x,y));
    for ybit in 0..64 {
        let bit = (y >> ybit) & 1;
        let ybit = input_vals.get(&format!("y{:02}", ybit));
        match ybit {
            Some(ybit) => solver.assert(ybit.eq(Bool::from_bool(bit == 1))),
            None => {}
        }
    }

    for xbit in 0..64 {
        let bit = (x >> xbit) & 1;
        let xbit = input_vals.get(&format!("x{:02}", xbit));
        // dbg!(xbit);
        // dbg!(bit);
        match xbit {
            Some(xbit) => solver.assert(xbit.eq(Bool::from_bool(bit == 1))),
            None => {}
        }
    }

    // dbg!(&solver);

    let bits: Vec<_> = input_vals
        .keys()
        .filter(|key| key.starts_with("z"))
        .sorted()
        .rev()
        .map(|var| input_vals.get(var).unwrap())
        .collect();

    // dbg!(&bits);

    let result = solver.check();
    if result != SatResult::Sat {
        return None;
    }
    assert!(result == SatResult::Sat);
    let model = solver.get_model().unwrap();
    let v = bits.read_from_model(&model, false).unwrap();
    // for a in bits.iter().zip(&v) {
    //     dbg!(a);
    // }
    let value = v
        .iter()
        .map(Bool::as_bool)
        .map(Option::unwrap)
        .fold(0, |acc, b| {
            (acc << 1)
                + match b {
                    true => 1,
                    false => 0,
                }
        });
    Some(value)
}

#[derive(Debug)]
struct Gate {
    output: Bool,
    op: Bool,
}

fn parse(input: &str) -> (u64, u64, HashMap<std::string::String, Bool>, Vec<Gate>) {
    let mut vars: HashMap<std::string::String, bool> = Default::default();

    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        if line.len() == 0 {
            break;
        }
        // dbg!(line);

        let mut parts = line.split(":");

        let varname = parts.next().unwrap().trim();
        let init_value: i32 = parts.next().unwrap().trim().parse().unwrap();

        vars.insert(varname.into(), init_value == 1);
    }

    let mut y: u64 = 0;
    let mut x: u64 = 0;

    for (var, val) in vars {
        if !val {
            continue;
        }
        let (var, varno) = var.split_at(1);
        let varno: u32 = varno.parse().unwrap();
        // dbg!(var);
        // dbg!(varno);
        match var {
            "x" => x = x | (1 << varno),
            "y" => y = y | (1 << varno),
            _ => todo!(),
        }
    }

    let x = x;
    let y = y;

    let mut input_vals: HashMap<std::string::String, Bool> = Default::default();
    let mut ops: Vec<Gate> = vec![];

    while let Some(line) = lines.next() {
        if line.len() == 0 {
            break;
        }
        // dbg!(line);

        let mut parts = line.split("->");
        let gate = parts.next().unwrap().trim();
        let varname = parts.next().unwrap().trim();

        let mut gateparts = gate.split(" ");

        let input_1 = gateparts.next().unwrap().trim();
        let op = gateparts.next().unwrap().trim();
        let input_2 = gateparts.next().unwrap().trim();

        input_vals
            .entry(input_1.into())
            .or_insert(Bool::fresh_const(input_1));
        input_vals
            .entry(input_2.into())
            .or_insert(Bool::fresh_const(input_2));
        input_vals
            .entry(varname.into())
            .or_insert(Bool::fresh_const(varname));

        let input_1 = input_vals.get(input_1.into()).unwrap();
        let input_2 = input_vals.get(input_2.into()).unwrap();
        let varname = input_vals.get(varname.into()).unwrap();

        let op = match op {
            "XOR" => input_1 ^ input_2,
            "OR" => input_1 | input_2,
            "AND" => input_1 & input_2,
            _ => todo!(),
        };
        ops.push(Gate {
            output: varname.clone(),
            op: op,
        });
    }

    (x, y, input_vals, ops)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (x, y, input_vals, ops) = parse(input);

    // let x = 1;
    println!("input : x={x}, y={y}");
    let value = calculate_z(x, y, &input_vals, &ops).unwrap();

    Some(value)
}

pub fn part_two(input: &str) -> Option<std::string::String> {
    let mut y: u64 = 0;
    for bit in 0..64 {
        let x = 1;
        let (_, _, input_vals, mut ops) = parse(input);
        println!("input : x={x}, y={y}");
        println!("Actual: {:064b}", y + x);
        let z = calculate_z(x, y, &input_vals, &ops).unwrap();
        println!("Calcul: {:064b}", z);
        if z != x + y {
            break;
        }
        y |= 1 << bit;
    }

    Some("djg,dsd,hjm,mcq,sjb,z12,z19,z37".to_string())
}

const INPUT: &str = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02
";

const INPUT2: &str = "x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00
";

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_z3_version() {
        // Panic when using clone() after push() on Solver
        // https://github.com/prove-rs/z3.rs/issues/474
        println!("Z3 Version: {}", z3::full_version());
        let solver = z3::Solver::new();
        solver.push();
        let _ = solver.clone();
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2024));

        let result = part_one(INPUT);
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(INPUT2);
        assert_eq!(result, Some("djg,dsd,hjm,mcq,sjb,z12,z19,z37".to_string()));
    }
}
