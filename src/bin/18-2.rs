use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
//use std::{thread, time};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
enum ClearType {
    Norm,
    Mark,
    Debug,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
enum Tile {
    Clear(ClearType),
    Wall,
    Key(char),
    Door(char),
}

fn main() {
    let mut file = File::open("nhrinput2.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let input = contents.trim().to_string();
    let (map, starts) = parse(&input);
    let key_list = map
        .iter()
        .enumerate()
        .map(|(y, r)| {
            r.iter().enumerate().filter_map(move |(x, p)| match p {
                Tile::Key(_) => Some([x as i64, y as i64]),
                _ => None,
            })
        })
        .flatten()
        .collect::<Vec<[i64; 2]>>();
    let mut best_dist = 1000000;
    let nn = 10000;
    let proto = vec![0; 40];
    let mut vs = vec![proto; nn];
    loop {
        let mut nvsh = vec![];
        for v in vs.iter_mut().skip(1) {
            mutate(v);
            let (dist, path) = simulate(&map, &key_list, &starts, &v);
            nvsh.push((v.clone(), dist));
            if dist <= best_dist {
                best_dist = dist;
                println!("{}", dist);
                println!("{}", path);
                println!("{:?}", v);
            }
        }
        nvsh.sort_by_key(|x| x.1);
        vs = nvsh
            .into_iter()
            .map(|x| x.0)
            .take(10)
            .cycle()
            .take(nn)
            .collect();
    }
}

fn mutate(v: &mut Vec<i64>) {
    for i in v.iter_mut() {
        let mut rand = rand::thread_rng();
        let rand: i8 = rand.gen();
        let fff = 11;
        let new = *i as i8 + (rand % fff - fff / 2);
        if new >= -1 {
            *i = (new as i64) % 10;
        }
    }
}
fn simulate(
    map: &Vec<Vec<Tile>>,
    key_list: &Vec<[i64; 2]>,
    starts: &Vec<[i64; 2]>,
    policy: &Vec<i64>,
) -> (i64, String) {
    //    match cache.get(policy) {
    //        Some(x) => {println!("hit!"); return x.clone()},
    //        _ =>{}
    //    };
    let mut key_list = key_list.clone();
    let mut map = map.clone();
    let mut dist = 0;
    let mut path = String::new();
    let mut counter = 0;
    let mut new_keyz = starts.clone(); 
    let ret = loop {
        for new_key in new_keyz.iter_mut() {

            let mut nk = new_keys(&mut map, *new_key, &key_list);
            nk.sort_by_key(|x| x.1);
            //let mut rand = rand::thread_rng();
            // let rand: usize = rand.gen();
            //let new =  rand % nk.len();
            if nk.len() == 0 {
        //        println!("skipped...");
                continue;
            };
            let new = policy[counter];
            if new != -1 {
                let new = new as usize % nk.len();
                *new_key = nk[new].0;
                dist += nk[new].1;
                path.push(unlock(&mut map, *new_key));
                key_list = key_list
                    .into_iter()
                    .filter(|x| *x != *new_key)
                    .collect::<Vec<_>>();
            } else {
                //path.push('#');
            };
            counter += 1;
        }
        if path.len() == 26 {
            break (dist, path);
        };
    };
    ret
}
fn print_map(map: &Vec<Vec<Tile>>) {
    use Tile::*;
    for l in map.iter() {
        println!(
            "{}",
            l.iter()
                .map(|x| match x {
                    Clear(ClearType::Norm) => ' ',
                    Clear(ClearType::Mark) => '\u{2591}',
                    Clear(ClearType::Debug) => '"',
                    Wall => '\u{2588}',
                    Key(y) => *y,
                    Door(y) => *y,
                })
                .collect::<String>()
        );
    }
}

fn unlock(map: &mut Vec<Vec<Tile>>, keycoord: [i64; 2]) -> char {
    let r = &mut map[keycoord[1] as usize][keycoord[0] as usize];
    //println!("{:?}", r);
    let key = if let Tile::Key(x) = *r { x } else { panic!() };
    *r = Tile::Clear(ClearType::Mark);
    let door = map
        .iter()
        .enumerate()
        .map(|(y, r)| {
            r.iter().enumerate().filter_map(move |(x, p)| match p {
                Tile::Door(p) if p == &key.to_ascii_uppercase() => Some([x as i64, y as i64]),
                _ => None,
            })
        })
        .flatten()
        .collect::<Vec<[i64; 2]>>();
    //println!("{:?}", map);
    //print_map(&map);
    if door.len() == 1 {
        let door = door[0];
        map[door[1] as usize][door[0] as usize] = Tile::Clear(ClearType::Mark);
    };
    key
}

fn parse(input: &str) -> (Vec<Vec<Tile>>, Vec<[i64; 2]>) {
    use Tile::*;
    let mut poss = vec![];
    let x = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, p)| match p {
                    p @ 'a'..='z' => Key(p),
                    p @ 'A'..='Z' => Door(p),
                    '/' | '#' | '\u{2588}' => Wall,
                    '.' | ' ' => Clear(ClearType::Norm),
                    '@' => {
                        poss.push([x as i64, y as i64]);
                        Clear(ClearType::Norm)
                    }
                    i => unreachable!(i),
                })
                .collect::<Vec<Tile>>()
        })
        .collect::<Vec<Vec<Tile>>>();
    (x, poss)
}

fn new_keys(
    map: &mut Vec<Vec<Tile>>,
    pos: [i64; 2],
    key_coords: &Vec<[i64; 2]>,
) -> Vec<([i64; 2], i64)> {
    key_coords
        .iter()
        .filter_map(|x| a_star(map, pos, *x))
        .collect::<Vec<_>>()
}

fn a_star(map: &mut Vec<Vec<Tile>>, start: [i64; 2], fin: [i64; 2]) -> Option<([i64; 2], i64)> {
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
            Wall | Door(_) => {}
            _ => {
                bset.insert([x, y], (0, dist([x, y], fin)));
            }
        }
        if map[y as usize][x as usize] != Tile::Wall {
            bset.insert([x, y], (1, dist([x, y], fin)));
        }
    }
    loop {
        let min = bset.iter().min_by_key(|x| (x.1).0 + (x.1).1);
        let (best_point, (travel, dis)) = match min {
            Some(x) => x,
            None => return None,
        };
        let (best_point, (travel, dis)) = (*best_point, (*travel, *dis));
        kset.insert(best_point, (travel, dis));
        //map[best_point[1] as usize][best_point[0] as usize] = Clear(ClearType::Debug);
        //print_map(&map);
        bset.remove(&best_point);
        if best_point == fin {
            break Some((best_point, travel));
        }
        //expand boundary set
        for dir in [[0, 1], [1, 0], [0, -1], [-1, 0]].iter() {
            let x = best_point[0] + dir[0];
            let y = best_point[1] + dir[1];
            if y >= 0 && x >= 0 && y < map.len() as i64 && x < map[0].len() as i64 {
                match map[y as usize][x as usize] {
                    Wall | Door(_) => {}
                    _ => {
                        if !kset.contains_key(&[x, y]) {
                            bset.insert([x, y], (travel + 1, dist([x, y], fin)));
                        }
                    }
                }
            }
        }
    }
}
