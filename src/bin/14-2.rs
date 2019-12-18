use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[test]
fn test_ore_req() {
    let input = "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL";
    let parsed = parse(input);
    let fuel = ore_req(&parsed);
    assert_eq!(fuel, 31);
    let input = "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL
";

    let parsed = parse(input);
    let fuel = ore_req(&parsed);
    assert_eq!(fuel, 165);
    let input = "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
    let parsed = parse(input);
    let fuel = ore_req(&parsed);
    assert_eq!(fuel, 13312);

    let input = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF";

    let parsed = parse(input);
    let fuel = ore_req(&parsed);
    assert_eq!(fuel, 180697);

    let input = "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX";

    let parsed = parse(input);
    let fuel = ore_req(&parsed);
    assert_eq!(fuel, 2210736);
}

fn main() {
    let mut file = File::open("input/14.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let input = &contents.trim().to_string();
    let parsed = parse(input);
    let ore = trillion_fuel(&parsed);
    println!("{:#?}", ore);
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Entry {
    name: String,
    amount: u64,
}

#[derive(Debug, PartialEq, Eq)]
struct Reaction {
    ing: Vec<Entry>,
    res: Entry,
}

fn make_levels(reactions: &Vec<Reaction>) -> HashMap<&String, u32> {
    let mut levels: HashMap<&String, u32> = HashMap::new();
    while levels.len() != reactions.len() {
        'outer: for re in reactions.iter() {
            if re.ing[0].name == "ORE" {
                levels.insert(&re.res.name, 1);
            } else {
                let mut level = 0;
                for i in re.ing.iter() {
                    match levels.get(&i.name) {
                        None => continue 'outer,
                        Some(x) => {
                            if *x > level {
                                level = *x
                            }
                        }
                    }
                }
                levels.insert(&re.res.name, level + 1);
            }
        }
    }
    levels
}

fn trillion_fuel(reactions: &Vec<Reaction>) -> u64 {
    //didn't even work in the end due to overflow, just guessed \_:)_/
    let mut lower = 1;
    let mut upper = 1_893_569;
    let target = 1_000_000_000_000;
    let mut rough = loop {
        let r1 = ore_req(reactions, lower);
        let r2 = ore_req(reactions, upper);
        println!("{}", r2);
        assert!(r2 > target);
        let new = (lower + upper) / 2;
        let rnew = ore_req(reactions, new);
        if ((target - 1000)..(target - 1000)).contains(&rnew) {
            break rnew;
        }
        if target > rnew {
            lower = rnew
        } else {
            upper = rnew
        }
    };
    while ore_req(reactions, rough) < target {
        rough += 1
    }
    rough - 1
}

fn ore_req(reactions: &Vec<Reaction>, fuel: u64) -> u64 {
    //this code is absolute shit
    let mut ing_list: HashMap<String, u64> = HashMap::new();
    let levels = make_levels(&reactions);
    ing_list.insert("FUEL".to_string(), fuel);
    while !(ing_list.contains_key("ORE") && ing_list.len() == 1) {
        let key_list = ing_list.keys().map(|x| x.clone()).collect::<Vec<String>>();
        let max_level = key_list
            .iter()
            .map(|x| levels.get(x).unwrap())
            .max()
            .unwrap();
        for name in key_list.iter().filter(|x| x != &"ORE") {
            //only expand things at max level
            if levels.get(name).unwrap() != max_level {
                continue;
            };
            let re: &Reaction = reactions.iter().find(|x| &x.res.name == name).unwrap();
            for i in re.ing.iter() {
                let ex = ing_list.get(name).unwrap();
                let remainder = ex % re.res.amount;
                let rounded = if remainder != 0 {
                    ex + re.res.amount - remainder
                } else {
                    *ex
                };
                let amount = i.amount * rounded / re.res.amount;
                ing_list
                    .entry(i.name.clone())
                    .and_modify(|x| *x += amount)
                    .or_insert(amount);
            }
            ing_list.remove(name);
        }
    }
    *ing_list.get("ORE").unwrap()
}

fn parse(input: &str) -> Vec<Reaction> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+ [A-Z]+)").unwrap();
    }
    input
        .lines()
        .map(|l| {
            let mut e: Vec<Entry> = RE
                .captures_iter(l)
                .map(|el| {
                    let p: Vec<&str> = el
                        .iter()
                        .map(|x| x.unwrap().as_str().split(" "))
                        .flatten()
                        .collect();
                    Entry {
                        name: p[1].to_string(),
                        amount: p[0].parse::<u64>().unwrap(),
                    }
                })
                .collect();
            let res = e.pop().unwrap();
            Reaction { ing: e, res }
        })
        .collect()
}
