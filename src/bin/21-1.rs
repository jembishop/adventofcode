use adventofcode::intcode::{IntCodeComputer, IntCodeOutcome};
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::prelude::*;

fn main() {
    let mut save = File::open("input/21.txt").unwrap();
    let mut contents = String::new();
    save.read_to_string(&mut contents).unwrap();
    contents = contents.trim().to_string();

    let mut comp = IntCodeComputer::parse_program(&contents).unwrap();
    comp.execute(&vec![]).unwrap();
    //println!("{}", to_ascii(&x));
    let mut failure_mode = vec![];
    let mut cache = HashMap::new();
    for _ in 0..50000 {
        let mut comp = comp.clone();
        let mut rng = rand::thread_rng();
        let prog = rand_prog(rng.gen::<usize>() % 13);
        let mut s = DefaultHasher::new();
        prog.hash(&mut s);
        let other = s.finish();
        if cache.get(&other) != None {
            continue;
        }
        let main = prog.bytes().map(|x| x as u8 as i64).collect::<Vec<i64>>();
        let x = comp.execute(&main).unwrap();
        let out = to_ascii(&x);
        //println!("{}", prog);
        match out {
            Outcome::Success(x) => {
                println!("{}", prog);
                println!("{}", x);
                break;
            }
            Outcome::Failure(y) => {
                let mut s = DefaultHasher::new();
                prog.hash(&mut s);
                let other = s.finish();
                let idx = match failure_mode.iter().position(|x| x == &y) {
                    Some(x) => x,
                    None => {
                        println!("{}", prog);
                        println!("{}", y);
                        //println!("{:?}", failure_mode);
                        let mut s = DefaultHasher::new();
                        prog.hash(&mut s);
                        failure_mode.push(y);
                        failure_mode.len() - 1
                    }
                };
                cache.insert(other, idx);
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Op {
    OR,
    AND,
    NOT,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Sen {
    A,
    B,
    C,
    D,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Reg {
    J,
    T,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Arg1 {
    Sen(Sen),
    Reg(Reg),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Ins {
    op: Op,
    arg1: Arg1,
    arg2: Reg,
}

fn rand_prog(len: usize) -> String {
    use Op::*;
    use Reg::*;
    use Sen::*;
    let mut prog = vec![];
    let ops = [OR, AND, NOT];
    let arg1s = [
        Arg1::Sen(B),
        Arg1::Sen(C),
        Arg1::Sen(D),
        Arg1::Reg(J),
        Arg1::Reg(T),
    ];
    let arg2s = [J, T];

    let op = *[NOT, OR].choose(&mut rand::thread_rng()).unwrap();
    let arg1 = Arg1::Sen(*[B, C, D].choose(&mut rand::thread_rng()).unwrap());
    let arg2 = *arg2s.choose(&mut rand::thread_rng()).unwrap();
    prog.push(Ins { op, arg1, arg2 });

    for _ in 0..len - 1 {
        let op = *ops.choose(&mut rand::thread_rng()).unwrap();
        let arg1 = *arg1s.choose(&mut rand::thread_rng()).unwrap();
        let arg2 = *arg2s.choose(&mut rand::thread_rng()).unwrap();
        prog.push(Ins { op, arg1, arg2 });
    }
    // optimize

    loop {
        let old_len = prog.len();
        prog = prog
            .into_iter()
            .filter(|x| {
                if let Ins {
                    op,
                    arg1: Arg1::Reg(a),
                    arg2,
                } = x
                {
                    if (*op == OR || *op == AND) && arg2 == a {
                        return false;
                    }
                }
                true
            })
            .collect();

        if prog.len() == old_len {
            break;
        }
    }

    let mut pstring = String::new();
    prog.push(Ins {
        op: OR,
        arg1: Arg1::Sen(A),
        arg2: J,
    });
    for el in prog {
        let Ins { op, arg1, arg2 } = el;
        let l = match arg1 {
            Arg1::Sen(x) => format!("{:?}", x),
            Arg1::Reg(x) => format!("{:?}", x),
        };
        pstring.push_str(&format!("{:?} {} {:?}\n", op, l, arg2));
    }
    pstring.push_str(&"WALK\n".to_string());
    pstring
}

enum Outcome {
    Failure(String),
    Success(String),
}

fn to_ascii(input: &IntCodeOutcome) -> Outcome {
    use IntCodeOutcome::*;
    let msg = match input {
        NeedInput(x) => (x, ""),
        Finished(x) => (x, ""),
    };

    let m = format!(
        "{}\n{}",
        msg.0.iter().map(|x| *x as u8 as char).collect::<String>(),
        msg.1
    );
    if let Some(_) = msg.0.iter().position(|x| x > &127) {
        Outcome::Success(m)
    } else {
        Outcome::Failure(m)
    }
}
