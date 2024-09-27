use std::io::{Read, Write};

pub type BF = Vec<Instruction>;

#[derive(Copy, Clone, PartialEq)]
pub enum Instruction {
    Add,
    Subtract,
    Left,
    Right,
    Input,
    Output,
    OpenLoop,
    CloseLoop,
}

pub fn parse(input: impl Read) -> std::io::Result<Vec<Instruction>> {
    let mut program = Vec::new();
    for char in input.bytes() {
        let instruction = match char? as char {
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

    Ok(program)
}

pub fn write(mut output: impl Write, program: &BF) -> std::io::Result<()> {
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
