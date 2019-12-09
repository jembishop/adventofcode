use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[test]
fn test_orbit_ancestry() {
    let sorted = vec![
        (Planet::Com, Planet::Other(1)),
        (Planet::Other(1), Planet::Other(2)),
        (Planet::Other(1), Planet::Other(6)),
        (Planet::Other(2), Planet::Other(3)),
        (Planet::Other(6), Planet::Other(7)),
        (Planet::Other(3), Planet::Other(4)),
        (Planet::Other(3), Planet::Other(8)),
        (Planet::Other(4), Planet::Other(5)),
        (Planet::Other(4), Planet::Other(9)),
        (Planet::Other(9), Planet::Other(10)),
        (Planet::Other(10), Planet::Other(11)),
    ];
    let ancestry = orbit_ancestry(&sorted, Planet::Other(10));
    println!("{:?}", ancestry);
    assert_eq!(
        ancestry,
        [
            Planet::Other(9),
            Planet::Other(4),
            Planet::Other(3),
            Planet::Other(2),
            Planet::Other(1),
            Planet::Com
        ]
    );
}

fn main() {
    let mut file = File::open("input/6.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    //let contents = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN".to_string();
    let enumed = enum_orbits(&contents);
    let sorted = sort_orbits(enumed);
    let transfer = min_transfer(&sorted, Planet::San, Planet::You);
    println!("{:?}", transfer);
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Planet {
    Com,
    San,
    You,
    Other(i32),
}

type OrbitMap = Vec<(Planet, Planet)>;

fn enum_orbits(contents: &str) -> OrbitMap {
    let mut enumed: OrbitMap = Vec::new();
    let mut curr_letter_code = 0;
    let mut letter_to_code = HashMap::new();
    letter_to_code.insert("COM", Planet::Com);
    letter_to_code.insert("SAN", Planet::San);
    letter_to_code.insert("YOU", Planet::You);
    for line in contents.lines() {
        let parsed: Vec<&str> = line.split(")").collect();
        let (parent, child) = (parsed[0], parsed[1]);
        let p_code = *letter_to_code.entry(parent).or_insert_with(|| {
            curr_letter_code += 1;
            Planet::Other(curr_letter_code)
        });
        let c_code = *letter_to_code.entry(child).or_insert_with(|| {
            curr_letter_code += 1;
            Planet::Other(curr_letter_code)
        });
        enumed.push((p_code, c_code));
    }
    enumed
}
fn sort_orbits(enumed: OrbitMap) -> OrbitMap {
    // sort to possible insertion order
    let mut sorted: OrbitMap = Vec::new();
    let mut search_set = vec![Planet::Com];
    let target_len = enumed.len();
    while sorted.len() != target_len {
        let mut new_search_set = vec![];
        for el in search_set {
            for (parent, child) in enumed.iter() {
                if *parent == el {
                    sorted.push((*parent, *child));
                    new_search_set.push(*child);
                }
            }
        }
        search_set = new_search_set;
    }
    sorted
}

fn orbit_ancestry(orbit_map: &OrbitMap, planet: Planet) -> Vec<Planet> {
    let mut curr = planet;
    let start_idx = orbit_map.iter().position(|&x| x.1 == curr).unwrap();
    let mut result = Vec::new();
    for (parent, child) in orbit_map[..=start_idx].iter().rev() {
        if *child == curr {
            result.push(*parent);
            curr = *parent;
        }
    }
    result
}

fn min_transfer(orbit_map: &OrbitMap, planet1: Planet, planet2: Planet) -> usize {
    let ancestry1 = orbit_ancestry(&orbit_map, planet1);
    let ancestry2 = orbit_ancestry(&orbit_map, planet2);
    for (idx1, planet) in ancestry1.iter().enumerate() {
        if let Some(idx2) = ancestry2.iter().position(|&x| x == *planet) {
            return idx1 + idx2;
        }
    }
    unreachable!();
}
