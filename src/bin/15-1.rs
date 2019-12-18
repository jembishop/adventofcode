use adventofcode::intcode::{IntCodeComputer, IntCodeOutcome};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::{thread, time};

const SIZE: usize = 50;

const OXYGEN: [usize; 2] = [9, 45];

fn main() {
    //let mut rng = rand::thread_rng();
    let mut file = File::open("input/15.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let input = &contents.trim().to_string();
    let mut droid = Droid::new(IntCodeComputer::parse_program(input).unwrap());
    //brute force to get map layout
    //    for _ in 0..10_000_000 {
    //        let dir = rng.gen::<u32>() % 4;
    //        droid.try_move(dir + 1);
    //    }
    //    droid.print_map();
    //    serde_json::to_writer(&File::create("map.json").unwrap(), &droid.map).unwrap();
    let mut save = File::open("map.json").unwrap();
    let mut contents = String::new();
    save.read_to_string(&mut contents).unwrap();
    contents = contents.trim().to_string();
    //println!("{}", contents);
    let map: Vec<Vec<Tile>> = serde_json::from_str(&contents).unwrap();
    let pos = [SIZE / 2; 2];
    println!(
        "dist is {}",
        a_star(
            &map,
            [pos[0] as i64, pos[1] as i64],
            [OXYGEN[0] as i64, OXYGEN[1] as i64]
        )
    );
}

fn a_star(map: &Vec<Vec<Tile>>, start: [i64; 2], fin: [i64; 2]) -> i64 {
    fn dist(p1: [i64; 2], p2: [i64; 2]) -> i64 {
        ((p1[0] - p2[0]).abs() + (p1[1] - p2[1]).abs())
    }
    let mut bset: HashMap<[i64; 2], (i64, i64)> = HashMap::new();
    let mut kset: HashMap<[i64; 2], (i64, i64)> = HashMap::new();
    kset.insert(start, (0, dist(start, fin)));

    for dir in [[0, 1], [1, 0], [0, -1], [-1, 0]].iter() {
        let x = start[0] + dir[0];
        let y = start[1] + dir[1];
        if map[y as usize][x as usize] != Tile::Wall {
            bset.insert([x, y], (1, dist([x, y], fin)));
        }
    }
    loop {
    //    println!("boundary {:?}", bset);
     //   println!("known {:?}", kset);
        //find best in boundary set
        let (best_point, (travel, dis)) = bset.iter().min_by_key(|x| (x.1).0 + (x.1).1).unwrap();
        let (best_point, (travel, dis)) = (*best_point, (*travel, *dis));
        kset.insert(best_point, (travel, dis));
        bset.remove(&best_point);
        if best_point == fin {
            break travel;
        }
        //expand boundary set
        for dir in [[0, 1], [1, 0], [0, -1], [-1, 0]].iter() {
            let x = best_point[0] + dir[0];
            let y = best_point[1] + dir[1];
            if map[y as usize][x as usize] != Tile::Wall && !kset.contains_key(&[x, y]) {
                bset.insert([x, y], (travel + 1, dist([x, y], fin)));
            }
        }
    }
}

#[derive(Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
enum Tile {
    Clear,
    Wall,
    Oxygen,
    Unknown,
}
struct Droid {
    pos: [usize; 2],
    map: Vec<Vec<Tile>>,
    comp: IntCodeComputer,
}

impl Droid {
    fn new(comp: IntCodeComputer) -> Self {
        Droid {
            pos: [SIZE / 2; 2],
            map: vec![vec![Tile::Unknown; SIZE]; SIZE],
            comp,
        }
    }
    fn try_move(&mut self, m: u32) -> i64 {
        if let IntCodeOutcome::NeedInput(res) = self.comp.execute(&vec![m as i64]).unwrap() {
            let new_dir = match m {
                1 => [0, 1],
                2 => [0, -1],
                3 => [-1, 0],
                4 => [1, 0],
                _ => unreachable!(),
            };
            let new_pos = [
                (self.pos[0] as i64 + new_dir[0]) as usize,
                (self.pos[1] as i64 + new_dir[1]) as usize,
            ];

            use Tile::*;
            match res[0] {
                0 => self.map[new_pos[1]][new_pos[0]] = Wall,
                1 => {
                    self.map[new_pos[1]][new_pos[0]] = Clear;
                    self.pos = new_pos
                }
                2 => {
                    self.map[new_pos[1]][new_pos[0]] = Oxygen;
                    self.pos = new_pos;
                    //println!("O found at {:?}", new_pos);
                }
                _ => unreachable!(),
            }
            res[0]
        } else {
            panic!()
        }
    }
    fn print_map(&self) {
        use Tile::*;
        let map = self
            .map
            .iter()
            .rev()
            .map(|r| {
                r.iter()
                    .map(|x| match x {
                        Clear => ' ',
                        Wall => '\u{2588}',
                        Oxygen => 'O',
                        Unknown => '\u{2591}',
                    })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n");
        println!("{}", map);
    }
}
