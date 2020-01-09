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
    let mut packets = vec![vec![]; n];
    let mut nat = [0; 2];
    let mut idle = false;
    for _ in 0..200 {
        if idle {
            println!("NAT {:?}", nat);
            let res = if let NeedInput(x) = computers[0].execute(&nat.to_vec()).unwrap() {
                x
            } else {
                panic!()
            };
            for pack in res.clone().chunks(3) {
                let dest = pack[0] as usize;
                let r = vec![pack[1], pack[2]];
                packets[dest].extend(r);
            }
        }
        for (idx, comp) in computers.iter_mut().enumerate() {
            let mut p = packets[idx].clone();
            p = if p.len() == 0 { vec![-1] } else { packets[idx] = vec![]; p };
            let res = if let NeedInput(x) = comp.execute(&p).unwrap() {
                x
            } else {
                panic!()
            };
            if res.len() != 0 {
                for pack in res.clone().chunks(3) {
                    let dest = pack[0] as usize;
                    if dest == 255 {
                        nat[0] = pack[1];
                        nat[1] = pack[2];
                    } else {
                        let r = vec![pack[1], pack[2]];
                        packets[dest].extend(r);
                    }
                }
            }
        }
        idle = packets.iter().filter(|x| x.len() != 0).count() == 0;
    }
}
