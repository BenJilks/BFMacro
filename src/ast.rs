use std::path::{Path, PathBuf};

pub type Program = Vec<Definition>;
pub type Variable = Vec<Identifier>;

#[derive(Debug, Clone)]
pub enum Definition {
    Include(String),
    Frame(FrameDefinition),
    Macro(Macro),
    Using(Using),
}

#[derive(Debug, Clone)]
pub struct FrameDefinition {
    pub name: Identifier,
    pub slots: Vec<SlotDefinition>,
}

#[derive(Debug, Clone)]
pub enum SlotDefinition {
    Variable(Identifier),
    SubFrame(Identifier, Identifier),
}

#[derive(Debug, Clone)]
pub struct Macro {
    pub name: Identifier,
    pub parameters: Vec<Parameter>,
    pub block: Block,
}

#[derive(Debug, Clone)]
pub enum Parameter {
    Slot(Identifier),
    SubFrame(Identifier, Identifier),
    Block(Identifier),
}

#[derive(Debug, Clone)]
pub struct Using {
    pub frame: Identifier,
    pub block: Block,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Add,
    Subtract,
    Left(Span),
    Right(Span),
    Input,
    Output,
    OpenLoop(Span),
    CloseLoop(Span),
    MovingBlock(Block),
    Using(Using),
    Variable(Variable),
    MacroInvoke(Identifier, Vec<Argument>),
}

#[derive(Debug, Clone)]
pub enum Argument {
    Variable(Variable),
    Block(Block),
}

#[derive(Debug, Clone)]
pub struct Block {
    pub instructions: Vec<Instruction>,
    pub span: Span,
    pub file_path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub value: String,
    pub span: Span,
}

pub type Span = (usize, usize);

fn set_block_file_path(block: &mut Block, file_path: &Path) {
    block.file_path = Some(file_path.to_owned());
    for instruction in &mut block.instructions {
        match instruction {
            Instruction::MovingBlock(block) => {
                set_block_file_path(block, file_path);
            }
            Instruction::Using(using) => {
                set_block_file_path(&mut using.block, file_path);
            }
            Instruction::MacroInvoke(_, arguments) => {
                for argument in arguments {
                    if let Argument::Block(block) = argument {
                        set_block_file_path(block, file_path);
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn set_program_file_path(program: &mut Program, file_path: &Path) {
    for definition in program {
        match definition {
            Definition::Macro(macro_) => {
                set_block_file_path(&mut macro_.block, file_path);
            }
            Definition::Using(using) => {
                set_block_file_path(&mut using.block, file_path);
            }
            _ => {}
        }
    }
}
