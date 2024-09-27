use std::fs::File;
use std::io::{stdout, Read, Write};

#[derive(Copy, Clone)]
enum Instruction {
    Add,
    Subtract,
    Left,
    Right,
    Input,
    Output,
    OpenLoop,
    CloseLoop,
}

fn parse_script(script: &str) -> Vec<Instruction> {
    let mut program = Vec::new();
    for char in script.chars() {
        let instruction = match char {
            '+' => Some(Instruction::Add),
            '-' => Some(Instruction::Subtract),
            '<' => Some(Instruction::Left),
            '>' => Some(Instruction::Right),
            ',' => Some(Instruction::Input),
            '.' => Some(Instruction::Output),
            '[' => Some(Instruction::OpenLoop),
            ']' => Some(Instruction::CloseLoop),
            _ => None,
        };

        if let Some(instruction) = instruction {
            program.push(instruction);
        }
    }

    program
}

fn resolve_combined_move(out: &mut Vec<Instruction>, count: &mut i32) {
    for _ in 0..count.abs() {
        if *count > 0 {
            out.push(Instruction::Right);
        } else {
            out.push(Instruction::Left);
        }
    }

    *count = 0;
}

fn resolve_combined_change(out: &mut Vec<Instruction>, count: &mut i32) {
    for _ in 0..count.abs() {
        if *count > 0 {
            out.push(Instruction::Add);
        } else {
            out.push(Instruction::Subtract);
        }
    }

    *count = 0;
}

fn simplify_program(program: &[Instruction]) -> Vec<Instruction> {
    let mut simplified = Vec::new();

    let mut combined_move = 0i32;
    let mut combined_change = 0i32;
    for instruction in program {
        match instruction {
            Instruction::Left => {
                resolve_combined_change(&mut simplified, &mut combined_change);
                combined_move -= 1;
            }
            Instruction::Right => {
                resolve_combined_change(&mut simplified, &mut combined_change);
                combined_move += 1;
            }

            Instruction::Add => {
                resolve_combined_move(&mut simplified, &mut combined_move);
                combined_change += 1;
            }
            Instruction::Subtract => {
                resolve_combined_move(&mut simplified, &mut combined_move);
                combined_change -= 1;
            }

            instruction => {
                resolve_combined_move(&mut simplified, &mut combined_move);
                resolve_combined_change(&mut simplified, &mut combined_change);
                simplified.push(*instruction);
            }
        }
    }

    resolve_combined_move(&mut simplified, &mut combined_move);
    resolve_combined_change(&mut simplified, &mut combined_change);
    simplified
}

fn format_program(mut output: impl Write, program: &[Instruction]) -> std::io::Result<()> {
    let mut column = 0;
    for instruction in program {
        match instruction {
            Instruction::Add => write!(output, "+")?,
            Instruction::Subtract => write!(output, "-")?,
            Instruction::Left => write!(output, "<")?,
            Instruction::Right => write!(output, ">")?,
            Instruction::Input => write!(output, ",")?,
            Instruction::Output => write!(output, ".")?,
            Instruction::OpenLoop => write!(output, "[")?,
            Instruction::CloseLoop => write!(output, "]")?,
        }

        column += 1;
        if column >= 80 {
            write!(output, "\n")?;
            column = 0;
        }
    }

    write!(output, "\n")?;
    Ok(())
}

pub fn format_file(file_path: &str) -> std::io::Result<bool> {
    let mut file = File::open(file_path)?;
    let mut script = String::new();
    file.read_to_string(&mut script)?;

    let program = parse_script(&script);
    let program = simplify_program(&program);
    format_program(stdout(), &program)?;

    Ok(true)
}
