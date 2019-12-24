use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::iter::repeat;

const INTCODE_MEM_SIZE: usize = 1000;

#[test]
fn test_parse_op() {
    use IntCodeInstruction::*;
    use IntCodeMode::*;
    assert_eq!(
        parse_op(1002),
        (vec![Position, Immediate, Position], Multiply)
    );
    assert_eq!(
        parse_op(11004),
        (vec![Position, Immediate, Immediate], Output)
    );
    assert_eq!(
        parse_op(10102),
        (vec![Immediate, Position, Immediate], Multiply)
    );
    assert_eq!(parse_op(103), (vec![Immediate, Position, Position], Input));
    assert_eq!(parse_op(2), (vec![Position, Position, Position], Multiply));
    assert_eq!(parse_op(99), (vec![Position, Position, Position], Halt));
}

#[test]
fn test_resolve_flags() {
    use IntCodeInstruction::*;
    let prog = "1102,30,40,5,3,0,2";
    let comp = IntCodeComputer::parse_program(prog).unwrap();
    let (flags, opcode) = parse_op(comp.state[0]);
    let arg0 = resolve_flags(&comp, 0, 0, &flags);
    let arg1 = resolve_flags(&comp, 0, 1, &flags);
    assert_eq!(opcode, Multiply);
    assert_eq!(arg0, 30);
    assert_eq!(arg1, 40);
    let comp = IntCodeComputer::parse_program(prog).unwrap();
    let (flags, opcode) = parse_op(comp.state[4]);
    assert_eq!(opcode, Input);
    let arg0 = resolve_flags(&comp, 4, 0, &flags);
    let arg1 = resolve_flags(&comp, 4, 1, &flags);
    assert_eq!(arg0, 1102);
    assert_eq!(arg1, 40);
}

#[test]
fn test_execute() {
    let prog = "3,9,8,9,10,9,4,9,99,-1,8";
    let mut comp = IntCodeComputer::parse_program(prog).unwrap();
    assert_eq!(
        comp.execute(&vec![8]).unwrap(),
        IntCodeOutcome::Finished(vec![1])
    );

    let prog = "3,9,7,9,10,9,4,9,99,-1,8";
    let mut comp = IntCodeComputer::parse_program(prog).unwrap();
    assert_eq!(
        comp.execute(&vec![3]).unwrap(),
        IntCodeOutcome::Finished(vec![1])
    );

    let prog = "3,3,1108,-1,8,3,4,3,99";
    let mut comp = IntCodeComputer::parse_program(prog).unwrap();
    assert_eq!(
        comp.execute(&vec![8]).unwrap(),
        IntCodeOutcome::Finished(vec![1])
    );
    let prog = "3,3,1108,-1,8,3,4,3,99";
    let mut comp = IntCodeComputer::parse_program(prog).unwrap();
    assert_eq!(
        comp.execute(&vec![10]).unwrap(),
        IntCodeOutcome::Finished(vec![0])
    );
    let prog = "3,3,1107,-1,8,3,4,3,99";
    let mut comp = IntCodeComputer::parse_program(prog).unwrap();
    assert_eq!(
        comp.execute(&vec![5]).unwrap(),
        IntCodeOutcome::Finished(vec![1])
    );
    let prog = "3,3,1107,-1,8,3,4,3,99";
    let mut comp = IntCodeComputer::parse_program(prog).unwrap();
    assert_eq!(
        comp.execute(&vec![20]).unwrap(),
        IntCodeOutcome::Finished(vec![0])
    );
}

#[test]
fn test_execute_jump() {
    let prog = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
    let mut comp = IntCodeComputer::parse_program(prog).unwrap();
    assert_eq!(
        comp.execute(&vec![8]).unwrap(),
        IntCodeOutcome::Finished(vec![1])
    );
    let prog = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
    let mut comp = IntCodeComputer::parse_program(prog).unwrap();
    assert_eq!(
        comp.execute(&vec![0]).unwrap(),
        IntCodeOutcome::Finished(vec![0])
    );

    let prog = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
    let mut comp = IntCodeComputer::parse_program(prog).unwrap();
    assert_eq!(
        comp.execute(&vec![8]).unwrap(),
        IntCodeOutcome::Finished(vec![1])
    );
    let prog = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
    let mut comp = IntCodeComputer::parse_program(prog).unwrap();
    assert_eq!(
        comp.execute(&vec![0]).unwrap(),
        IntCodeOutcome::Finished(vec![0])
    );
}

#[test]
fn test_execute_complex() {
    let prog = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,\
                0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,\
                20,1105,1,46,98,99";
    let comp = IntCodeComputer::parse_program(prog).unwrap();
    assert_eq!(
        comp.clone().execute(&vec![8]).unwrap(),
        IntCodeOutcome::Finished(vec![1000])
    );
    assert_eq!(
        comp.clone().execute(&vec![18]).unwrap(),
        IntCodeOutcome::Finished(vec![1001])
    );
    assert_eq!(
        comp.clone().execute(&vec![3]).unwrap(),
        IntCodeOutcome::Finished(vec![999])
    );
}

#[test]
fn test_execute_relative() {
    let prog = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
    let mut comp = IntCodeComputer::parse_program(prog).unwrap();
    assert_eq!(
        comp.execute(&vec![]).unwrap(),
        IntCodeOutcome::Finished(vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99
        ])
    );
}

#[test]
fn test_execute_big() {
    let prog = "1102,34915192,34915192,7,4,7,99,0";
    let mut comp = IntCodeComputer::parse_program(prog).unwrap();
    assert_eq!(
        comp.execute(&vec![]).unwrap(),
        IntCodeOutcome::Finished(vec![1219070632396864])
    );
    let prog = "104,1125899906842624,99";
    let mut comp = IntCodeComputer::parse_program(prog).unwrap();
    assert_eq!(
        comp.execute(&vec![]).unwrap(),
        IntCodeOutcome::Finished(vec![1125899906842624])
    );
}
fn parse_op(op: i64) -> (Vec<IntCodeMode>, IntCodeInstruction) {
    let op_str = op.to_string();
    let flags = op_str
        .chars()
        .rev()
        .chain(repeat('0'))
        .skip(2)
        .take(3)
        .map(|x| x.to_digit(10).unwrap())
        .map(|x| IntCodeMode::try_from(x).unwrap())
        .collect::<Vec<IntCodeMode>>();
    let opcode = if op_str.len() == 1 {
        IntCodeInstruction::try_from(op_str.parse::<i32>().unwrap()).unwrap()
    } else {
        IntCodeInstruction::try_from(op_str[op_str.len() - 2..].parse::<i32>().unwrap()).unwrap()
    };
    (flags, opcode)
}

fn resolve_flags(comp: &IntCodeComputer, i: usize, argno: i64, flags: &Vec<IntCodeMode>) -> i64 {
    use IntCodeMode::*;
    let prog = &comp.state;
    let argno = argno as usize;
    match flags[argno] {
        Position => prog[prog[i + argno + 1] as usize],
        Immediate => prog[i + argno + 1],
        Relative => prog[(prog[i + argno + 1] + comp.relative_base) as usize],
    }
}

#[derive(Debug, TryFromPrimitive, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
enum IntCodeMode {
    Position = 0,
    Immediate = 1,
    Relative = 2,
}

#[derive(Debug)]
pub enum IntCodeError {
    CannotLoadProgram,
    CannotParseProgram,
    BadOpcode(IntCodeInstruction),
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

#[derive(Debug, TryFromPrimitive, Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum IntCodeInstruction {
    Add = 1,
    Multiply,
    Input,
    Output,
    JumpNotEqualZero,
    JumpEqualZero,
    Less,
    Equal,
    ChangeRelativeBase,
    Halt = 99,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct IntCodeComputer {
    pub state: Vec<i64>,
    pub pc: usize,
    pub relative_base: i64,
    pub input_buffer: Vec<i64>, 
    pub input_buffer_idx: usize,
}

impl IntCodeComputer {
    pub fn parse_program(contents: &str) -> Result<Self, IntCodeError> {
        let mut comp = Self {
            state: contents
                .split(",")
                .map(|el| el.parse::<i64>())
                .collect::<Result<Vec<i64>, _>>()?,
            pc: 0,
            relative_base: 0,
            input_buffer: vec![],
            input_buffer_idx: 0,
        };
        comp.state.append(&mut vec![0; INTCODE_MEM_SIZE]);
        Ok(comp)
    }
    pub fn execute(&mut self, input: &Vec<i64>) -> Result<IntCodeOutcome, IntCodeError> {
        use IntCodeInstruction::*;
        use IntCodeOutcome::*;
        self.input_buffer = input.clone();
        self.input_buffer_idx = 0;
        let mut output = vec![];
        loop {
            let op = self.state[self.pc];
            let (flags, opcode) = parse_op(op);

            if opcode == Halt {
                return Ok(Finished(output));
            };
            match opcode {
                Add => {
                    let arg0 = resolve_flags(&self, self.pc, 0, &flags);
                    let arg1 = resolve_flags(&self, self.pc, 1, &flags);
                    let offset = if flags[2] == IntCodeMode::Relative {
                        self.relative_base
                    } else {
                        0
                    };
                    let arg2 = (self.state[self.pc + 3] + offset) as usize;
                    self.state[arg2] = arg0 + arg1;
                    self.pc += 4
                }
                Multiply => {
                    let arg0 = resolve_flags(&self, self.pc, 0, &flags);
                    let arg1 = resolve_flags(&self, self.pc, 1, &flags);
                    let offset = if flags[2] == IntCodeMode::Relative {
                        self.relative_base
                    } else {
                        0
                    };
                    let arg2 = (self.state[self.pc + 3] + offset) as usize;
                    self.state[arg2] = arg0 * arg1;
                    self.pc += 4
                }
                Input => {
                    let offset = if flags[0] == IntCodeMode::Relative {
                        self.relative_base
                    } else {
                        0
                    };
                    let arg0 = (self.state[self.pc + 1] + offset) as usize;
                    self.state[arg0] = if self.input_buffer_idx == self.input_buffer.len() {
                        return Ok(NeedInput(output));
                    } else {
                        self.pc += 2;
                        let x = self.input_buffer[self.input_buffer_idx];
                        self.input_buffer_idx += 1;
                        x
                    }
                }
                Output => {
                    let arg0 = resolve_flags(&self, self.pc, 0, &flags);
                    output.push(arg0 as i64);
                    self.pc += 2
                }
                JumpNotEqualZero => {
                    let arg0 = resolve_flags(&self, self.pc, 0, &flags);
                    let arg1 = resolve_flags(&self, self.pc, 1, &flags);
                    if arg0 != 0 {
                        self.pc = arg1 as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                JumpEqualZero => {
                    let arg0 = resolve_flags(&self, self.pc, 0, &flags);
                    let arg1 = resolve_flags(&self, self.pc, 1, &flags);
                    if arg0 == 0 {
                        self.pc = arg1 as usize;
                    } else {
                        self.pc += 3;
                    }
                }

                Less => {
                    let arg0 = resolve_flags(&self, self.pc, 0, &flags);
                    let arg1 = resolve_flags(&self, self.pc, 1, &flags);
                    let offset = if flags[2] == IntCodeMode::Relative {
                        self.relative_base
                    } else {
                        0
                    };
                    let arg2 = (self.state[self.pc + 3] + offset) as usize;
                    if arg0 < arg1 {
                        self.state[arg2] = 1;
                    } else {
                        self.state[arg2] = 0;
                    }
                    self.pc += 4
                }
                Equal => {
                    let arg0 = resolve_flags(&self, self.pc, 0, &flags);
                    let arg1 = resolve_flags(&self, self.pc, 1, &flags);
                    let offset = if flags[2] == IntCodeMode::Relative {
                        self.relative_base
                    } else {
                        0
                    };
                    let arg2 = (self.state[self.pc + 3] + offset) as usize;
                    if arg0 == arg1 {
                        self.state[arg2] = 1;
                    } else {
                        self.state[arg2] = 0;
                    }
                    self.pc += 4
                }
                ChangeRelativeBase => {
                    let arg0 = resolve_flags(&self, self.pc, 0, &flags);
                    self.relative_base += arg0;
                    self.pc += 2;
                }

                e @ _ => return Err(IntCodeError::BadOpcode(e)),
            }
        }
    }
}
