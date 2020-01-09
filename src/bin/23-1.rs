use adventofcode::intcode::{IntCodeComputer, IntCodeOutcome};
use std::fs;

fn main() {
    use IntCodeOutcome::*;
    let contents = fs::read_to_string("input/23.txt").unwrap();
    //println!("{}", contents);
    let n: usize = 50;
    let mut computers = (0..n)
        .map(|x| {
            let mut c = IntCodeComputer::parse_program(&contents.trim()).unwrap();
            c.execute(&vec![x as i64]).unwrap();
            c
        })
        .collect::<Vec<_>>();
    let mut packets = vec![vec![];n];
    loop {
        for (idx, comp) in computers.iter_mut().enumerate() {
            let mut p = packets[idx].clone();
            p = if p.len() == 0 {vec![-1]} else {p};
            let res = if let NeedInput(x) = comp.execute(&p).unwrap() {
                x
            } else {
                panic!()
            };
            if res.len() != 0 {
                for pack in res.clone().chunks(3) {
                let dest = pack[0] as usize;
                if dest == 255 {
                println!("{:?}", pack);
                    break
                }
                let r = vec![pack[1], pack[2]];
                packets[dest].extend(r);
                }
            }
        }
    }
}
