use std::fs::File;
use std::io::prelude::*;
use std::iter::repeat;

#[test]
fn test_parse_op() {
    assert_eq!(parse_op(1002), (vec![false, true, false], 2));
    assert_eq!(parse_op(11004), (vec![false, true, true], 4));
    assert_eq!(parse_op(10102), (vec![true, false, true], 2));
    assert_eq!(parse_op(103), (vec![true, false, false], 3));
    assert_eq!(parse_op(2), (vec![false, false, false], 2));
    assert_eq!(parse_op(99), (vec![false, false, false], 99));
}

#[test]
fn test_resolve_flags() {
    let program = vec![1102, 30, 40, 5, 3, 0, 2];
    let (flags, opcode) = parse_op(program[0]);
    let arg0 = resolve_flags(&program, 0, 0, &flags);
    let arg1 = resolve_flags(&program, 0, 1, &flags);
    assert_eq!(opcode, 2);
    assert_eq!(arg0, 30);
    assert_eq!(arg1, 40);
    let (flags, opcode) = parse_op(program[4]);
    assert_eq!(opcode, 3);
    let arg0 = resolve_flags(&program, 4, 0, &flags);
    let arg1 = resolve_flags(&program, 4, 1, &flags);
    assert_eq!(arg0, 1102);
    assert_eq!(arg1, 40);
}

#[test]
fn test_execute() {
    let equal_to_8 = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
    assert_eq!(execute(&equal_to_8, &vec![8]), vec![1]);
    assert_eq!(execute(&equal_to_8, &vec![10]), vec![0]);
    let less_than_8 = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
    assert_eq!(execute(&less_than_8, &vec![5]), vec![1]);
    assert_eq!(execute(&less_than_8, &vec![20]), vec![0]);

    let equal_to_8_immediate = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
    assert_eq!(execute(&equal_to_8_immediate, &vec![8]), vec![1]);
    assert_eq!(execute(&equal_to_8_immediate, &vec![10]), vec![0]);

    let less_than_8_immediate = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
    assert_eq!(execute(&less_than_8_immediate, &vec![5]), vec![1]);
    assert_eq!(execute(&less_than_8_immediate, &vec![20]), vec![0]);
}

#[test]
fn test_execute_jump() {
    let non_zero = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
    assert_eq!(execute(&non_zero, &vec![8]), vec![1]);
    assert_eq!(execute(&non_zero, &vec![0]), vec![0]);

    let non_zero_immediate = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
    assert_eq!(execute(&non_zero_immediate, &vec![8]), vec![1]);
    assert_eq!(execute(&non_zero_immediate, &vec![0]), vec![0]);
}

#[test]
fn test_execute_complex() {
    let complex = vec![
        3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
        1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20,
        1105, 1, 46, 98, 99,
    ];
    assert_eq!(execute(&complex, &vec![8]), vec![1000]);
    assert_eq!(execute(&complex, &vec![18]), vec![1001]);
    assert_eq!(execute(&complex, &vec![3]), vec![999]);
}

fn main() {
    let mut file = File::open("input/5.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents = contents.trim().to_string();
    let program: Vec<_> = contents
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let input = vec![5];
    let output = execute(&program, &input);
    println!("{:?}", output);
}

fn parse_op(op: i64) -> (Vec<bool>, i64) {
    let mut flags: Vec<bool> = op
        .to_string()
        .chars()
        .rev()
        .chain(repeat('0'))
        .skip(2)
        .take(3)
        .map(|x| match x {
            '0' => false,
            '1' => true,
            _ => panic!("Parse error in flags {}", op),
        })
        .collect();
    let op_str = op.to_string();
    let opcode: i64 = if op_str.len() == 1 {
        op
    } else {
        op_str[op_str.len() - 2..].parse().unwrap()
    };
    (flags, opcode)
}

fn resolve_flags(program: &Vec<i64>, i: usize, argno: i64, flags: &Vec<bool>) -> i64 {
    let argno = argno as usize;
    if flags[argno] {
        program[i + argno + 1]
    } else {
        program[program[i + argno + 1] as usize]
    }
}

fn execute(program: &Vec<i64>, input: &Vec<i64>) -> Vec<i64> {
    let mut program = program.clone();
    let mut input = input.iter();
    let mut output = vec![];
    let mut pc = 0;
    let mut kill_count = 0;
    let kill_limit = 1_000;
    loop {
        kill_count += 1;
        if kill_count > kill_limit {
            println!("{:?}", program);
            panic!("kill limit exceeded!")
        }
        let op = program[pc];
        let (flags, opcode) = parse_op(op);
        if opcode == 99 {
            break;
        };
        match opcode {
            1 => {
                let arg0 = resolve_flags(&program, pc, 0, &flags);
                let arg1 = resolve_flags(&program, pc, 1, &flags);
                let arg2 = program[pc + 3] as usize;
                program[arg2] = arg0 + arg1;
                pc += 4
            }
            2 => {
                let arg0 = resolve_flags(&program, pc, 0, &flags);
                let arg1 = resolve_flags(&program, pc, 1, &flags);
                let arg2 = program[pc + 3] as usize;
                program[arg2] = arg0 * arg1;
                pc += 4
            }
            3 => {
                let arg0 = program[pc + 1] as usize;
                program[arg0] = *input
                    .next()
                    .expect("Tried to get input but none remaining!");
                pc += 2
            }
            4 => {
                let arg0 = resolve_flags(&program, pc, 0, &flags);
                output.push(arg0 as i64);
                pc += 2
            }
            5 => {
                let arg0 = resolve_flags(&program, pc, 0, &flags);
                let arg1 = resolve_flags(&program, pc, 1, &flags);
                if arg0 != 0 {
                    pc = arg1 as usize;
                } else {
                    pc += 3;
                }
            }
            6 => {
                let arg0 = resolve_flags(&program, pc, 0, &flags);
                let arg1 = resolve_flags(&program, pc, 1, &flags);
                if arg0 == 0 {
                    pc = arg1 as usize;
                } else {
                    pc += 3;
                }
            }

            7 => {
                let arg0 = resolve_flags(&program, pc, 0, &flags);
                let arg1 = resolve_flags(&program, pc, 1, &flags);
                let arg2 = program[pc + 3] as usize;
                if arg0 < arg1 {
                    program[arg2] = 1;
                } else {
                    program[arg2] = 0;
                }
                pc += 4
            }
            8 => {
                let arg0 = resolve_flags(&program, pc, 0, &flags);
                let arg1 = resolve_flags(&program, pc, 1, &flags);
                let arg2 = program[pc + 3] as usize;
                if arg0 == arg1 {
                    program[arg2] = 1;
                } else {
                    program[arg2] = 0;
                }
                pc += 4
            }

            e @ _ => {
                println!("{:?}", program);
                panic!("bad opcode! {}", e);
            }
        }
    }
    output
}
