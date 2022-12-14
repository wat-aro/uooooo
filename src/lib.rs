use std::char;
use thiserror::Error;
use Instruction::*;
use MachineError::*;

pub fn run(input: String) {
    let instructions = parse(&input[..]);
    let mut machine = Machine::new(instructions);

    machine.run().unwrap();
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

    for c in input.chars() {
        match c {
            '>' => instructions.push(NextPtr),
            '<' => instructions.push(PrevPtr),
            '+' => instructions.push(Increment),
            '-' => instructions.push(Decrement),
            '.' => instructions.push(Print),
            ',' => instructions.push(Read),
            '[' => instructions.push(Begin),
            ']' => instructions.push(End),
            _ => {}
        }
    }

    instructions
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
                    self.stack.push(self.pc);
                    self.pc += 1;
                }
                End => match self.stack.pop() {
                    Some(address) => {
                        if self.current_value() != 0 {
                            self.pc = address;
                        } else {
                            self.pc += 1;
                        }
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
            *x += 1;
        }
    }

    fn decrement(&mut self) {
        if let Some(x) = self.memory.get_mut(self.current) {
            *x -= 1;
        }
    }

    fn current_value(&self) -> u8 {
        *self.memory.get(self.current).unwrap()
    }
}

#[derive(Error, Debug)]
enum MachineError {
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
