use adventofcode::intcode::{IntCodeComputer, IntCodeOutcome};
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut save = File::open("input/17.txt").unwrap();
    let mut contents = String::new();
    save.read_to_string(&mut contents).unwrap();
    contents = contents.trim().to_string();
    //println!("{}", contents);
    let mut comp = IntCodeComputer::parse_program(&contents).unwrap();
    if let IntCodeOutcome::Finished(x) = comp.execute(&vec![]).unwrap() {
        print_scaffold(&x);
        let pa = path(&x);
        let du = duplicates(&pa);
        println!("{:?}", du.iter().map(|x| x[0] * x[1]).sum::<i64>());
    }
}

enum Turn{
    L,
    R,
}

struct Move {
    dist: i64,
    turn: Turn,
}

fn path_to_ins(v: &Vec<[i64;2]>) -> Vec<Move> {
    let mut dist = 0;
    let mut turn = Turn::L;
    let mut moves = vec![];
    let mut diff = [-1, 0];
    for w in v.windows(2) {
        let ndiff = [w[1][0] - w[0][0], w[1][1] - w[0][1]];
        if ndiff != diff {
            moves.push(Move{dist, turn});
            turn = match (diff, ndiff) {
                ([-1, 0], [0, 1]) | ([0, 1], [1, 0]) | ([1, 0], [0, -1]) | ([0, -1], [-1, 0]) => Turn::R,
                ([-1, 0], [0, -1]) | ([0, 1], [-1, 0]) | ([1, 0], [0, 1]) | ([0, -1], [1, 0]) => Turn::L,
                _ => unreachable!()
            };
            dist = 0;
        }
        else {dist += 1}
    }
    moves
}


enum Dir {
    L,
    R,
    U,
    D,
}

fn duplicates(v: &Vec<[i64; 2]>) -> Vec<[i64; 2]> {
    let mut dups = vec![];
    let mut enc = HashSet::new();
    for el in v.iter() {
        if !enc.insert(el) {
            dups.push(*el)
        };
    }
    dups
}

fn path(v: &Vec<i64>) -> Vec<[i64; 2]> {
    use Dir::*;
    let lines = &v[..v.len() - 3].split(|x| *x == 10).collect::<Vec<_>>();
    let mut pos = [
        lines[lines.len() - 1]
            .iter()
            .position(|x| *x == 94)
            .unwrap() as i64,
            (lines.len() - 1) as i64,
    ];
    let mut current_dir = L;
    let mut v: Vec<[i64; 2]> = vec![];
    loop {
        let next = match current_dir {
            L => [pos[0] - 1, pos[1]],
            R => [pos[0] + 1, pos[1]],
            U => [pos[0], pos[1] + 1],
            D => [pos[0], pos[1] - 1],
        };
        match lines
            .get(next[1] as usize)
            .and_then(|x| x.get(next[0] as usize))
        {
            Some(35) => {
                pos = next;
                v.push(pos);
            }
            _ => match current_dir {
                L | R => {
                    if lines
                        .get((pos[1] + 1) as usize)
                        .and_then(|x| x.get(pos[0] as usize))
                        == Some(&35)
                    {
                        current_dir = U;
                    } else if pos[1] >= 0
                        && lines
                            .get((pos[1] - 1) as usize)
                            .and_then(|x| x.get(pos[0] as usize))
                            == Some(&35)
                    {
                        current_dir = D;
                    } else {
                        break;
                    }
                }
                U | D => {
                    if lines
                        .get(pos[1] as usize)
                        .and_then(|x| x.get((pos[0] + 1) as usize))
                        == Some(&35)
                    {
                        current_dir = R;
                    } else if pos[0] >= 0
                        && lines
                            .get(pos[1] as usize)
                            .and_then(|x| x.get((pos[0] - 1) as usize))
                            == Some(&35)
                    {
                        current_dir = L;
                    } else {
                        break;
                    }
                }
            },
        }
    }
    v
}
fn print_scaffold(v: &Vec<i64>) {
    for i in v.iter() {
        print!("{}", *i as u8 as char);
    }
}
