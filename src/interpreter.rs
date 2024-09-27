use crate::bf::{Instruction, BF};
use std::num::Wrapping;

pub fn run_program(program: BF) {
    let mut memory = vec![Wrapping(0u8); 1];
    let mut pointer = 0usize;
    let mut pc = 0usize;
    let mut stack = Vec::<usize>::new();

    while pc < program.len() {
        let instruction = program[pc];
        pc += 1;

        match instruction {
            Instruction::Add => memory[pointer] += 1,
            Instruction::Subtract => memory[pointer] -= 1,

            Instruction::Left => {
                if pointer == 0 {
                    println!("pointer={pointer} pc={pc}");
                    unreachable!();
                }
                pointer -= 1;
            }
            Instruction::Right => {
                pointer += 1;
                if pointer >= memory.len() {
                    memory.push(Wrapping(0));
                }
            }

            Instruction::Input => {}
            Instruction::Output => print!("{}", memory[pointer].0 as char),

            Instruction::OpenLoop => {
                if memory[pointer].0 == 0 {
                    let mut depth = 1;
                    while pc < program.len() && depth > 0 {
                        pc += 1;
                        match program[pc] {
                            Instruction::OpenLoop => depth += 1,
                            Instruction::CloseLoop => depth -= 1,
                            _ => {}
                        }
                    }
                    pc += 1;
                } else {
                    stack.push(pc - 1);
                }
            }
            Instruction::CloseLoop => pc = stack.pop().unwrap(),
        }
    }
}
