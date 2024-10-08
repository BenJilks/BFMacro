use crate::bf::{Instruction, BF};
use std::num::Wrapping;
use std::io::Read;

fn print_memory(memory: &[Wrapping<u8>], _pointer: usize) {
    for cell in memory {
        print!("{cell:03} ");
    }
    println!();
    println!();
}

pub fn run_program(program: BF) {
    let mut memory = vec![Wrapping(0u8); 1];
    let mut pointer = 0usize;
    let mut pc = 0usize;
    let mut stack = Vec::<usize>::new();
    let mut input_pointer = 0usize;

    let code = program.code();
    while pc < program.len() {
        let instruction = code[pc];
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

            Instruction::Input => {
                if input_pointer >= program.input.len() {
                    memory[pointer] = Wrapping(0);
                } else {
                    memory[pointer] = Wrapping(program.input[input_pointer]);
                    input_pointer += 1;
                }
            }
            Instruction::Output => print!("{}", memory[pointer].0 as char),

            Instruction::OpenLoop => {
                if memory[pointer].0 == 0 {
                    let mut depth = 1;
                    while pc < program.len() && depth > 0 {
                        pc += 1;
                        match code[pc] {
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

            Instruction::Break => {
                print_memory(&memory, pointer);
                std::io::stdin().read(&mut [0u8]).unwrap();
            }
        }
    }
}
