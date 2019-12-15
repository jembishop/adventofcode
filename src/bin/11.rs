use adventofcode::intcode::IntCodeComputer;
use adventofcode::intcode::IntCodeOutcome;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input/11.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents = contents.trim().to_string();
    let comp = IntCodeComputer::parse_program(&contents).unwrap();
    let mut walker = Walker::new(180, comp, 1);
    while walker.step() != WalkerOutcome::Finished {
    }
        print_grid(&walker.grid);
        println!("{}", walker.painted.len())
}

fn print_grid(grid: &Vec<Vec<i64>>) {
    for row in grid.iter().rev() {
        println!(
            "{:?}",
            row.iter()
                .map(|x| match x {
                    1 => "\u{2588}",
                    0 => " ",
                    _ => "?",
                })
                .collect::<Vec<&str>>()
                .join("")
        );
    }
}

struct Walker {
    grid: Vec<Vec<i64>>,
    position: (usize, usize),
    direction: u32,
    comp: IntCodeComputer,
    painted: HashMap<(usize, usize), usize>,
}

#[derive(PartialEq, Eq)]
enum WalkerOutcome {
    Continue,
    Finished,
}

impl Walker {
    fn new(size: usize, comp: IntCodeComputer, start_panel_col: i64) -> Self {
        let mut res = Self {
            grid: vec![vec![0; size]; size],
            position: (size / 2, size / 2),
            direction: 0,
            comp,
            painted: HashMap::new(),
        };
        let (x, y) = res.position;
        res.grid[y][x] = start_panel_col;    
        res
    }
    fn step(&mut self) -> WalkerOutcome {
        let (x, y) = self.position;
        let panel = self.grid[y][x];
        match self.comp.execute(&vec![panel]).unwrap() {
            IntCodeOutcome::Finished(_) => WalkerOutcome::Finished,
            IntCodeOutcome::NeedInput(res) => {
                self.grid[y][x] = res[0];
                *self.painted.entry(self.position).or_insert(0) += 1;
                self.direction = (self.direction
                    + (match res[1] {
                        0 => 3,
                        1 => 1,
                        _ => unreachable!(),
                    }))
                    % 4 as u32;
                self.position = match self.direction {
                    0 => (x, y + 1),
                    1 => (x + 1, y),
                    2 => (x, y - 1),
                    3 => (x - 1, y),
                    _ => unreachable!(),
                };
                WalkerOutcome::Continue
            }
        }
    }
}
