use std::fs::File;
use std::io::prelude::*;
use std::iter;

#[test]
fn test_check() {
    assert!(!check(&111111));
    assert!(!check(&223450));
    assert!(!check(&123789));
    assert!(check(&146689));
    assert!(!check(&946689));
    assert!(check(&1466888));
    assert!(!check(&14666888));
    assert!(!check(&777888999));
    assert!(check(&14788));
}

fn main() {
    let mut file = File::open("input/4.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
    let range: Vec<_> = contents
        .split("-")
        .map(|x| {
            println!("value: {}", x);
            x.trim().parse::<i64>().unwrap()
        })
        .collect();
    let start = range[0];
    //add one to last to make usual range
    let end = range[1] + 1;
    println!("begin");

    fn count(start: i64, end: i64) -> usize {
        (start..end).filter(check).count()
    }

    let total = count(start, end);
    println!("{}", total);
}

fn check(num: &i64) -> bool {
    let mut last: char = '0';
    let mut good = false;
    let mut n_row = 0;
    // chain on colon to give it high terminator
    for c in num.to_string().chars().chain(iter::once(':')) {
        if c < last {
            return false;
        }
        if c == last {
            n_row += 1;
        } else {
            if n_row == 1 {
                good = true
            };
            n_row = 0;
        }
        last = c;
    }
    good
}
