use std::io::{Read, Write};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Instruction {
    Add,
    Subtract,
    Left,
    Right,
    Input,
    Output,
    OpenLoop,
    CloseLoop,
    Break,
}

pub struct BF {
    code: Vec<Instruction>,
    pub input: Vec<u8>,
}

impl BF {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            input: Vec::new(),
        }
    }

    pub fn parse(stream: impl Read) -> std::io::Result<Self> {
        let mut code = Vec::new();
        let mut input = Vec::new();
        let mut is_input = false;

        for char in stream.bytes() {
            if is_input {
                input.push(char?);
                continue;
            }

            let instruction = match char? as char {
                '+' => Some(Instruction::Add),
                '-' => Some(Instruction::Subtract),
                '<' => Some(Instruction::Left),
                '>' => Some(Instruction::Right),
                ',' => Some(Instruction::Input),
                '.' => Some(Instruction::Output),
                '[' => Some(Instruction::OpenLoop),
                ']' => Some(Instruction::CloseLoop),
                '*' => Some(Instruction::Break),

                '!' => {
                    is_input = true;
                    None
                }
                _ => None,
            };

            if let Some(instruction) = instruction {
                code.push(instruction);
            }
        }

        Ok(Self { code, input })
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
                Instruction::Break => write!(output, "*")?,
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
