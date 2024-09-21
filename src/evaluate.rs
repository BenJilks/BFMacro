use crate::ast::{Block, Definition, Instruction, Program, Using};
use crate::error::{display_error_message, variable_span, Error};
use crate::frame::{Frame, Lookup};
use crate::scope::Scope;
use std::io::Write;

fn evaluate_moving_block(
    output: &mut impl Write,
    block: &Block,
    scope: &Scope,
) -> std::io::Result<bool> {
    let mut did_error = false;

    for instruction in &block.instructions {
        match instruction {
            Instruction::Add => write!(output, "+")?,
            Instruction::Subtract => write!(output, "-")?,
            Instruction::Left(_) => write!(output, "<")?,
            Instruction::Right(_) => write!(output, ">")?,
            Instruction::Input => write!(output, ",")?,
            Instruction::Output => write!(output, ".")?,
            Instruction::OpenLoop => write!(output, "[")?,
            Instruction::CloseLoop => write!(output, "]")?,

            Instruction::MovingBlock(block) => {
                did_error |= evaluate_moving_block(output, block, scope)?;
            }

            Instruction::Using(using) => {
                let (using_did_error, _) = evaluate_using(output, using, scope)?;
                did_error |= using_did_error;
            }

            Instruction::Variable(variable) => {
                did_error = true;
                display_error_message(
                    &block.file_path,
                    Error {
                        span: variable_span(variable),
                        message: "Cannot access variables from inside a moving block".to_owned(),
                    },
                );
            }

            Instruction::MacroInvoke(name, _) => {
                did_error = true;
                display_error_message(
                    &block.file_path,
                    Error {
                        span: name.span,
                        message: "Cannot access macros from inside a moving block".to_owned(),
                    },
                );
            }
        }
    }

    Ok(did_error)
}

fn evaluate(
    output: &mut impl Write,
    frame: &Frame,
    frame_offset: usize,
    block: &Block,
    scope: &Scope,
) -> std::io::Result<(bool, usize)> {
    let mut frame_offset = frame_offset;
    let mut did_error = false;

    for instruction in &block.instructions {
        match instruction {
            Instruction::Add => write!(output, "+")?,
            Instruction::Subtract => write!(output, "-")?,
            Instruction::Input => write!(output, ",")?,
            Instruction::Output => write!(output, ".")?,
            Instruction::OpenLoop => write!(output, "[")?,
            Instruction::CloseLoop => write!(output, "]")?,

            Instruction::Left(span) | Instruction::Right(span) => {
                did_error = true;
                display_error_message(
                    &block.file_path,
                    Error {
                        span: span.clone(),
                        message: "Can only use manual pointer movement inside a moving block"
                            .to_owned(),
                    },
                );
            }

            Instruction::MovingBlock(block) => {
                evaluate_moving_block(output, block, scope)?;
            }

            Instruction::Using(using) => {
                let (using_did_error, using_frame_offset) = evaluate_using(output, using, scope)?;
                frame_offset += using_frame_offset;
                did_error |= using_did_error;
            }

            Instruction::Variable(variable) => {
                match frame.lookup(&variable) {
                    Ok(Lookup::Slot(offset)) => {
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

                    Ok(Lookup::Block(block, frame)) => {
                        let (block_did_error, block_frame_offset) =
                            evaluate(output, &frame, frame_offset, &block, scope)?;
                        frame_offset = block_frame_offset;
                        did_error |= block_did_error;
                    }

                    Err(err) => {
                        did_error = true;
                        display_error_message(&block.file_path, err);
                    }
                };
            }

            Instruction::MacroInvoke(name, arguments) => {
                let macro_ = scope.macro_(&name.value);
                if macro_.is_none() {
                    did_error = true;
                    display_error_message(
                        &block.file_path,
                        Error {
                            span: name.span,
                            message: format!("Error: No macro '{}' found", name.value),
                        },
                    );
                    continue;
                }

                let macro_ = macro_.unwrap();
                match frame.macro_frame(&macro_.parameters, &arguments) {
                    Ok(frame) => {
                        #[cfg(feature = "comments")]
                        writeln!(output, "\n\n# {}", name.value)?;
                        let (macro_did_error, macro_frame_offset) =
                            evaluate(output, &frame, frame_offset, &macro_.block, scope)?;
                        frame_offset = macro_frame_offset;
                        did_error |= macro_did_error;
                    }

                    Err(err) => {
                        did_error = true;
                        display_error_message(&block.file_path, err);
                    }
                }
            }
        }
    }

    Ok((did_error, frame_offset))
}

fn evaluate_using(
    output: &mut impl Write,
    using: &Using,
    scope: &Scope,
) -> std::io::Result<(bool, usize)> {
    let frame_definition = scope.frame_definition(&using.frame.value);
    if frame_definition.is_none() {
        display_error_message(
            &using.block.file_path,
            Error {
                span: using.frame.span,
                message: format!("Error: No frame '{}' found", using.frame.value),
            },
        );
        return Ok((true, 0));
    }

    #[cfg(feature = "comments")]
    writeln!(output, "\n# Using frame '{}'", using.frame.value)?;

    let frame = Frame::from_definition(frame_definition.unwrap(), scope);
    evaluate(output, &frame, 0, &using.block, scope)
}

pub fn evaluate_program(output: &mut impl Write, program: &Program) -> std::io::Result<bool> {
    let mut did_error = false;

    let scope = Scope::new(program, &std::env::current_dir()?)?;
    for definition in program {
        if let Definition::Using(using) = definition {
            let (using_did_error, _) = evaluate_using(output, using, &scope)?;
            did_error |= using_did_error;
        }
    }

    writeln!(output)?;
    Ok(did_error)
}
