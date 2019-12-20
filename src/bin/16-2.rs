use std::fs::File;
use std::io::prelude::*;

//sorry this puzzle was bullshit did not enjoy

const OFFSET: usize = 5976521;

fn main() {
    let mut save = File::open("input/16.txt").unwrap();
    let mut contents = String::new();
    save.read_to_string(&mut contents).unwrap();
    contents = contents.trim().to_string();
    let n = 10_000;
    let off = OFFSET;
    let msg_len = 8;
    // let contents = "12345678";
    // let n = 20;
    // let off = 130;
    // let msg_len = 2;

    let len = contents.len();
    let offset_idx = off % len;
    assert!(!(offset_idx + msg_len > contents.len()));
    let extended = contents[offset_idx..]
        .chars()
        .chain(contents.chars().cycle().take(len * ((n * len - off) / len)))
        .collect::<String>();
    println!("input len {:?}", extended.len());
    let p = phase(&parse(&extended), 100);
    println!("{:?}", &p[..msg_len])
}

fn phase(inp: &Vec<u8>, n: usize) -> Vec<u8> {
    //println!("{:?}", inp);
    let mut old = inp.clone();
    for i in 0..n {
        let mut new = Vec::with_capacity(inp.len());
        let mut curr = 0;
        for i in old.iter().rev() {
            curr = (curr + i) % 10;
            new.insert(0, curr);
        }
        println!("{:?}", i);
        old = new;
    }
    //println!("{:?}", new);
    old
}

fn parse(input: &str) -> Vec<u8> {
    input
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u8)
        .collect()
}

