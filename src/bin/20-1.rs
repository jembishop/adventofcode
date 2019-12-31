use ndarray::{array, Array2, Axis};
use rand::{seq::SliceRandom, thread_rng};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input/20.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut grid = parse_to_grid(&contents);
    println!("{:?}", grid);
    //print_map(&grid);
    //print_map(&grid);
    fill_in_dead_ends(&mut grid);
    print_map(&grid);
    let cons = best_route(&grid);
    println!("{:?}", cons);
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Tile {
    Wall,
    Clear,
    Portal([char; 2]),
}

fn print_map(map: &Array2<Tile>) {
    use Tile::*;
    for l in map.axis_iter(Axis(1)) {
        println!(
            "{}",
            l.iter()
                .map(|x| match x {
                    Wall => '\u{2588}',
                    Portal(x) => x[0],
                    _ => ' ',
                })
                .collect::<String>()
        );
    }
}

fn best_route(grid: &Array2<Tile>) -> usize {
    let mut best = 1000000;
    for _ in 0..10000 {
        let c = rand_route(grid);
        if c < best {
            best = c;
            println!("{:?}", best);
        };
    }
    best
}

fn rand_route(grid: &Array2<Tile>) -> usize {
    let mut p = grid
        .indexed_iter()
        .find(|(_, el)| **el == Tile::Portal(['A', 'A']))
        .unwrap().0;
    let mut dist = 0;
    let mut visted = HashSet::new();
    for _ in 0..700 {
        visted.insert(p);
        //println!("{:?}", p);
        let mut tile_coord = [
            (p.0 + 1, p.1),
            (p.0 - 1, p.1),
            (p.0, p.1 + 1),
            (p.0, p.1 - 1),
        ];
        tile_coord.shuffle(&mut thread_rng());
        for co in tile_coord.iter() {
            if visted.contains(co) {
                continue;
            };
            match grid[*co] {
                Tile::Clear => {
                    dist += 1;
                    p = *co;
                    break;
                }
                Tile::Portal(['Z', 'Z']) => return dist + 1,
                //Tile::Portal(['A', 'A']) => (), 
                Tile::Portal(x) => {
                    dist += 2; 
                    p = grid
                        .indexed_iter()
                        .find(|(po, el)| **el == Tile::Portal(x) && (po != co))
                        .unwrap()
                        .0;
                    //println!("{:?}, {:?}, {:?} {}", x, p, co, dist);
                    break;
                }
                _ => (),
            }
        }
    }
    1000000
}

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
            match ch {
                ' ' | '#' => *el = Tile::Wall,
                '.' => match (
                    input[[p[0] + 1, p[1]]],
                    input[[p[0] - 1, p[1]]],
                    input[[p[0], p[1] + 1]],
                    input[[p[0], p[1] - 1]],
                ) {
                    (c @ 'A'..='Z', _, _, _) => *el = Tile::Portal([c, input[[p[0] + 2, p[1]]]]),
                    (_, c @ 'A'..='Z', _, _) => *el = Tile::Portal([input[[p[0] - 2, p[1]]], c]),
                    (_, _, c @ 'A'..='Z', _) => *el = Tile::Portal([c, input[[p[0], p[1] + 2]]]),
                    (_, _, _, c @ 'A'..='Z') => *el = Tile::Portal([input[[p[0], p[1] - 2]], c]),
                    (_, _, _, _) => *el = Tile::Clear,
                },
                'A'..='Z' => continue,
                _ => unreachable!(),
            };
        }
    }
    out
}
