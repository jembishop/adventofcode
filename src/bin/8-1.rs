use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;
use counter::Counter;

type Layer = Vec<Vec<i64>>; 
type Image = Vec<Layer>;

fn main() {
    let mut file = File::open("input/8.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents = contents.trim().to_string();
    //let contents = "123456789100";
    let parsed = parse(&contents, (25,6));
    //println!("{:?}", parsed); 
    let few = fewest_0(parsed);
    println!("{:?}", n1_times_n2(few)); 
}

fn parse(input: &str, dims: (usize, usize)) -> Image {
    let mut res = vec![];
    for i in input
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i64)
        .chunks(dims.0)
        .into_iter()
        .map(|x| x.collect::<Vec<i64>>())
        .chunks(dims.1)
        .into_iter()
    {
        let x = i.collect::<Vec<Vec<i64>>>();
        if x.len() == dims.1 {
            res.push(x)
        }
    }
    res
}

fn fewest_0(image: Image) -> Layer {
    image.iter().min_by_key(|x| x.into_iter().flatten().filter(|x| **x==0).count()).unwrap().to_vec()
}

fn n1_times_n2(layer: Layer) -> usize {
    let c: Counter<i64> = layer.into_iter().flatten().collect();
    c.get(&1).unwrap()*c.get(&2).unwrap()
}
