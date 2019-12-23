use adventofcode::intcode::{IntCodeComputer, IntCodeOutcome};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::{thread, time};

fn main() {
    //let mut rng = rand::thread_rng();
    let mut file = File::open("input/15.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let input = &contents.trim().to_string();
//    let map: Vec<Vec<Tile>> = parse(input);
}

fn new_keys(map:&Vec<Vec<Tile>>,  pos: [i64; 2]) -> Vec<(char, [i64; 2], i64)> {


fn a_star(map: &Vec<Vec<Tile>>, start: [i64; 2], fin: [i64; 2]) -> Option<(i64, [i64; 2])> {
    use Tile::*;
    fn dist(p1: [i64; 2], p2: [i64; 2]) -> i64 {
        ((p1[0] - p2[0]).abs() + (p1[1] - p2[1]).abs())
    }
    let mut bset: HashMap<[i64; 2], (i64, i64)> = HashMap::new();
    let mut kset: HashMap<[i64; 2], (i64, i64)> = HashMap::new();
    kset.insert(start, (0, dist(start, fin)));

    for dir in [[0, 1], [1, 0], [0, -1], [-1, 0]].iter() {
        let x = start[0] + dir[0];
        let y = start[1] + dir[1];
        match map[y as usize][x as usize] {
            Wall | Door(_) => {
                    bset.insert([x, y], (0, dist([x, y], fin)));
            }
            _ => {}
        }
        if map[y as usize][x as usize] != Tile::Wall {
            bset.insert([x, y], (1, dist([x, y], fin)));
        }
    }
    loop {
        let min = bset.iter().min_by_key(|x| (x.1).0 + (x.1).1);
        let (best_point, (travel, dis)) = match min {
            Some(x) => x,
            None => return None
        };
        let (best_point, (travel, dis)) = (*best_point, (*travel, *dis));
        kset.insert(best_point, (travel, dis));
        bset.remove(&best_point);
        if best_point == fin {
            break Some((travel, best_point));
        }
        //expand boundary set
        for dir in [[0, 1], [1, 0], [0, -1], [-1, 0]].iter() {
            let x = best_point[0] + dir[0];
            let y = best_point[1] + dir[1];
            match map[y as usize][x as usize] {
                Wall | Door(_) => {
                    if !kset.contains_key(&[x, y]) {
                        bset.insert([x, y], (travel + 1, dist([x, y], fin)));
                    }
                }
                _ => {}
                
            }
        }
    }
}

#[derive(Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
enum Tile {
    Clear,
    Wall,
    Key(char),
    Door(char),
}

