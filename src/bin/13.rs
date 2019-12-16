use adventofcode::intcode::IntCodeComputer;
use adventofcode::intcode::IntCodeOutcome;
use device_query::{keymap, DeviceQuery, DeviceState};
use itertools::Itertools;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::{thread, time};

const SCREEN_HEIGHT: usize = 20;
const SCREEN_WIDTH: usize = 50;

#[derive(Deserialize, Serialize)]
struct State {
    comp: IntCodeComputer,
    screen: Vec<Vec<char>>,
}
fn main() {
    loop {
        let state = match File::open("save.json") {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                contents = contents.trim().to_string();
                serde_json::from_str(&contents).unwrap()
            }
            Err(_) => {
                let mut file = File::open("input/13.txt").unwrap();
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                contents = contents.trim().to_string();
                let mut comp = IntCodeComputer::parse_program(&contents).unwrap();
                let screen = vec![vec![' '; SCREEN_WIDTH]; SCREEN_HEIGHT + 1];
                comp.state[0] = 2;
                State { comp, screen }
            }
        };
        let State {
            mut comp,
            mut screen,
        } = state;
        let mut joystick = 0;
        let device_state = DeviceState::new();
        loop {
            use IntCodeOutcome::*;
            let mut finished = false;
            let res = comp.execute(&vec![joystick]).unwrap();
            let res = match res {
                Finished(out) => {
                    finished = true;
                    out
                }
                NeedInput(out) => out,
            };
            let tiles = &process(res);
            render(tiles, &mut screen);
            if finished {
                    serde_json::to_writer(
                        &File::create("fin.json").unwrap(),
                        &State {
                            comp: comp.clone(),
                            screen: screen.clone(),
                        },
                    )
                    .unwrap();
                break;
            };
            draw(&screen);
            thread::sleep(time::Duration::from_millis(200));
            let keys = device_state.get_keys();
            use keymap::Keycode;
            match keys.get(0) {
                Some(Keycode::J) => joystick = -1,
                Some(Keycode::K) => joystick = 1,
                Some(Keycode::Space) => serde_json::to_writer(
                    &File::create("save.json").unwrap(),
                    &State {
                        comp: comp.clone(),
                        screen: screen.clone(),
                    },
                )
                .unwrap(),
                _ => joystick = 0,
            }
            println!("{}[2J", 27 as char);
        }
    }
}

#[derive(TryFromPrimitive)]
#[repr(u32)]
enum TileId {
    Empty = 0,
    Wall,
    Block,
    HPaddle,
    Ball,
}

enum OutputIns {
    Tile { pos: [u32; 2], id: TileId },
    Score(i64),
}

fn process(input: Vec<i64>) -> Vec<OutputIns> {
    use OutputIns::*;
    input
        .chunks_exact(3)
        .map(|v| {
            let v: Vec<i64> = Vec::from(v);
            if v[0] == -1 && v[1] == 0 {
                return Score(v[2]);
            };
            Tile {
                id: TileId::try_from_primitive(v[2] as u32).unwrap(),
                pos: [v[0] as u32, v[1] as u32],
            }
        })
        .collect()
}

fn render(tiles: &Vec<OutputIns>, screen: &mut Vec<Vec<char>>) {
    use OutputIns::*;
    use TileId::*;
    for tile in tiles.iter() {
        match tile {
            Tile { pos, id } => {
                let (x, y) = (pos[0], pos[1]);
                screen[y as usize][x as usize] = match id {
                    Empty => ' ',
                    Wall => '\u{2591}',
                    Block => '\u{2588}',
                    HPaddle => '\u{2014}',
                    Ball => '\u{25CF}',
                }
            }
            Score(x) => {
                screen[SCREEN_HEIGHT] = x
                    .to_string()
                    .chars()
                    .pad_using(SCREEN_WIDTH, |_| ' ')
                    .collect()
            }
        }
    }
}

fn draw(screen: &Vec<Vec<char>>) {
    for row in screen.iter() {
        println!("{}", row.iter().collect::<String>());
    }
}
