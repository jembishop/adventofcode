use std::fs::File;
use std::io::prelude::*;
#[macro_use]
extern crate itertools;

#[derive(Copy, Clone, Debug)]
struct Segment {
    x: i64,
    y: i64,
    extent: i64,
}

#[derive(Debug)]
struct Wire {
    horizontal: Vec<Segment>,
    vertical: Vec<Segment>,
}
fn main() {
    let mut file = File::open("input/3.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let wires = parse(&contents);
    let wire1 = &wires[0];
    let wire2 = &wires[1];
    let intersections = find_intersections(wire1, wire2);
    let smallest_distance = intersections
        .iter()
        .map(|x| x.0.abs() + x.1.abs())
        .min()
        .unwrap();
    println!("found intersections: {:?}", intersections);
    println!("smallest sum {}", smallest_distance);
}

fn find_intersections(wire1: &Wire, wire2: &Wire) -> Vec<(i64, i64)> {
    let mut intersections = vec![];
    for (h, v) in iproduct!(&wire1.horizontal, &wire2.vertical) {
        if (h.y > v.y && h.y < v.extent) && (v.x > h.x && v.x < h.extent) {
            intersections.push((v.x, h.y));
        }
    }
    for (h, v) in iproduct!(&wire2.horizontal, &wire1.vertical) {
        if (h.y > v.y && h.y < v.extent) && (v.x > h.x && v.x < h.extent) {
            intersections.push((v.x, h.y));
        }
    }
    intersections
}

fn parse(contents: &String) -> Vec<Wire> {
    contents
        .trim()
        .split("\n")
        .map(|s| {
            let mut wire = Wire {
                horizontal: vec![],
                vertical: vec![],
            };
            let mut pos = (0, 0);
            for symb in s.split(",") {
                let dir = symb.as_bytes()[0] as char;
                let len = symb[1..].parse::<i64>().unwrap();
                match dir {
                    'R' => {
                        let new_x = pos.0 + len;
                        wire.horizontal.push(Segment {
                            x: pos.0,
                            y: pos.1,
                            extent: new_x,
                        });
                        pos.0 = new_x;
                    }
                    'L' => {
                        let new_x = pos.0 - len;
                        wire.horizontal.push(Segment {
                            x: new_x,
                            y: pos.1,
                            extent: pos.0,
                        });
                        pos.0 = new_x;
                    }
                    'U' => {
                        let new_y = pos.1 + len;
                        wire.vertical.push(Segment {
                            x: pos.0,
                            y: pos.1,
                            extent: new_y,
                        });
                        pos.1 = new_y;
                    }
                    'D' => {
                        let new_y = pos.1 - len;
                        wire.vertical.push(Segment {
                            x: pos.0,
                            y: new_y,
                            extent: pos.1,
                        });
                        pos.1 = new_y;
                    }
                    _ => panic!("unrecognised dir!"),
                }
            }
            wire
        })
        .collect()
}
