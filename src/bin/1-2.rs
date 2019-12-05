use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input/1.txt")?;
    let reader = BufReader::new(file);
    let res: i64 = reader
        .lines()
        .map(|line| {
            if let Ok(x) = line.unwrap().parse::<i64>() {
                fuel(x)
            } else {
                0
            }
        })
        .sum();
    println!("{}", res);
    Ok(())
}

fn fuel(mut x: i64) -> i64 {
    let mut sum = 0;
    loop {
        x = x/3 - 2;
        if x < 0 {break sum;}
        sum +=x;
    }
}

