#![allow(dead_code)]
#![allow(unused_imports)]
use std::fs;
use adventofcode::intcode::{IntCodeComputer, IntCodeOutcome};
use rand::{thread_rng, seq::SliceRandom};
use std::io;

const SIZE: usize = 100;

fn main() {
    use Direction::*;
    let mut d = Droid::new();
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();
        let dir = match guess.as_str() {
            "k\n" => Some(North),
            "l\n" => Some(East),
            "j\n" => Some(South),
            "h\n" => Some(West),
            _ => None 
        }; 
        let res = if let Some(x) = dir {
        d.mov(x) 
        } 
        else {
        d.cmd(&guess)
        };
        println!("{}", res);
    }
   // for _ in 0..1000000 {
   //     let mut rng = thread_rng();
   //     let choices = [North, South, East, West];
   //     let choice = choices.choose(&mut rng).unwrap();
   //     let r = d.mov(*choice);
   //     println!("{}", r);
   // }
}


struct Droid {
    comp: IntCodeComputer,
    pos: [usize;2]
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South, 
    West
}
impl Droid {
    fn new() -> Self {
        let contents = fs::read_to_string("input/25.txt").unwrap();
        let comp = IntCodeComputer::parse_program(contents.trim()).unwrap();
        Self {comp, pos: [SIZE/2, SIZE/2]}
    }
    fn cmd(&mut self, input: &str) -> String{
        use IntCodeOutcome::*;
        let ints = input.bytes().map(|x| x as u8 as i64).collect();
        let res = match self.comp.execute(&ints).unwrap(){
            NeedInput(x)=>x,
            Finished(x)=>x
        };
        res.iter().map(|x| *x as u8 as char).collect()

    }
    fn drop(&mut self, item: &str) -> String{
        self.cmd(&format!("drop {}\n", item))
    }
    fn take(&mut self, item: &str) -> String{
        self.cmd(&format!("take {}\n", item))
    }
    fn mov(&mut self, dir: Direction) -> String{
        use Direction::*;
        let s = match dir {
            North=>{self.pos[1] += 1;"north\n"},
            East=>{self.pos[0] += 1; "east\n"},
            South=>{self.pos[1] -= 1; "south\n"},
            West=>{self.pos[0] -= 1; "west\n"},
        };
        self.cmd(s)
    }
}
