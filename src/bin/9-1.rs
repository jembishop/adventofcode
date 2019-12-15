use std::fs::File;
use std::io::prelude::*;
use adventofcode::intcode::IntCodeComputer;

fn main() {
    let mut file = File::open("input/9.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents = contents.trim().to_string();
    let mut comp = IntCodeComputer::parse_program(&contents).unwrap();
    let out = comp.execute(&vec![2]).unwrap();
    println!("{:?}", out);
}
