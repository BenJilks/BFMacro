use crate::ast::{Block, Definition, Instruction, Program, Using};
use crate::error::{display_error_message, variable_span, variable_string};
use crate::frame::{Frame, Lookup};
use crate::scope::Scope;
use std::io::Write;

fn evaluate_moving_block(
    output: &mut impl Write,
    block: &Block,
    scope: &Scope,
) -> std::io::Result<()> {
    for instruction in &block.instructions {
        match instruction {
            Instruction::Add => write!(output, "+")?,
            Instruction::Subtract => write!(output, "-")?,
            Instruction::Left => write!(output, "<")?,
            Instruction::Right => write!(output, ">")?,
            Instruction::Input => write!(output, ",")?,
            Instruction::Output => write!(output, ".")?,
            Instruction::OpenLoop => write!(output, "[")?,
            Instruction::CloseLoop => write!(output, "]")?,

            Instruction::MovingBlock(block) => {
                evaluate_moving_block(output, block, scope)?;
            }

            Instruction::Using(using) => {
                evaluate_using(output, using, scope)?;
            }

            Instruction::Variable(_) => {
                panic!("Cannot access variables from inside a moving block")
            }

            Instruction::MacroInvoke(_, _) => {
                panic!("Cannot access macros from inside a moving block")
            }
        }
    }

    Ok(())
}

fn evaluate(
    output: &mut impl Write,
    frame: &Frame,
    frame_offset: usize,
    block: &Block,
    scope: &Scope,
) -> std::io::Result<usize> {
    let mut frame_offset = frame_offset;

    for instruction in &block.instructions {
        match instruction {
            Instruction::Add => write!(output, "+")?,
            Instruction::Subtract => write!(output, "-")?,
            Instruction::Input => write!(output, ",")?,
            Instruction::Output => write!(output, ".")?,
            Instruction::OpenLoop => write!(output, "[")?,
            Instruction::CloseLoop => write!(output, "]")?,

            Instruction::Left | Instruction::Right => {
                panic!("Can only use manual pointer movement inside a moving block")
            }

            Instruction::MovingBlock(block) => {
                evaluate_moving_block(output, block, scope)?;
            }

            Instruction::Using(using) => {
                frame_offset += evaluate_using(output, using, scope)?;
            }

            Instruction::Variable(variable) => {
                match frame.lookup(&variable) {
                    Some(Lookup::Slot(offset)) => {
                        if offset > frame_offset {
                            for _ in frame_offset..offset {
                                write!(output, ">")?;
                            }
                        } else if offset < frame_offset {
                            for _ in offset..frame_offset {
                                write!(output, "<")?;
                            }
                        }

                        frame_offset = offset;
                    }

                    Some(Lookup::Block(block, frame)) => {
                        frame_offset = evaluate(output, &frame, frame_offset, &block, scope)?;
                    }

                    None => display_error_message(
                        &block.file_path,
                        variable_span(variable),
                        format!("Not symbol '{}' found", variable_string(variable)),
                    ),
                };
            }

            Instruction::MacroInvoke(name, arguments) => {
                let marco_ = scope.macro_(&name.value).unwrap_or_else(|| {
                    panic!("No macro '{}' found", name.value);
                });

                let frame = frame.macro_frame(&marco_.parameters, &arguments);
                frame_offset = evaluate(output, &frame, frame_offset, &marco_.block, scope)?;
            }
        }
    }

    Ok(frame_offset)
}

fn evaluate_using(output: &mut impl Write, using: &Using, scope: &Scope) -> std::io::Result<usize> {
    let frame_definition = scope
        .frame_definition(&using.frame.value)
        .unwrap_or_else(|| {
            panic!("Error: No frame '{}' found", using.frame.value);
        });

    let frame = Frame::from_definition(frame_definition, scope);
    evaluate(output, &frame, 0, &using.block, scope)
}

pub fn evaluate_program(output: &mut impl Write, program: &Program) -> std::io::Result<()> {
    let scope = Scope::new(program)?;
    for definition in program {
        if let Definition::Using(using) = definition {
            evaluate_using(output, using, &scope)?;
        }
    }

    Ok(())
}
