use ndarray::{Array2, Axis};
use std::fs;

fn main() {
    let contents = fs::read_to_string("input/24.txt").unwrap();
    //let contents = "....#\n#..#.\n#..##\n..#..\n#....";
    let bugs = parse(&contents.trim());
    let mins = 200;
    //print_bugs(&bugs);
    let n = bugs.shape()[0];
    let m = 500;
    let mut levels = vec![Array2::from_elem((n, n), false); m];
    levels[m / 2] = bugs;
    for i in 1..=mins {
        evolve(&mut levels);
        for b in levels.iter().rev().take(10) {
            if i == mins {
    //        print_bugs(&b);
     //       println!();
            }
        }
    //    println!("-------");
    }
    println!("{}", levels.iter().map(|x| x.iter().filter(|x| **x)).flatten().count());
}

fn print_bugs(bugs: &Array2<bool>) {
    let n = bugs.shape()[0];
    for i in 0..n {
        for j in 0..n {
            match bugs[[j, i]] {
                true => print!("\u{2588}"),
                false => print!("\u{2591}"),
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

fn evolve(a: &mut Vec<Array2<bool>>) {
    let n = a[0].shape()[0];
    let mut counts: Vec<Array2<usize>> = vec![Array2::zeros((n, n)); a.len()];
    let center = [n / 2, n / 2];
    for (li, level) in a[1..a.len() - 1].iter().enumerate() {
        let li = li + 1;
        for (idx, el) in level.indexed_iter() {
            if [idx.0, idx.1] == center {
                continue;
            }
            for co in [[1, 0], [-1_i64, 0], [0, 1], [0, -1]].iter() {
                let point = [
                    (idx.0 as i64 + co[0]) as usize,
                    (idx.1 as i64 + co[1]) as usize,
                ];
                if point == center {
                    let sl = match co {
                        [1, 0] => a[li + 1].index_axis(Axis(0), 0),
                        [-1, 0] => a[li + 1].index_axis(Axis(0), n - 1),
                        [0, 1] => a[li + 1].index_axis(Axis(1), 0),
                        [0, -1] => a[li + 1].index_axis(Axis(1), n - 1),
                        _ => panic!(),
                    };
                    for els in sl.iter() {
                        if *els {
                            counts[li][[idx.0, idx.1]] += 1
                        }
                    }
                    let mut osl = match co {
                        [1, 0] => counts[li + 1].index_axis_mut(Axis(0), 0),
                        [-1, 0] => counts[li + 1].index_axis_mut(Axis(0), n - 1),
                        [0, 1] => counts[li + 1].index_axis_mut(Axis(1), 0),
                        [0, -1] => counts[li + 1].index_axis_mut(Axis(1), n - 1),
                        _ => panic!(),
                    };
                    for els in osl.iter_mut() {
                        if *el {
                            *els += 1;
                        }
                    }
                    continue;
                }

                if let Some(b) = level.get(point) {
                    if *b {
                        counts[li][[idx.0, idx.1]] += 1
                    }
                }
            }
        }
    }
    //    for (c, a) in counts.iter().zip(a.iter()) {
    //        println!("{:?}", a);
    //        println!("{:?}", c);
    //        println!("-----");
    //    }

    *a = a
        .iter()
        .zip(counts.iter())
        .map(|(arr, count)| {
            let mut new = arr.clone();
            for ((n, c), ar) in new.iter_mut().zip(count.iter()).zip(arr.iter()) {
                if *c != 1 && *ar {
                    *n = false
                }
                if [1, 2].contains(c) && !ar {
                    *n = true
                }
            }
            new[center] = false;
            new
        })
        .collect();
}
