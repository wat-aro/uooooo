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
    Increment,
    Print,
}

fn parse(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    for c in input.chars() {
        match c {
            '+' => instructions.push(Increment),
            '.' => instructions.push(Print),
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
}

impl Machine {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            pc: 0,
            current: 0,
            memory: [0; 256],
        }
    }

    pub fn run(&mut self) -> Result<(), MachineError> {
        while let Some(instruction) = self.instructions.get(self.pc) {
            match instruction {
                Increment => {
                    self.increment();
                }
                Print => match char::from_u32(self.current_value() as u32) {
                    Some(c) => print!("{}", c),
                    None => return Err(InvalidChar(self.current_value())),
                },
            }
            self.pc += 1;
        }

        Ok(())
    }

    fn increment(&mut self) {
        if let Some(x) = self.memory.get_mut(self.current) {
            *x += 1;
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
}
