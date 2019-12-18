use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

const OXYGEN: [usize; 2] = [9, 45];

fn main() {
    let mut save = File::open("map.json").unwrap();
    let mut contents = String::new();
    save.read_to_string(&mut contents).unwrap();
    contents = contents.trim().to_string();
    let map: Vec<Vec<Tile>> = serde_json::from_str(&contents).unwrap();
    println!(
        "dist is {}",
        furthest_point(&map)
    );
}

fn furthest_point(map: &Vec<Vec<Tile>>) -> i64 {
    map.iter()
        .enumerate()
        .map(|a| {
            let (y, r) = a;
            r.iter()
                .enumerate()
                .filter(|x| x.1 == &Tile::Clear)
                .map(move |p| {
                    let (x, _) = p;
                    a_star(
                        &map,
                        [OXYGEN[0] as i64, OXYGEN[1] as i64],
                        [x as i64, y as i64],
                    )
                })
        })
        .flatten()
        .max()
        .unwrap()
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
