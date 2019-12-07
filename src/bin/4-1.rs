use std::fs::File;
use std::io::prelude::*;
use std::thread;

#[test]
fn test_check() {
    assert!(check(&111111));
    assert!(!check(&223450));
    assert!(!check(&123789));
    assert!(!check(&1466891));
    assert!(!check(&946689));
}

fn main() {
    let mut file = File::open("input/4.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
    let mut range: Vec<_> = contents.split("-").map(|x| {println!("value: {}", x); x.trim().parse::<i64>().unwrap()}).collect();
    //add one to last to make usual range 
    range[1] += 1;
    let start = range[0];
    let end = range[1];
    let split = (end - start) + start; 
    println!("begin");

    fn count(start: i64, end: i64) -> usize {
        (start..end).filter(check).count()
    }

    let handle = thread::spawn(move || { 
        count(start, split)
    }
    );
        
    let total = count(split, end) + handle.join().unwrap(); 
    println!("{}", total);


}

fn check(num: &i64) -> bool {
    let mut last: char = '0';
    let mut good = false;
    for c in num.to_string().chars(){
        if c < last {
            return false;
        }
        if c == last {
            good = true;
        }
        last = c;
    }
    good
}






