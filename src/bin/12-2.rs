use arrayvec::ArrayVec;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use num::integer::lcm;
use num_bigint::BigUint;

type Coord = ArrayVec<[i64; 3]>;
type Planets = (Vec<Coord>, Vec<Coord>);

#[test]
fn test_step() {
    let input = "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>
";
    let mut planets = parse(input);
    step(&mut planets, 0);
    step(&mut planets, 1);
    step(&mut planets, 2);
    println!("{:?}", planets);
    assert_eq!(
        planets,
        (
            vec![
                ArrayVec::from([2, -1, 1]),
                ArrayVec::from([3, -7, -4]),
                ArrayVec::from([1, -7, 5]),
                ArrayVec::from([2, 2, 0])
            ],
            vec![
                ArrayVec::from([3, -1, -1]),
                ArrayVec::from([1, 3, 3]),
                ArrayVec::from([-3, 1, -3]),
                ArrayVec::from([-1, -3, 1])
            ]
        )
    );
    for _ in 0..9 {
        step(&mut planets, 0);
        step(&mut planets, 1);
        step(&mut planets, 2);
    }
    assert_eq!(energy(&planets), 179);
}
fn main() {
    let mut file = File::open("input/12.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents = contents.trim().to_string();
    let mut planets = parse(&contents);
    // because the x,y,z are independent we can simulate each independently and find
    // the cycle. The final cycle is then the lcm of the cycles.
    let mut cycles = [0; 3];
    for i in 0..3 {
        let mut hs = HashSet::new();
        let mut c = 0;
        while hs.insert(planets.clone()) {
            step(&mut planets, i);
            c += 1;
        }
        cycles[i] = c;
        
    }
    println!("{:#?}", cycles);
    let a = BigUint::from(cycles[0] as u64);
    let b = BigUint::from(cycles[1] as u64);
    let c = BigUint::from(cycles[2] as u64);
    let g1 = lcm(a, b);
    let g2 = lcm(g1, c);
    println!("{:?}", g2.to_string());
}

fn parse(input: &str) -> Planets {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();
    }
    let pos: Vec<Coord> = RE
        .captures_iter(input)
        .map(|x| {
            x.iter()
                .skip(1)
                .map(|x| x.unwrap().as_str().parse::<i64>().unwrap())
                .collect::<Coord>()
        })
        .collect();
    let s = pos.len();
    (pos, vec![ArrayVec::from([0; 3]); s])
}

fn step(planets: &mut Planets, i: usize) {
    use std::cmp::Ordering::*;
    for ps in planets.0.iter().enumerate().combinations(2) {
        let ((idx1, p1), (idx2, p2)) = (ps[0], ps[1]);
        match p1[i].cmp(&p2[i]) {
            Greater => {
                planets.1[idx1][i] -= 1;
                planets.1[idx2][i] += 1
            }
            Less => {
                planets.1[idx1][i] += 1;
                planets.1[idx2][i] -= 1
            }
            Equal => (),
        }
    }
    for (p, v) in planets.0.iter_mut().zip(planets.1.iter()) {
        p[i] += v[i];
    }
}
