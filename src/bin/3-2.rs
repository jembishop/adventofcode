use std::fs::File;
use std::io::prelude::*;
#[macro_use]
extern crate itertools;

#[derive(Copy, Clone, Debug)]
struct Segment {
    x: i64,
    y: i64,
    extent: i64,
    cum_dist: i64,
}

#[derive(Debug)]
struct Wire {
    horizontal: Vec<Segment>,
    vertical: Vec<Segment>,
}
fn main() {
    //basically rewrote my old solution here
    let mut file = File::open("input/3.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    //contents = "R8,U5,L5,D3\nU7,R6,D4,L4".to_string();
    //contents = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"
    //contents = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7" .to_string();
    let wires = parse(&contents);
    let wire1 = &wires[0];
    let wire2 = &wires[1];
    let intersections = find_intersections(wire1, wire2);
    fn dist(x: i64, y: i64) -> i64 {
        x.abs() + y.abs()
    }
    let closest = intersections
        .iter()
        .min_by_key(|el| {
            let ((x, y), _) = el;
            dist(*x, *y)
        })
        .unwrap();
    let ((x, y), lens) = closest;
    println!(
        "Closest point to origin is: {:?}, with dist from origin of {} and total wire length of {}",
        closest,
        dist(*x, *y),
        dist(lens.0, lens.1)
    );
    let least_distance = intersections
        .iter()
        .min_by_key(|el| {
            let (_, (x, y)) = el;
            dist(*x, *y)
        })
        .unwrap();
    let ((x, y), lens) = least_distance;
    println!(
        "Lowest combined distance is : {:?}, with dist from origin of {} and total wire length of {}",
        closest,
        dist(*x, *y),
        dist(lens.0, lens.1)
    )
}

fn find_intersections(wire1: &Wire, wire2: &Wire) -> Vec<((i64, i64), (i64, i64))> {
    let mut intersections = vec![];
    for i in &[
        (&wire1.horizontal, &wire2.vertical),
        (&wire2.horizontal, &wire1.vertical),
    ] {
        let (a, b) = *i;
        for (h, v) in iproduct!(a, b) {
            let (h_x1, h_x2) = if h.x < h.extent {
                (h.x, h.extent)
            } else {
                (h.extent, h.x)
            };
            let (v_y1, v_y2) = if v.y < v.extent {
                (v.y, v.extent)
            } else {
                (v.extent, v.y)
            };

            if (h.y > v_y1 && h.y < v_y2) && (v.x > h_x1 && v.x < h_x2) {
                intersections.push((
                    (v.x, h.y),
                    (
                        h.cum_dist + (v.x - h.x).abs(),
                        v.cum_dist + (v.y - h.y).abs(),
                    ),
                ));
            }
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
            let mut cum_dist = 0;
            for symb in s.split(",") {
                let dir = symb.as_bytes()[0] as char;
                let len = symb[1..].parse::<i64>().unwrap();
                let new: i64;
                let h: bool;
                match dir {
                    'R' => {
                        new = pos.0 + len;
                        h = true;
                    }
                    'L' => {
                        new = pos.0 - len;
                        h = true;
                    }
                    'U' => {
                        new = pos.1 + len;
                        h = false;
                    }
                    'D' => {
                        new = pos.1 - len;
                        h = false;
                    }
                    _ => panic!("unrecognised dir!"),
                }
                let seg = Segment {
                    x: pos.0,
                    y: pos.1,
                    extent: new,
                    cum_dist: cum_dist,
                };
                if h {
                    wire.horizontal.push(seg);
                    pos.0 = new
                } else {
                    wire.vertical.push(seg);
                    pos.1 = new
                };
                cum_dist += len;
            }
            wire
        })
        .collect()
}
