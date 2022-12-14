use std::{char, error::Error};
use thiserror::Error;
use Instruction::*;
use MachineError::*;

pub fn run(input: String) -> Result<(), Box<dyn Error>> {
    let instructions = parse(&input[..]);
    let mut machine = Machine::new(instructions);

    machine.run()?;
    Ok(())
}

#[derive(Debug)]
enum Instruction {
    NextPtr,
    PrevPtr,
    Increment,
    Decrement,
    Print,
    Read,
    Begin,
    End,
}

fn parse(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    let mut i = 0;

    while i < input.chars().count() {
        if substring(input, i, 1) == "う" {
            instructions.push(NextPtr);
            i += 1;
        } else if substring(input, i, 2) == "おう" {
            instructions.push(Read);
            i += 2;
        } else if substring(input, i, 4) == "おおうう" {
            instructions.push(Begin);
            i += 4;
        } else if substring(input, i, 4) == "おおうお" {
            instructions.push(End);
            i += 4;
        } else if substring(input, i, 4) == "おおおう" {
            instructions.push(PrevPtr);
            i += 4;
        } else if substring(input, i, 5) == "おおおおう" {
            instructions.push(Print);
            i += 5;
        } else if substring(input, i, 6) == "おおおおおう" {
            instructions.push(Decrement);
            i += 6;
        } else if substring(input, i, 6) == "おおおおおお" {
            instructions.push(Increment);
            i += 6;
        } else {
            i += 1;
        }
    }

    instructions
}

fn substring(input: &str, begin: usize, len: usize) -> String {
    input.chars().skip(begin).take(len).collect::<String>()
}

#[derive(Debug)]
struct Machine {
    instructions: Vec<Instruction>,
    pc: usize,
    current: usize,
    memory: [u8; 256],
    stack: Vec<usize>,
}

impl Machine {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            pc: 0,
            current: 0,
            memory: [0; 256],
            stack: Vec::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), MachineError> {
        while let Some(instruction) = self.instructions.get(self.pc) {
            match instruction {
                NextPtr => {
                    self.next_ptr()?;
                    self.pc += 1;
                }
                PrevPtr => {
                    self.prev_ptr()?;
                    self.pc += 1;
                }
                Increment => {
                    self.increment();
                    self.pc += 1;
                }
                Decrement => {
                    self.decrement();
                    self.pc += 1;
                }
                Print => match char::from_u32(self.current_value() as u32) {
                    Some(c) => {
                        print!("{}", c);
                        self.pc += 1;
                    }
                    None => return Err(InvalidChar(self.current_value())),
                },
                Read => {
                    if let Some(x) = self.memory.get_mut(self.current) {
                        *x = getchar();
                        self.pc += 1;
                    }
                }
                Begin => {
                    if self.current_value() == 0 {
                        self.jump_to_end()?;
                    } else {
                        self.stack.push(self.pc);
                        self.pc += 1;
                    }
                }
                End => match self.stack.pop() {
                    Some(address) => {
                        self.pc = address;
                    }
                    None => return Err(UnmatchedBeginEnd),
                },
            }
        }

        Ok(())
    }

    fn next_ptr(&mut self) -> Result<(), MachineError> {
        self.current += 1;
        if self.current > 255 {
            Err(PointerAccessViolation(self.current as isize))
        } else {
            Ok(())
        }
    }

    fn prev_ptr(&mut self) -> Result<(), MachineError> {
        if self.current == 0 {
            Err(PointerAccessViolation(-1))
        } else {
            self.current -= 1;
            Ok(())
        }
    }

    fn increment(&mut self) {
        if let Some(x) = self.memory.get_mut(self.current) {
            *x = x.wrapping_add(1);
        }
    }

    fn decrement(&mut self) {
        if let Some(x) = self.memory.get_mut(self.current) {
            *x = x.wrapping_sub(1);
        }
    }

    fn current_value(&self) -> u8 {
        *self.memory.get(self.current).unwrap()
    }

    fn jump_to_end(&mut self) -> Result<(), MachineError> {
        let mut counter = 1;

        while counter != 0 {
            self.pc += 1;
            match self.instructions.get(self.pc) {
                Some(instruction) => match instruction {
                    Begin => {
                        counter += 1;
                    }
                    End => {
                        counter -= 1;
                    }
                    _ => {}
                },
                None => {
                    return Err(UnmatchedBeginEnd);
                }
            }
        }

        self.pc += 1;
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum MachineError {
    #[error("Invalid char code {0}")]
    InvalidChar(u8),
    #[error("Pointer access violation. Pointer must be more than or equal 0 and less than 256, but it is {0}")]
    PointerAccessViolation(isize),
    #[error("Unmached Begin and end.")]
    UnmatchedBeginEnd,
}

fn getchar() -> u8 {
    unsafe { libc::getchar() as u8 }
}
