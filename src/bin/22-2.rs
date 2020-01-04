use std::fs;

#[test]
fn test() {
    let case = "deal with increment 7
deal into new stack
deal into new stack";
    let mut deck = Deck::new(10);
    shuffle(&case, &mut deck);
    assert_eq!(deck.0, vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);
    println!("Case 1 passed");
    let case = "cut 6
deal with increment 7
deal into new stack";
    let mut deck = Deck::new(10);
    shuffle(&case, &mut deck);
    assert_eq!(deck.0, vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);
    println!("Case 2 passed");
    let case = "deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1";
    let mut deck = Deck::new(10);
    shuffle(&case, &mut deck);
    assert_eq!(deck.0, vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);
    println!("Case 3 passed");
}
fn main() {
    let contents = fs::read_to_string("input/22.txt").unwrap();
    let mut deck = Deck::new(119315717514047);
    shuffle(&contents, &mut deck);
    println!("{}", deck.0.iter().position(|x| *x == 2019).unwrap());
}

fn shuffle(inp: &str, deck: &mut Deck) {
    for l in inp.lines() {
        let ins = l.split(" ").collect::<Vec<&str>>();
        let ins = &ins[ins.len() - 2..];
        // println!("{:?}", deck.0);
        //println!("{:?}", ins);
        match ins[0] {
            "new" => deck.deal_into_new_stack(),
            "cut" => deck.cut(ins[1].parse::<i64>().unwrap()),
            "increment" => deck.deal_with_increment(ins[1].parse::<usize>().unwrap()),
            _ => unreachable!(),
        }
    }
}

struct BigDeck {
    size: usize,
    start_cards: Vec<usize>,
    end_cards: Vec<usize>,
}

impl BigDeck {
    fn new(n: usize, size: usize, bound_size: usize) -> Self {
        Self {
            start_cards: (0..bound_size).collect(),
            end_cards: (size - bound_size..size).collect(),
            size,
        }
    }

    fn deal_into_new_stack(&mut self) {
        std::mem::swap(&mut self.start_cards, &mut self.end_cards);
        self.start_cards.reverse();
        self.end_cards.reverse();
    }
    fn cut(&mut self, n: i64) {
        use std::cmp::Ordering::*;
        let mag = n.abs() as usize;
        match n.cmp(&0) {
            Greater => {
                for i in 0..mag {
                    self.end_cards.push(self.start_cards.remove(0))
                }
            }
            Less => {
                for i in 0..mag {
                    self.start_cards.insert(0, self.end_cards.pop().unwrap())
                }
            }
            Equal => (),
        };
    }
    fn deal_with_increment(&mut self, n: usize) {
    }
}
