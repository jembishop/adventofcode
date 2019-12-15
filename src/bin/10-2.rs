use num::integer::gcd;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::f64::INFINITY;
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
    let input = "
......#.#.
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
    let parsed = parse(&input).unwrap();
    let res = most_visible(&parsed);
    assert_eq!(res, 35);
}
fn main() {
    let mut file = File::open("input/10.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents = contents.trim().to_string();
    let base = (11, 19);
    let parsed = parse(&contents).unwrap();
    let d = destruction_order(base, &parsed);
    println!("{:?}", d[199]);
}

type Asteroid = (i64, i64);

#[derive(Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Direction {
    Down,
    Right,
    Up,
    Left,
}

#[derive(Hash, Debug, PartialEq, Eq, Copy, Clone)]
struct RationalPolar {
    r: usize,
    angle: Option<(usize, usize)>,
    dir: Direction,
}

impl RationalPolar {
    fn new(a: (i64, i64)) -> Self {
        use Direction::*;
        use Ordering::*;
        let (x, y) = a;
        assert_ne!((x, y), (0, 0));
        match (x.abs() as usize, y.abs() as usize, x.cmp(&0), y.cmp(&0)) {
            (0, uy, _, Greater) => RationalPolar {
                r: uy,
                angle: None,
                dir: Up,
            },
            (0, uy, _, Less) => RationalPolar {
                r: uy,
                angle: None,
                dir: Down,
            },
            (ux, 0, Greater, _) => RationalPolar {
                r: ux,
                angle: None,
                dir: Right,
            },
            (ux, 0, Less, _) => RationalPolar {
                r: ux,
                angle: None,
                dir: Left,
            },
            (ux, uy, xs, ys) => {
                let z = gcd(ux, uy);
                RationalPolar {
                    r: z,
                    angle: Some((ux / z, uy / z)),
                    dir: match (xs, ys) {
                        (Greater, Greater) => Right,
                        (Greater, Less) => Down,
                        (Less, Less) => Left,
                        (Less, Greater) => Up,
                        _ => unreachable!(),
                    },
                }
            }
        }
    }
    fn to_cartesian(&self) -> (i64, i64) {
        use Direction::*;
        match self {
            RationalPolar {
                r,
                angle: None,
                dir,
            } => {
                let r = *r as i64;
                match dir {
                    Up => (0, r),
                    Down => (0, -r),
                    Right => (r, 0),
                    Left => (-r, 0),
                }
            }
            RationalPolar {
                r,
                angle: Some((x, y)),
                dir,
            } => {
                let r = *r as i64;
                let x = *x as i64;
                let y = *y as i64;
                match dir {
                    Up => (-r * x, r * y),
                    Down =>(r * x, -r * y),
                    Right => (r * x, r * y),
                    Left => (-r * x, -r * y),
                }
            }
        }
    }
}
impl PartialOrd for RationalPolar {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        fn gradient(s: &RationalPolar) -> f64 {
            use Direction::*;
            match (&s.dir, &s.angle) {
                (Down, None) | (Up, None) => INFINITY,
                (Left, None) | (Right, None) => 0.0,
                (_, Some((x, y))) => (*y as f64) / (*x as f64),
            }
        }

        use Direction::*;
        use Ordering::*;
        let fin = match self.dir.cmp(&other.dir) {
            Equal => {
                if self.angle == other.angle {
                    self.r.cmp(&other.r)
                } else {
                    let grads = gradient(&self);
                    let grado = gradient(&other);
                    match self.dir {
                        Up | Down => grado.partial_cmp(&grads).unwrap(),
                        Right | Left => grads.partial_cmp(&grado).unwrap(),
                    }
                }
            }
            ord => ord,
        };
        Some(fin)
    }
}

impl Ord for RationalPolar {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
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

fn destruction_order(base: Asteroid, asteroids: &Vec<Asteroid>) -> Vec<Asteroid> {
    use Ordering::*;
    let mut rel: Vec<RationalPolar> = asteroids
        .iter()
        .filter(|x| **x != base)
        .map(|a| (a.0 - base.0, a.1 - base.1))
        .map(RationalPolar::new)
        .collect();
    rel.sort();
    let mut count = HashMap::new();
    let mut with_wind: Vec<(usize, RationalPolar)> = rel
        .iter()
        .map(|el| {
            let count = count.entry((el.dir, el.angle)).or_insert(0);
            *count += 1;
            (*count, *el)
        })
        .collect();
    with_wind.sort_by(|a, b| match a.0.cmp(&b.0) {
        Equal => a.1.cmp(&b.1),
        other => other,
    });
    with_wind.iter()
        .map(|x| &x.1)
        .map(RationalPolar::to_cartesian)
        .map(|a| (a.0 + base.0, a.1 + base.1))
        .collect()
}
