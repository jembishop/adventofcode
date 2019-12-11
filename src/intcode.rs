use std::iter::repeat;

#[test]
fn test_parse_op() {
    assert_eq!(parse_op(1002).unwrap(), (vec![false, true, false], 2));
    assert_eq!(parse_op(11004).unwrap(), (vec![false, true, true], 4));
    assert_eq!(parse_op(10102).unwrap(), (vec![true, false, true], 2));
    assert_eq!(parse_op(103).unwrap(), (vec![true, false, false], 3));
    assert_eq!(parse_op(2).unwrap(), (vec![false, false, false], 2));
    assert_eq!(parse_op(99).unwrap(), (vec![false, false, false], 99));
}

#[test]
fn test_resolve_flags() {
    let program = vec![1102, 30, 40, 5, 3, 0, 2];
    let (flags, opcode) = parse_op(program[0]).unwrap();
    let arg0 = resolve_flags(&program, 0, 0, &flags);
    let arg1 = resolve_flags(&program, 0, 1, &flags);
    assert_eq!(opcode, 2);
    assert_eq!(arg0, 30);
    assert_eq!(arg1, 40);
    let (flags, opcode) = parse_op(program[4]).unwrap();
    assert_eq!(opcode, 3);
    let arg0 = resolve_flags(&program, 4, 0, &flags);
    let arg1 = resolve_flags(&program, 4, 1, &flags);
    assert_eq!(arg0, 1102);
    assert_eq!(arg1, 40);
}

#[test]
fn test_execute() {
    let mut equal_to_8 = IntCodeComputer {
        state: vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8],pc:0
    };
    assert_eq!(
        equal_to_8.execute(&vec![8]).unwrap(),
        IntCodeOutcome::Finished(vec![1])
    );
    let mut equal_to_8 = IntCodeComputer {
        state: vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8],pc:0
    };
    assert_eq!(
        equal_to_8.execute(&vec![10]).unwrap(),
        IntCodeOutcome::Finished(vec![0])
    );
    let mut less_than_8 = IntCodeComputer {
        state: vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8],pc:0
    };
    assert_eq!(
        less_than_8.execute(&vec![5]).unwrap(),
        IntCodeOutcome::Finished(vec![1])
    );
    let mut less_than_8 = IntCodeComputer {
        state: vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8],pc:0
    };
    assert_eq!(
        less_than_8.execute(&vec![20]).unwrap(),
        IntCodeOutcome::Finished(vec![0])
    );

    let mut equal_to_8_immediate = IntCodeComputer {
        state: vec![3, 3, 1108, -1, 8, 3, 4, 3, 99],pc:0
    };
    assert_eq!(
        equal_to_8_immediate.execute(&vec![8]).unwrap(),
        IntCodeOutcome::Finished(vec![1])
    );
    let mut equal_to_8_immediate = IntCodeComputer {
        state: vec![3, 3, 1108, -1, 8, 3, 4, 3, 99],pc:0
    };
    assert_eq!(
        equal_to_8_immediate.execute(&vec![10]).unwrap(),
        IntCodeOutcome::Finished(vec![0])
    );

    let mut less_than_8_immediate = IntCodeComputer {
        state: vec![3, 3, 1107, -1, 8, 3, 4, 3, 99],pc:0
    };
    assert_eq!(
        less_than_8_immediate.execute(&vec![5]).unwrap(),
        IntCodeOutcome::Finished(vec![1])
    );
    let mut less_than_8_immediate = IntCodeComputer {
        state: vec![3, 3, 1107, -1, 8, 3, 4, 3, 99],pc:0
    };
    assert_eq!(
        less_than_8_immediate.execute(&vec![20]).unwrap(),
        IntCodeOutcome::Finished(vec![0])
    );
}

#[test]
fn test_execute_jump() {
    let mut non_zero = IntCodeComputer {
        state: vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],pc:0
    };
    assert_eq!(
        non_zero.execute(&vec![8]).unwrap(),
        IntCodeOutcome::Finished(vec![1])
    );
    let mut non_zero = IntCodeComputer {
        state: vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],pc:0
    };
    assert_eq!(
        non_zero.execute(&vec![0]).unwrap(),
        IntCodeOutcome::Finished(vec![0])
    );

    let mut non_zero_immediate = IntCodeComputer {
        state: vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],pc:0
    };
    assert_eq!(
        non_zero_immediate.execute(&vec![8]).unwrap(),
        IntCodeOutcome::Finished(vec![1])
    );
    let mut non_zero_immediate = IntCodeComputer {
        state: vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],pc:0
    };
    assert_eq!(
        non_zero_immediate.execute(&vec![0]).unwrap(),
        IntCodeOutcome::Finished(vec![0])
    );
}

#[test]
fn test_execute_complex() {
    let complex = IntCodeComputer {
        state: vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ],pc:0
    };
    assert_eq!(
        complex.clone().execute(&vec![8]).unwrap(),
        IntCodeOutcome::Finished(vec![1000])
    );
    assert_eq!(
        complex.clone().execute(&vec![18]).unwrap(),
        IntCodeOutcome::Finished(vec![1001])
    );
    assert_eq!(
        complex.clone().execute(&vec![3]).unwrap(),
        IntCodeOutcome::Finished(vec![999])
    );
}

fn parse_op(op: i64) -> Result<(Vec<bool>, i64), IntCodeError> {
    let flags = op
        .to_string()
        .chars()
        .rev()
        .chain(repeat('0'))
        .skip(2)
        .take(3)
        .map(|x| match x {
            '0' => Ok(false),
            '1' => Ok(true),
            _ => Err(IntCodeError::BadOp(op)),
        })
        .collect::<Result<Vec<bool>, _>>()?;
    let op_str = op.to_string();
    let opcode: i64 = if op_str.len() == 1 {
        op
    } else {
        op_str[op_str.len() - 2..].parse()?
    };
    Ok((flags, opcode))
}

fn resolve_flags(program: &Vec<i64>, i: usize, argno: i64, flags: &Vec<bool>) -> i64 {
    let argno = argno as usize;
    if flags[argno] {
        program[i + argno + 1]
    } else {
        program[program[i + argno + 1] as usize]
    }
}

#[derive(Debug)]
pub enum IntCodeError {
    CannotLoadProgram,
    CannotParseProgram,
    BadOpcode(i64),
    BadOp(i64),
}

impl From<std::num::ParseIntError> for IntCodeError {
    fn from(_err: std::num::ParseIntError) -> Self {
        IntCodeError::CannotParseProgram
    }
}

#[derive(PartialEq, Debug)]
pub enum IntCodeOutcome {
    Finished(Vec<i64>),
    NeedInput(Vec<i64>),
}

#[derive(PartialEq, Debug, Clone)]
pub struct IntCodeComputer {
    state: Vec<i64>,
    pc: usize
}

impl IntCodeComputer {
    pub fn parse_program(contents: &str) -> Result<Self, IntCodeError> {
        Ok(Self {
            state: contents
                .split(",")
                .map(|el| el.parse::<i64>())
                .collect::<Result<Vec<i64>, _>>()?,pc:0
        })
    }
    pub fn execute(&mut self, input: &Vec<i64>) -> Result<IntCodeOutcome, IntCodeError> {
        let mut input = input.iter();
        let mut output = vec![];
        loop {
            let op = self.state[self.pc];
            let (flags, opcode) = parse_op(op)?;
            if opcode == 99 {
                break;
            };
            match opcode {
                1 => {
                    let arg0 = resolve_flags(&self.state, self.pc, 0, &flags);
                    let arg1 = resolve_flags(&self.state, self.pc, 1, &flags);
                    let arg2 = self.state[self.pc + 3] as usize;
                    self.state[arg2] = arg0 + arg1;
                    self.pc += 4
                }
                2 => {
                    let arg0 = resolve_flags(&self.state, self.pc, 0, &flags);
                    let arg1 = resolve_flags(&self.state, self.pc, 1, &flags);
                    let arg2 = self.state[self.pc + 3] as usize;
                    self.state[arg2] = arg0 * arg1;
                    self.pc += 4
                }
                3 => {
                    let arg0 = self.state[self.pc + 1] as usize;
                    self.state[arg0] = match input.next() {
                        Some(x) => {
                            self.pc += 2;
                            *x
                        }
                        None => return Ok(IntCodeOutcome::NeedInput(output)),
                    };
                }
                4 => {
                    let arg0 = resolve_flags(&self.state, self.pc, 0, &flags);
                    output.push(arg0 as i64);
                    self.pc += 2
                }
                5 => {
                    let arg0 = resolve_flags(&self.state, self.pc, 0, &flags);
                    let arg1 = resolve_flags(&self.state, self.pc, 1, &flags);
                    if arg0 != 0 {
                        self.pc = arg1 as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                6 => {
                    let arg0 = resolve_flags(&self.state, self.pc, 0, &flags);
                    let arg1 = resolve_flags(&self.state, self.pc, 1, &flags);
                    if arg0 == 0 {
                        self.pc = arg1 as usize;
                    } else {
                        self.pc += 3;
                    }
                }

                7 => {
                    let arg0 = resolve_flags(&self.state, self.pc, 0, &flags);
                    let arg1 = resolve_flags(&self.state, self.pc, 1, &flags);
                    let arg2 = self.state[self.pc + 3] as usize;
                    if arg0 < arg1 {
                        self.state[arg2] = 1;
                    } else {
                        self.state[arg2] = 0;
                    }
                    self.pc += 4
                }
                8 => {
                    let arg0 = resolve_flags(&self.state, self.pc, 0, &flags);
                    let arg1 = resolve_flags(&self.state, self.pc, 1, &flags);
                    let arg2 = self.state[self.pc + 3] as usize;
                    if arg0 == arg1 {
                        self.state[arg2] = 1;
                    } else {
                        self.state[arg2] = 0;
                    }
                    self.pc += 4
                }

                e @ _ => return Err(IntCodeError::BadOpcode(e)),
            }
        }
        Ok(IntCodeOutcome::Finished(output))
    }
}
