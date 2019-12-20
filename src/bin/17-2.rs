use adventofcode::intcode::{IntCodeComputer, IntCodeOutcome};
use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::io::prelude::*;

#[test]
fn test_func_cov() {
    let mut save = File::open("input/17.txt").unwrap();
    let mut contents = String::new();
    save.read_to_string(&mut contents).unwrap();
    contents = contents.trim().to_string();
    let mut comp = IntCodeComputer::parse_program(&contents).unwrap();
    if let IntCodeOutcome::Finished(x) = comp.execute(&vec![]).unwrap() {
        let pa = path(&x);
        let ins = path_to_ins(&pa);
        let exfc = func_cov(
            &ins,
            &[
                Move {
                    turn: Turn::R,
                    dist: 8,
                },
                Move {
                    turn: Turn::L,
                    dist: 12,
                },
            ],
        );
        assert_eq!(exfc, 6);
        let exfc = func_cov(
            &ins,
            &[
                Move {
                    turn: Turn::R,
                    dist: 6,
                },
                Move {
                    turn: Turn::R,
                    dist: 6,
                },
                Move {
                    turn: Turn::R,
                    dist: 10,
                },
            ],
        );
        assert_eq!(exfc, 9);
        let exfc = func_cov(
            &ins,
            &[
                Move {
                    turn: Turn::R,
                    dist: 7,
                },
                Move {
                    turn: Turn::R,
                    dist: 6,
                },
                Move {
                    turn: Turn::R,
                    dist: 10,
                },
            ],
        );
        assert_eq!(exfc, 0);
    }
}

fn main() {
    let mut save = File::open("input/17.txt").unwrap();
    let mut contents = String::new();
    save.read_to_string(&mut contents).unwrap();
    contents = contents.trim().to_string();

    let mut comp = IntCodeComputer::parse_program(&contents).unwrap();
    comp.state[0] = 2;

    let main = "C,B,C,A,A,C,B,A,B,B\n".bytes().map(|x| x as u8 as i64).collect::<Vec<i64>>();
    let x = comp.execute(&main).unwrap();
   // println!("{:?}", x);
    
    let pA = "L,8,R,6,R,6,R,10,L,8\n".bytes().map(|x| x as u8 as i64).collect::<Vec<i64>>();
    let x = comp.execute(&pA).unwrap();
    //println!("{:?}", x);
    let pB = "L,12,R,8,R,8\n".bytes().map(|x| x as u8 as i64).collect::<Vec<i64>>();
    let x = comp.execute(&pB).unwrap();
    //println!("{:?}", x);
    let pC = "L,8,R,10,L,8,R,8\n".bytes().map(|x| x as u8 as i64).collect::<Vec<i64>>();
    let x = comp.execute(&pC).unwrap();
    //println!("{:?}", x);
    let y = "n\n".bytes().map(|x| x as u8 as i64).collect::<Vec<i64>>();
    let x = comp.execute(&y).unwrap();
    if let IntCodeOutcome::Finished(xx) = x {
        println!("{:?}", xx);
        print_scaffold(&xx);
    }
    //println!("{:?}", x);
}

fn print_ins(ins: &Vec<Move>) {
    let s = ins
        .iter()
        .map(|x| format!("{:?},{},", x.turn, x.dist))
        .collect::<String>();
    println!("{}", s);
}
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Turn {
    L,
    R,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Move {
    turn: Turn,
    dist: i64,
}

fn func_cov(moves: &Vec<Move>, sub: &[Move]) -> usize {
    let mut counter = 0;
    let mut counts = 0;
    while counter <= moves.len() - sub.len() {
        if &moves[counter..counter + sub.len()] == sub {
            counts += 1;
            counter += sub.len() - 1;
        }
        counter += 1
    }
    counts * sub.len()
}

fn find_mov_funcs(moves: &Vec<Move>) {
    let mut big_funcs: HashMap<Vec<Move>, usize> = HashMap::new();
    let mut l_mov_idx: usize = 0;
    let mut r_mov_idx: usize = 1;
    let min_cov = moves.len() / 3;
    while l_mov_idx != moves.len() - 2 {
        let pot_ss = &moves[l_mov_idx..r_mov_idx];
        let cov = func_cov(moves, pot_ss);
        if cov >= min_cov {
            big_funcs.insert(pot_ss.to_vec(), cov);
        }
        if cov - pot_ss.len() == 0 || pot_ss.len() * 4 > 20 {
            l_mov_idx += 1
        } else if r_mov_idx < moves.len() {
            r_mov_idx += 1
        } else {
            break;
        }
    }
    for el in big_funcs.iter() {
        print_ins(&el.0);
        println!("{}", el.1);
    }
}

fn path_to_ins(v: &Vec<[i64; 2]>) -> Vec<Move> {
    let mut dist = 1;
    let mut turn = Turn::L;
    let mut moves = vec![];
    let mut diff = [-1, 0];
    for w in v.windows(2) {
        let ndiff = [w[1][0] - w[0][0], w[1][1] - w[0][1]];
        if ndiff != diff {
            moves.push(Move { dist, turn });
            turn = match (diff, ndiff) {
                ([-1, 0], [0, 1]) | ([0, 1], [1, 0]) | ([1, 0], [0, -1]) | ([0, -1], [-1, 0]) => {
                    Turn::L
                }
                ([-1, 0], [0, -1]) | ([0, 1], [-1, 0]) | ([1, 0], [0, 1]) | ([0, -1], [1, 0]) => {
                    Turn::R
                }
                _ => unreachable!(),
            };
            dist = 1;
        } else {
            dist += 1
        }
        diff = ndiff;
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
