use adventofcode::intcode::{IntCodeComputer, IntCodeOutcome};
use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;

const AMP_NUM: usize = 5;

#[test]
fn test_thrust() {
    let contents =
        "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
            .to_string();
    let prog = IntCodeComputer::parse_program(&contents).unwrap();
    let t = thrust(prog, &vec![9, 8, 7, 6, 5]);
    assert_eq!(t, 139629729);

    let contents = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,\
    55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,\
    53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10".to_string();
    let prog = IntCodeComputer::parse_program(&contents).unwrap();
    let t = thrust(prog, &vec![9, 7, 8, 5, 6]);
    assert_eq!(t, 18216);
}
fn main() {
    let mut file = File::open("input/7.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents = contents.trim().to_string();
    let prog = IntCodeComputer::parse_program(&contents).unwrap();
    let best = max_thrust(&prog);
    println!("{:?}", best);
}

fn max_thrust(prog: &IntCodeComputer) -> i64 {
    (5..=9)
        .permutations(AMP_NUM)
        .map(|config| thrust(prog.clone(), &config))
        .max()
        .unwrap()
}

fn thrust(prog: IntCodeComputer, config: &Vec<i64>) -> i64 {
    let mut computers = vec![prog.clone(); AMP_NUM];
    let mut prev = 0;
    //initialize
    let mut input;
    for (computer, setting) in computers.iter_mut().zip(config) {
        input = vec![*setting, prev];
        let result = if let IntCodeOutcome::NeedInput(res) = computer.execute(&input).unwrap() {
            res
        } else {
            panic!()
        };
        assert_eq!(result.len(), 1);
        prev = result[0];
    }
    //loop
    input = vec![prev];
    loop {
        for (idx, computer) in computers.iter_mut().enumerate() {
            match computer.execute(&input).unwrap() {
                IntCodeOutcome::NeedInput(res) => input = res,
                IntCodeOutcome::Finished(res) => {
                    if idx == (AMP_NUM - 1) {
                        return res[0];
                    } else {
                        input = res
                    }
                }
            }
        }
    }
}
