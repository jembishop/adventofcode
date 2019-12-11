use counter::Counter;
use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

#[derive(Hash, TryFromPrimitive, Copy, Clone,PartialEq, Eq)]
#[repr(u32)]
enum Color {
    Black,
    White,
    Clear,
}

type Layer = Vec<Vec<Color>>;
type Image = Vec<Layer>;

fn main() {
    let mut file = File::open("input/8.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents = contents.trim().to_string();
    let parsed = parse(&contents, (25, 6));
    display(process(&parsed));
}

fn parse(input: &str, dims: (usize, usize)) -> Image {
    let mut res = vec![];
    for i in input
        .chars()
        .map(|x| Color::try_from(x.to_digit(10).unwrap()).unwrap())
        .chunks(dims.0)
        .into_iter()
        .map(|x| x.collect::<Vec<Color>>())
        .chunks(dims.1)
        .into_iter()
    {
        let x = i.collect::<Vec<Vec<Color>>>();
        if x.len() == dims.1 {
            res.push(x)
        }
    }
    res
}

fn display(layer: Layer) {
    let mut output = String::new();
    for row in layer.iter() {
        for pixel in row.iter() {
            output.push_str(match pixel {
                Color::White => "\u{2588}",
                Color::Black => " ",
                Color::Clear => "\u{2592}",
            }
            )
        }
    output.push_str("\n");
    }
    println!("{}", output);
}
fn process(image: &Image) -> Layer {
    let mut fin = image[0].clone();
    for y in 0..fin.len() {
        for x in 0..fin[0].len() {
            fin[y][x] = resolve_pixel(image, x, y);
        }
    }
    fin
}


fn resolve_pixel(image: &Image, x: usize, y:usize) -> Color {
    let mut layer_idx = 0;
    loop {
        let pixel = image[layer_idx][y][x];
        match pixel {
            Color::Clear => layer_idx+=1,
            other => break other
        }
    }
}


