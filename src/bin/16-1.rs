use std::fs::File;
use std::io::prelude::*;
use std::iter::repeat;


#[test]
fn test_multi_phase() {
    let input = "80871224585914546619083218645595";
    let parsed = parse(input);
    let mp = multi_phase(parsed, 100);
    assert_eq!(mp[..8].to_vec(), vec![2,4,1,7,6,1,7,6]);
}
fn main() {
    let mut save = File::open("input/16.txt").unwrap();
    let mut contents = String::new();
    save.read_to_string(&mut contents).unwrap();
    contents = contents.trim().to_string();
    println!("{:?}", &multi_phase(parse(&contents), 100)[..8]); 
}

fn parse(input: &str) -> Vec<i64> {
    input.chars().map(|x| x.to_digit(10).unwrap() as i64).collect()
}

fn make_pattern(phase: usize) -> impl Iterator<Item = i64> {
    repeat(0)
        .take(phase)
        .chain(repeat(1).take(phase))
        .chain(repeat(0).take(phase))
        .chain(repeat(-1).take(phase))
        .cycle()
        .skip(1)
}

fn multi_phase(mut inp: Vec<i64>, n: u32) -> Vec<i64> {
    for _ in 0..n {
    inp = (1..=inp.len()).map(|i| resolve(i, &inp)).collect::<Vec<i64>>()
    }
    inp
}

fn last_digit(x: i64) -> i64 {
    x.to_string().chars().last().unwrap().to_digit(10).unwrap() as i64
}

fn resolve(phase: usize, input: &Vec<i64>) -> i64 {
    last_digit(input
        .iter()
        .zip(make_pattern(phase))
        .map(|(x, c)| x * c)
        .sum::<i64>()
    )
}
