use modulo::Mod;
use num_bigint::BigInt;
use num_traits::cast::ToPrimitive;
use num_traits::{One, Zero};
use std::fs;

#[test]
fn test() {}
fn main() {
    let contents = fs::read_to_string("input/22.txt").unwrap();
    let num: usize = 119315717514047;
    let times: usize = 101741582076661;
    //let num = 10007;
    //let times = 34;
    //let num: u64 = 10007;
//    let mut deck = BigDeck::new(BigInt::from(num));
    //let contents = "deal into new stack";
 //   shuffle(&contents, &mut deck, times);
  //  println!("time shuffle {} offset {}", times, deck.offset.to_usize().unwrap());
   // println!("time shuffle {} inc {}", times, deck.inc.to_usize().unwrap());

    let mut deck = BigDeck::new(BigInt::from(num));
    //let contents = "deal into new stack";
    shuffle(&contents, &mut deck, 1);
    let o = deck.offset;
    let i = deck.inc;
    let (off, inc) = multiply(o.clone(), i.clone(), times, num);
    println!("time {} matrix offset {}", times, off.to_usize().unwrap());
    println!("time {} matrix inc {}", times, inc.to_usize().unwrap());
    let res = ((off + inc * BigInt::from(2020)) % num).to_usize().unwrap();
    println!("result {}", res);
}

fn multiply(o: BigInt, i: BigInt, times: usize, size: usize) -> (BigInt, BigInt) {
    let mat = [One::one(), o, Zero::zero(), i];
    let mut run: usize = 2;
    let mut remaining = 0;
    let mut a = mat.clone();
    println!("run {}", times);
    if times == 1 {
        return (mat[1].clone(), mat[3].clone());
    }
    for _ in 0..(times - 1) {
        if run > times {
            remaining = times - run / 2;
            break;
        }
        a = [
            (a[0].clone() * a[0].clone() + a[1].clone() * a[2].clone()).modulo(size),
            (a[0].clone() * a[1].clone() + a[1].clone() * a[3].clone()).modulo(size),
            (a[2].clone() * a[0].clone() + a[3].clone() * a[2].clone()).modulo(size),
            (a[3].clone() * a[3].clone() + a[2].clone() * a[1].clone()).modulo(size),
        ];
        run *= 2;
    }
    if remaining == 0 {
        return (a[1].clone(), a[3].clone());
    }
    let (o1, o2) = multiply(mat[1].clone(), mat[3].clone(), remaining, size);
    let mato = [One::one(), o1.clone(), Zero::zero(), o2.clone()];

    let ret = [
        (a[0].clone() * mato[0].clone() + a[1].clone() * mato[2].clone()).modulo(size),
        (a[0].clone() * mato[1].clone() + a[1].clone() * mato[3].clone()).modulo(size),
        (a[2].clone() * mato[0].clone() + a[3].clone() * mato[2].clone()).modulo(size),
        (a[3].clone() * mato[3].clone() + a[2].clone() * mato[1].clone()).modulo(size),
    ];
    return (ret[1].clone(), ret[3].clone());
}

fn shuffle(inp: &str, deck: &mut BigDeck, times: usize) {
    for i in 1..=times {
        for l in inp.lines() {
            let ins = l.split(" ").collect::<Vec<&str>>();
            let ins = &ins[ins.len() - 2..];
            match ins[0] {
                "new" => deck.deal_into_new_stack(),
                "cut" => deck.cut(ins[1].parse::<BigInt>().unwrap()),
                "increment" => deck.deal_with_increment(ins[1].parse::<BigInt>().unwrap()),
                _ => unreachable!(),
            }
        }
    ////println!("time shuffle {} offset {}", i, deck.offset.to_usize().unwrap());
    //println!("time shuffle {} inc {}", i, deck.inc.to_usize().unwrap());
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct BigDeck {
    size: BigInt,
    offset: BigInt,
    inc: BigInt,
}

fn mod_inv(a: &BigInt, module: &BigInt) -> BigInt {
    let mut mn = (module.clone(), a.clone());
    let mut xy: (BigInt, BigInt) = (Zero::zero(), One::one());

    while mn.1 != Zero::zero() {
        xy = (
            xy.1.clone(),
            xy.0.clone() - (mn.0.clone() / mn.1.clone()) * xy.1.clone(),
        );
        mn = (mn.1.clone(), (mn.0.clone() % mn.1.clone()));
    }

    while xy.0 < Zero::zero() {
        xy.0 = xy.0 + module;
    }
    xy.0
}

impl BigDeck {
    fn new(size: BigInt) -> Self {
        Self {
            size,
            offset: Zero::zero(),
            inc: One::one(),
        }
    }

    fn deal_into_new_stack(&mut self) {
        self.offset = (self.offset.clone() - self.inc.clone()).modulo(self.size.clone());
        self.inc = (-self.inc.clone()).modulo(self.size.clone());
    }
    fn cut(&mut self, n: BigInt) {
        self.offset =
            (self.offset.clone() + self.inc.clone() * n.clone()).modulo(self.size.clone());
    }
    fn deal_with_increment(&mut self, n: BigInt) {
        self.inc = (self.inc.clone() * mod_inv(&n, &self.size)).modulo(self.size.clone());
    }
}
