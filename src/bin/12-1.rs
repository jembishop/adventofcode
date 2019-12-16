use arrayvec::ArrayVec;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

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
    step(&mut planets);
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
        step(&mut planets);
    }
    assert_eq!(energy(&planets), 179);

}
fn main() {
    let mut file = File::open("input/12.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents = contents.trim().to_string();
    let mut p = parse(&contents);
    for _ in 0..1000 {
    step(&mut p);
    }
    println!("{:#?}", energy(&p));

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

fn step(planets: &mut Planets) {
    use std::cmp::Ordering::*;
    for i in 0..3 {
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
}

fn energy(planets: &Planets) -> usize {
    let pe: Vec<usize> = planets
        .0
        .iter()
        .map(|x: &Coord| {
            x.iter()
                .map(|z: &i64| -> usize { z.abs() as usize })
                .sum::<usize>()
        })
        .collect();
    let ke: Vec<usize> = planets
        .1
        .iter()
        .map(|x: &Coord| {
            x.iter()
                .map(|z: &i64| -> usize { z.abs() as usize })
                .sum::<usize>()
        })
        .collect();

    pe.iter().zip(ke.iter()).map(|x| x.0*x.1).sum()
}
