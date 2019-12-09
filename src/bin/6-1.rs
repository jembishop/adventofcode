use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[test]
fn test_total_orbits() {
    let contents = "COM)B\nC)D\nD)E\nB)C\nE)F\nG)H\nD)I\nE)J\nB)G\nJ)K\nK)L".to_string();
    let enumed = enum_orbits(&contents);
    let sorted = sort_orbits(enumed);
    let tot = total_orbits(&sorted);
    assert_eq!(tot, 42);
    let contents = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L".to_string();
    let enumed = enum_orbits(&contents);
    let sorted = sort_orbits(enumed);
    let tot = total_orbits(&sorted);
    assert_eq!(tot, 42);
}

fn main() {
    let mut file = File::open("input/6.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let enumed = enum_orbits(&contents);
    let sorted = sort_orbits(enumed);
    let total_orbits = total_orbits(&sorted);
    println!("{:?}", total_orbits);
}

fn enum_orbits(contents: &str) -> Vec<(u32, u32)> {
    let mut enumed: Vec<(u32, u32)> = Vec::new();
        let mut curr_letter_code = 0;
        let mut letter_to_code = HashMap::new();
        letter_to_code.insert("COM", 0);
        for line in contents.lines() {
            let parsed: Vec<&str> = line.split(")").collect();
            let (parent, child) = (parsed[0], parsed[1]);
            let p_code = *letter_to_code.entry(parent).or_insert_with(|| {
                curr_letter_code += 1;
                curr_letter_code
            });
            let c_code = *letter_to_code.entry(child).or_insert_with(|| {
                curr_letter_code += 1;
                curr_letter_code
            });
            enumed.push((p_code, c_code));
    }
    enumed
}
fn sort_orbits(enumed: Vec<(u32, u32)> ) -> Vec<(u32, u32)> {
    // sort to possible insertion order
    let mut sorted: Vec<(u32, u32)> = Vec::new();
    let mut search_set = vec![0];
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

fn total_orbits(orbit_chart: &Vec<(u32, u32)>) -> u32 {
    let mut orbits: HashMap<u32, u32> = HashMap::new();
    orbits.insert(0, 0);
    for (parent, child) in orbit_chart {
        orbits.insert(*child, orbits.get(&parent).unwrap() + 1);
    }
    orbits.values().sum()
}
