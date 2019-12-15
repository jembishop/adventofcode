use num::integer::gcd;
use std::collections::HashSet;
use std::fs::File;
use std::hash::Hash;
use std::io::prelude::*;

#[test]
fn test_most_visible() {
    let input = ".#..#
.....
#####
....#
...##"
        .to_string();
    let parsed = parse(&input).unwrap();
    let res = most_visible(&parsed);
    assert_eq!(res, 8);
    let input = "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";
    let parsed = parse(&input).unwrap();
    let res = most_visible(&parsed);
    assert_eq!(res, 33);
    let input = "#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.";
    let parsed = parse(&input).unwrap();
    let res = most_visible(&parsed);
    assert_eq!(res, 35);
}
fn main() {
    let mut file = File::open("input/10.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents = contents.trim().to_string();
    let parsed = parse(&contents).unwrap();
    let res = most_visible(&parsed);
    println!("{:?}", res);
}

type Asteroid = (i64, i64);

#[derive(Hash, Debug, PartialEq, Eq)]
enum Quadrant {
    One,
    Two,
    Three,
    Four,
}

#[derive(Hash, Debug, PartialEq, Eq)]
enum RationalDirection {
    Norm((usize, usize), Quadrant),
    Inf(bool),
    Zero(bool),
}

impl RationalDirection {
    fn new(a: Asteroid) -> Self {
        use RationalDirection::*;
        let (x, y) = a;
        assert_ne!((x, y), (0, 0));
        let x = match (
            x.abs() as usize,
            y.abs() as usize,
            x.is_positive(),
            y.is_positive(),
        ) {
            (0, _, _, true) => Inf(true),
            (0, _, _, false) => Inf(false),
            (_, 0, true, _) => Zero(true),
            (_, 0, false, _) => Zero(false),
            (ux, uy, xp, yp) => {
                use Quadrant::*;
                let quad = match (xp, yp) {
                    (true, true) => One,
                    (true, false) => Two,
                    (false, false) => Three,
                    (false, true) => Four,
                };

                let z = gcd(ux, uy) as usize;
                Norm((ux / z, uy / z), quad)
            }
        };
        x
    }
}

#[derive(Debug)]
struct AsteroidParseError;

fn parse(input: &str) -> Result<Vec<Asteroid>, AsteroidParseError> {
    input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, ch)| match ch {
                    '#' => Some(Ok((x as i64, y as i64))),
                    '.' => None,
                    _ => Some(Err(AsteroidParseError)),
                })
        })
        .flatten()
        .collect()
}

fn most_visible(asteroids: &Vec<Asteroid>) -> Asteroid {
    *asteroids
        .iter()
        .max_by_key(|a1| {
         let n = asteroids
                .iter()
                .map(|a2| (a2.0 - a1.0, a1.1 - a2.1))
                .filter_map(|x| if x == (0, 0) { None } else { Some(x) })
                .map(|(x, y)| RationalDirection::new((x, y)))
                .collect::<HashSet<_>>().len();
        println!("{:?} has {} visible",a1, n);
         n
        })
        .unwrap()
}
