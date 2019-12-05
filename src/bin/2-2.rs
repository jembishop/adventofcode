use std::fs::File;
use std::io::prelude::*;
#[macro_use]
extern crate itertools;

fn main() {
    let mut file = File::open("input/2.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents = contents.trim().to_string();
    let program: Vec<_> = contents
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let res = execute(&program, 12, 2);
    for (noun, verb) in iproduct!(0..=99, 0..=99) {
        let x = execute(&program, noun, verb);
        if x == 19690720 {
            println!("noun: {} verb: {}", noun, verb);
            println!("answer: {}", noun*100 + verb);
        }
    }
}

fn execute(input: &Vec<i64>, noun: i64, verb: i64) -> i64 {
    let mut program = input.clone();
    program[1] = noun;
    program[2] = verb;
    for i in (0..program.len() - 1).step_by(4) {
        let op = program[i];
        if op == 99 {
            break;
        };
        let arg1 = program[i + 1] as usize;
        let arg2 = program[i + 2] as usize;
        let dest = program[i + 3] as usize;
        match op {
            1 => program[dest] = program[arg1] + program[arg2],
            2 => program[dest] = program[arg1] * program[arg2],
            _ => {
                panic!("bad opcode!");
            }
        }
    }
    program[0]
}
