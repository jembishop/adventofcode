use adventofcode::intcode::{IntCodeComputer, IntCodeOutcome, IntCodeError}; 
use std::fs::File;
use std::io::prelude::*;
use itertools::Itertools;

#[test]
fn test_thrust() {
    let contents = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0".to_string();
    let prog = IntCodeComputer::parse_program(&contents).unwrap();
    let t = thrust(prog, &vec![4, 3, 2, 1, 0]);
    assert_eq!(t, 43210);

    let contents =
        "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0".to_string();
    let prog = IntCodeComputer::parse_program(&contents).unwrap();
    let t = thrust(prog, &vec![0, 1, 2, 3, 4]);
    assert_eq!(t, 54321);

    let contents = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0".to_string();
    let prog = IntCodeComputer::parse_program(&contents).unwrap();
    let t = thrust(prog, &vec![1, 0, 4, 3, 2]);
    assert_eq!(t, 65210);
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
    (0..=4).permutations(5)
    .map(|config| {
        thrust(prog.clone(), &config)
    })
    .max()
    .unwrap()
}

fn thrust(prog: IntCodeComputer, config: &Vec<i64>) -> i64 {
    let mut prev = 0;
    let mut res = vec![0];
    for i in config.iter() {
        let mut comp = prog.clone();
        let input = vec![*i, prev];
        res = if let IntCodeOutcome::Finished(res) = comp.execute(&input).unwrap() {res} else {panic!()};
        assert_eq!(res.len(), 1);
        prev = res[0];
    }
    res[0]
}
