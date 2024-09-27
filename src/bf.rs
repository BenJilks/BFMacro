use std::io::{Read, Write};

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

pub struct BF {
    code: Vec<Instruction>,
}

impl BF {
    pub fn new() -> Self {
        Self { code: Vec::new() }
    }

    pub fn parse(input: impl Read) -> std::io::Result<Self> {
        let mut instructions = Vec::new();
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
                instructions.push(instruction);
            }
        }

        Ok(Self { code: instructions })
    }

    pub fn push(&mut self, instruction: Instruction) {
        self.code.push(instruction);
    }

    pub fn code(&self) -> &[Instruction] {
        &self.code
    }

    pub fn len(&self) -> usize {
        self.code.len()
    }

    pub fn write(&self, mut output: impl Write) -> std::io::Result<()> {
        let mut column = 0;
        for instruction in &self.code {
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
}
