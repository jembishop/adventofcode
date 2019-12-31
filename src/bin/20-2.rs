use ndarray::{array, Array2, Axis};
use rand::{seq::SliceRandom, thread_rng};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

#[allow(dead_code)]

fn main() {
    let mut file = File::open("input/20.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut grid = parse_to_grid(&contents);
    fill_in_dead_ends(&mut grid);
    print_map(&grid);
    //let cons = best_route(&grid);
    let cons = best_route(&grid);
    println!("{:?}", cons);
   // for x in cons.iter() {
   //     println!(
   //         "{:?}<->{:?}, {}, {}",
   //         pts(&grid[x.0]),
   //         pts(&grid[x.1]),
   //         x.2,
   //         x.3
   //     )
   // }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Tile {
    Wall,
    Clear,
    Portal { name: [char; 2], outer: bool },
}
fn pts(t: &Tile) -> String {
    if let Tile::Portal { name: x, .. } = t {
        x.iter().collect()
    } else {
        panic!()
    }
}
fn print_map(map: &Array2<Tile>) {
    use Tile::*;
    for l in map.axis_iter(Axis(1)) {
        println!(
            "{}",
            l.iter()
                .map(|x| match x {
                    Wall => '\u{2588}',
                    Portal { name, outer } =>
                        if *outer {
                            name[0]
                        } else {
                            name[0].to_ascii_lowercase()
                        },
                    _ => ' ',
                })
                .collect::<String>()
        );
    }
}

fn find_route(grid: &Array2<Tile>) {
    let mut start = grid
        .indexed_iter()
        .find(|(_, el)| {
            **el == Tile::Portal {
                name: ['A', 'A'],
                outer: true,
            }
        })
        .unwrap()
        .0;
}

//fn form_routes(grid: &Array2<Tile>) -> Vec<([usize; 2], [usize; 2], usize, i32)> {
//    let mut nodes: Vec<([usize; 2], [usize; 2], usize, i32)> = vec![];
//    for _ in 0..1000 {
//        for (i, el) in grid.indexed_iter() {
//            if let Tile::Portal { name, outer } = el {
//                let start = [i.0, i.1];
//                let (end, dist, re) = rand_route(grid, start);
//                if nodes.iter().find(|x| (x.0 == start && x.1 == end)) == None {
//                    nodes.push((start, end, dist, re))
//                }
//            }
//        }
//    }
//    loop {
//        let mut uniques = vec![];
//        for (l, k) in nodes.iter().enumerate() {
//            let mut unique = true;
//            for (i, j) in nodes.iter().enumerate() {
//                if i != l && (j.0 == k.0) {
//                    unique = false
//                };
//            }
//            if unique {
//                uniques.push(*k)
//            }
//        }
//        for u in uniques.iter() {
//            nodes.retain(|x| x != u);
//            for n in nodes.iter_mut() {
//                if pts(&grid[n.1]) == pts(&grid[u.0]) && n.0 != u.1 {
//                    //            println!("{:?} ---> {:?}", pts(&grid[n.0]), pts(&grid[n.1]));
//                    n.1 = u.1;
//                    n.2 += u.2;
//                    n.3 += u.3;
//                }
//            }
//        }
//
//        if uniques.len() == 0 {
//            nodes.sort_by_key(|x| x.2);
//            return nodes;
//        }
//    }
    //let matching  = grid
    //    .indexed_iter()
    //    .filter_map(|(po, el)| {
    //        if let Tile::Portal { name, .. } = el {
    //            Some((po, name))
    //        } else {
    //            None
    //        }
    //    })
    //    .find(|(po, el)| **el == x && (po != co))
    //    .unwrap()
    //    .0;
    //}
//}

fn best_route(grid: &Array2<Tile>) -> usize {
    let mut best = 1000000;
    for _ in 0..10000 {
        let c = rand_route(grid);
    //    println!("-----");
        if c < best {
            best = c;
            println!("FOUND       -------  {:?}", best);
        };
    }
    best
}

fn rand_route(grid: &Array2<Tile>) -> usize {
    let mut p = grid
        .indexed_iter()
        .find(|(_, el)| **el == Tile::Portal{name: ['A', 'A'], outer: true})
        .unwrap()
        .0;
    let mut dist = 0;
    let mut rlevel = 0;
    let mut last = p;
    for _ in 0..10000 {
        //println!("p {:?}, last {:?}", p, last); 
        let ll = p;
        let mut tile_coord = [
            (p.0 + 1, p.1),
            (p.0 - 1, p.1),
            (p.0, p.1 + 1),
            (p.0, p.1 - 1),
        ];
        tile_coord.shuffle(&mut thread_rng());
        for co in tile_coord.iter() {
            if *co == last {continue}
            match grid[*co] {
                Tile::Clear => {
                    dist += 1;
                    p = *co;
                    break;
                }
                Tile::Portal {
                    name: ['Z', 'Z'], ..
                } => {
                    if rlevel == 0 {
                        println!("FOUND       -------  {:?}", dist + 1);
                        return dist + 1;
                    }
                }
                Tile::Portal{name: ['A', 'A'], ..} => (),
                Tile::Portal { name: x, outer } => {
                    dist += 2;
                    if rlevel == 0 && outer {
                        continue;
                    }
                    if outer {
                        rlevel -= 1
                    } else {
                        rlevel += 1;
                        if rlevel > 40 {return 203958439}

                    }
                    //println!("{:?}, {:?}, {:?} {}", x, p, dist, rlevel);
                    p = grid
                        .indexed_iter()
                        .find(|(po, el)| {
                            if let Tile::Portal { name, .. } = **el {
                                po != co && x == name
                            } else {
                                false
                            }
                        })
                        .unwrap()
                        .0;
                    break;
                }
                _ => (),
            }
        }
        if ll == p {return 23452345} 
        last = ll;
    }
    1000000
}

//fn rand_route(grid: &Array2<Tile>, start: [usize; 2]) -> ([usize; 2], usize, i32) {
//    let mut dist = 0;
//    let mut p = start;
//    let mut last = start;
//    loop {
//        //    println!("{:?}", p);
//        let mut tile_coord = [
//            [p[0] + 1, p[1]],
//            [p[0] - 1, p[1]],
//            [p[0], p[1] + 1],
//            [p[0], p[1] - 1],
//        ];
//        tile_coord.shuffle(&mut thread_rng());
//        for co in tile_coord.iter() {
//            if *co == last {
//                continue;
//            }
//            match grid[*co] {
//                Tile::Clear => {
//                    dist += 1;
//                    last = p;
//                    p = *co;
//                    break;
//                }
//                Tile::Portal { name: _, outer } => {
//                    dist += 2;
//                    let o = if let Tile::Portal { name: _, outer: o } = grid[start] {
//                        o
//                    } else {
//                        panic!()
//                    };
//                    let rdiff = match (o, outer) {
//                        (true, true) => 0,
//                        (false, false) => 0,
//                        (true, false) => 1,
//                        (false, true) => -1,
//                    };
//                    return (*co, dist, rdiff);
//                }
//                _ => (),
//            }
//        }
//    }
//}

fn fill_in_dead_ends(grid: &mut Array2<Tile>) {
    let mut cleared = false;
    while !cleared {
        cleared = true;
        for x in 1..grid.shape()[0] - 1 {
            for y in 1..grid.shape()[1] - 1 {
                let p = [x, y];
                let tiles = [
                    grid[[p[0] + 1, p[1]]],
                    grid[[p[0] - 1, p[1]]],
                    grid[[p[0], p[1] + 1]],
                    grid[[p[0], p[1] - 1]],
                ];
                if tiles.iter().filter(|x| **x == Tile::Wall).count() >= 3 {
                    if grid[p] == Tile::Clear {
                        cleared = false;
                        grid[p] = Tile::Wall;
                    }
                }
            }
        }
    }
}

fn parse_to_grid(input: &str) -> Array2<Tile> {
    let width = input.chars().position(|x| x == '\n').unwrap();
    let height = input.lines().count();
    let input = Array2::from_shape_vec(
        (height, width),
        input
            .lines()
            .map(|x| x.chars())
            .flatten()
            .collect::<Vec<char>>(),
    )
    .unwrap()
    .reversed_axes();
    //println!("{:#?},", input);
    let width = width - 2;
    let height = height - 2;
    let mut out = Array2::from_elem((width, height), Tile::Wall);
    for x in 1..width - 1 {
        for y in 1..height - 1 {
            let f = [x, y];
            let p = [x + 1, y + 1];
            let ch = input[p];
            let el = &mut out[f];
            let outer = x == 1 || x == width - 2 || y == 1 || y == height - 2;
            match ch {
                ' ' | '#' => *el = Tile::Wall,
                '.' => match (
                    input[[p[0] + 1, p[1]]],
                    input[[p[0] - 1, p[1]]],
                    input[[p[0], p[1] + 1]],
                    input[[p[0], p[1] - 1]],
                ) {
                    (c @ 'A'..='Z', _, _, _) => {
                        *el = Tile::Portal {
                            name: [c, input[[p[0] + 2, p[1]]]],
                            outer,
                        }
                    }
                    (_, c @ 'A'..='Z', _, _) => {
                        *el = Tile::Portal {
                            name: [input[[p[0] - 2, p[1]]], c],
                            outer,
                        }
                    }
                    (_, _, c @ 'A'..='Z', _) => {
                        *el = Tile::Portal {
                            name: [c, input[[p[0], p[1] + 2]]],
                            outer,
                        }
                    }
                    (_, _, _, c @ 'A'..='Z') => {
                        *el = Tile::Portal {
                            name: [input[[p[0], p[1] - 2]], c],
                            outer,
                        }
                    }
                    (_, _, _, _) => *el = Tile::Clear,
                },
                'A'..='Z' => continue,
                _ => unreachable!(),
            };
        }
    }
    out
}
