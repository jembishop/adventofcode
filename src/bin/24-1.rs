use ndarray::Array2;
use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input/24.txt").unwrap();
//    let contents = "....#
//#..#.
//#..##
//..#..
//#....";
    let mut bugs = parse(&contents.trim());
    let mut enc = HashSet::new();
    while enc.insert(bugs.clone()) {
        evolve(&mut bugs);
    }
    print_bugs(&bugs);
    println!("{}", bio_rating(&bugs));
}

fn bio_rating(bugs: &Array2<bool>) -> usize {
    let mut rating = 0;
    let mut counter = 0;
    let n = bugs.shape()[0];
    for j in 0..n {
        for i in 0..n {
            if bugs[[i, j]] {
                rating += 2_usize.pow(counter as u32)
            }
            counter += 1;
        }
    }
    rating
}
fn print_bugs(bugs: &Array2<bool>) {
    let n = bugs.shape()[0];
    for i in (0..n) {
        for j in 0..n {
            match bugs[[j, i]] {
                true => print!("\u{2588}"),
                false => print!(" "),
            }
        }
        println!();
    }
}

fn parse(input: &str) -> Array2<bool> {
    let n = 5;
    let mut res = Array2::from_elem((n, n), false);
    for (i, l) in input.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            if c == '#' {
                res[[j, i]] = true
            }
        }
    }
    res
}

fn evolve(a: &mut Array2<bool>) {
    let mut next = a.clone();
    for (idx, el) in a.indexed_iter() {
        let mut bc = 0;
        for co in [
            [idx.0 + 1, idx.1],
            [idx.0 - 1, idx.1],
            [idx.0, idx.1 + 1],
            [idx.0, idx.1 - 1],
        ]
        .iter()
        {
            if let Some(b) = a.get(*co) {
                if *b {
                    bc += 1
                }
            }
        }
        if bc != 1 && *el {
            next[idx] = false
        }
        if !el && [1, 2].contains(&bc) {
            next[idx] = true
        }
    }
    *a = next;
}
