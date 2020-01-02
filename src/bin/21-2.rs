use adventofcode::intcode::{IntCodeComputer, IntCodeOutcome};
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut save = File::open("input/21.txt").unwrap();
    let mut contents = String::new();
    save.read_to_string(&mut contents).unwrap();
    contents = contents.trim().to_string();

    let mut comp = IntCodeComputer::parse_program(&contents).unwrap();
    comp.execute(&vec![]).unwrap();
    //println!("{}", to_ascii(&x));
    loop {
        let mut comp = comp.clone();
        let prog = rand_prog(8);
        let main = prog.bytes().map(|x| x as u8 as i64).collect::<Vec<i64>>();
        let x = comp.execute(&main).unwrap();
        let out = to_ascii(&x);
        //println!("{}", prog);
        match out {
            Outcome::Success(x) => {
                println!("SUCCESSSSSSS");
                println!("{}", prog);
                println!("{}", x);
                return;
            }
            Outcome::Failure(y) => {}
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
    E,
    F,
    G,
    H,
    I,
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
        Arg1::Sen(A),
        Arg1::Sen(B),
        Arg1::Sen(C),
        Arg1::Sen(D),
        Arg1::Sen(E),
        Arg1::Sen(F),
        Arg1::Sen(G),
        Arg1::Sen(H),
        Arg1::Sen(I),
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
    for el in prog {
        let Ins { op, arg1, arg2 } = el;
        let l = match arg1 {
            Arg1::Sen(x) => format!("{:?}", x),
            Arg1::Reg(x) => format!("{:?}", x),
        };
        pstring.push_str(&format!("{:?} {} {:?}\n", op, l, arg2));
    }
    pstring.push_str(&"RUN\n".to_string());
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
    if let Some(x) = msg.0.iter().find(|x| x > &&127) {
        println!("{}", x);
        Outcome::Success(m)
    } else {
        Outcome::Failure(m)
    }
}
