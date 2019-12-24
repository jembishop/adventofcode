use adventofcode::intcode::{IntCodeComputer, IntCodeOutcome};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut d = Drone::new();

    let mut counter = 0;
    for x in (0..50) {
        for y in (0..50) {
            d.pos = [x, y];
            if d.is_pulled() {counter +=1};
        }
    }
    println!("{}", counter);

}

struct Drone {
    pos: [usize; 2],
    comp: IntCodeComputer,

}

impl Drone {
    fn new() -> Self {
        let mut file = File::open("input/19.txt").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let input = &contents.trim().to_string();
        let comp = IntCodeComputer::parse_program(input).unwrap();
        Self { pos: [0, 0], comp }
    }
    fn is_pulled(&self) -> bool {
        let mut mcomp = self.comp.clone();
        let res = if let IntCodeOutcome::Finished(x) = mcomp
            .execute(&[self.pos[0] as i64, self.pos[1] as i64].to_vec())
            .unwrap()
        {
            x
        } else {
            unreachable!()
        };
        res[0] == 1
    }
}
