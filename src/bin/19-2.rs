use adventofcode::intcode::{IntCodeComputer, IntCodeOutcome};
use std::fs::File;
use std::io::prelude::*;

const SQUARE: usize = 100;

fn main() {
    let d = Drone::new();
    let n = 5000;
    let mut beam = vec![];
    let mut start = 0;
    let input = 
    for y in 0..n {
        let mut started = false;
        let mut b = [0, 0];
        for x in start..n {
            let res = d.is_pulled([x, y]);
            if res && !started {
                b[0] = x;
                start = x;
                started = true;
            }
            if !res && started {
                b[1] = x;
                break;
            }
        }
            beam.push(b);
    }
    //println!("{:?} beam", beam);
    println!("finding...");
    for (y, row) in beam.iter().enumerate() {
        for x in row[0]..row[1] {
            if test_square(&beam, [x, y], SQUARE) {
                println!("found! on x,y {}, {}", x, y);
                println!("row {:?}", &beam[y..(y+10)]);
                println!("{}", x * 10000 + y);
                println!("d to emitter {}", x.pow(2) + y.pow(2));
                return;
            }
        }
    }
}

fn test_square(b: &Vec<[usize; 2]>, pos: [usize; 2], size: usize) -> bool {
    let y = pos[1];
    for j in y..(y + size) {
        //println!("x,y {:?}, {}", pos, j);
        let row = b[j];
        if !(pos[0] >= row[0] && pos[0] + size <= row[1]) {
            return false;
        };
    }
    true
}

struct Drone {
    comp: IntCodeComputer,
}

impl Drone {
    fn new() -> Self {
        let mut file = File::open("input/19.txt").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let input = &contents.trim().to_string();
        let comp = IntCodeComputer::parse_program(input).unwrap();
        Self { comp }
    }
    fn is_pulled(&self, pos: [usize; 2]) -> bool {
        let mut mcomp = self.comp.clone();
        let res = if let IntCodeOutcome::Finished(x) = mcomp
            .execute(&[pos[0] as i64, pos[1] as i64].to_vec())
            .unwrap()
        {
            x
        } else {
            unreachable!()
        };
        res[0] == 1
    }
}
