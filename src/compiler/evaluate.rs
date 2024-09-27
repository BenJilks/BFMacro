use super::ast::{Block, Definition, Instruction, Program, Using};
use super::error::{display_error_message, variable_span, Error};
use super::frame::{Frame, Lookup};
use super::scope::Scope;
use crate::bf::{self, BF};

fn evaluate_moving_block(output: &mut BF, block: &Block, scope: &Scope) -> bool {
    let mut did_error = false;
    let mut loop_depth = 0;

    for instruction in &block.instructions {
        match instruction {
            Instruction::Add => output.push(bf::Instruction::Add),
            Instruction::Subtract => output.push(bf::Instruction::Subtract),
            Instruction::Left(_) => output.push(bf::Instruction::Left),
            Instruction::Right(_) => output.push(bf::Instruction::Right),
            Instruction::Input => output.push(bf::Instruction::Input),
            Instruction::Output => output.push(bf::Instruction::Output),

            Instruction::OpenLoop => {
                loop_depth += 1;
                output.push(bf::Instruction::OpenLoop);
            }

            Instruction::CloseLoop(span) => {
                if loop_depth == 0 {
                    display_error_message(
                        &block.file_path,
                        Error {
                            span: *span,
                            message: "Too many closing brackets".to_owned(),
                        },
                    );
                } else {
                    loop_depth -= 1;
                }

                output.push(bf::Instruction::CloseLoop);
            }

            Instruction::MovingBlock(block) => {
                did_error |= evaluate_moving_block(output, block, scope);
            }

            Instruction::Using(using) => {
                let (using_did_error, _) = evaluate_using(output, using, scope);
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

    if loop_depth > 0 {
        display_error_message(
            &block.file_path,
            Error {
                span: block.span,
                message: "Too many open brackets".to_owned(),
            },
        );
    }

    did_error
}

fn evaluate(
    output: &mut BF,
    frame: &Frame,
    frame_offset: usize,
    block: &Block,
    scope: &Scope,
) -> (bool, usize) {
    let mut frame_offset = frame_offset;
    let mut did_error = false;
    let mut loop_stack = Vec::<usize>::new();

    for instruction in &block.instructions {
        match instruction {
            Instruction::Add => output.push(bf::Instruction::Add),
            Instruction::Subtract => output.push(bf::Instruction::Subtract),
            Instruction::Input => output.push(bf::Instruction::Input),
            Instruction::Output => output.push(bf::Instruction::Output),

            Instruction::OpenLoop => {
                loop_stack.push(frame_offset);
                output.push(bf::Instruction::OpenLoop);
            }

            Instruction::CloseLoop(span) => {
                match loop_stack.pop() {
                    Some(loop_start_offset) => {
                        if loop_start_offset != frame_offset {
                            did_error = true;
                            display_error_message(
                                &block.file_path,
                                Error {
                                    span: *span,
                                    message:
                                        "Must exit a loop at the same pointer that you entered"
                                            .to_owned(),
                                },
                            );
                        }
                    }

                    None => {
                        did_error = true;
                        display_error_message(
                            &block.file_path,
                            Error {
                                span: *span,
                                message: "Too many closing brackets".to_owned(),
                            },
                        );
                    }
                }

                output.push(bf::Instruction::CloseLoop);
            }

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
                evaluate_moving_block(output, block, scope);
            }

            Instruction::Using(using) => {
                let (using_did_error, using_frame_offset) = evaluate_using(output, using, scope);
                frame_offset += using_frame_offset;
                did_error |= using_did_error;
            }

            Instruction::Variable(variable) => {
                match frame.lookup(&variable) {
                    Ok(Lookup::Slot(offset)) => {
                        if offset > frame_offset {
                            for _ in frame_offset..offset {
                                output.push(bf::Instruction::Left);
                            }
                        } else if offset < frame_offset {
                            for _ in offset..frame_offset {
                                output.push(bf::Instruction::Right);
                            }
                        }

                        frame_offset = offset;
                    }

                    Ok(Lookup::Block(block, frame)) => {
                        let (block_did_error, block_frame_offset) =
                            evaluate(output, &frame, frame_offset, &block, scope);
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
                match frame.macro_frame(&macro_.name, &macro_.parameters, &arguments) {
                    Ok(frame) => {
                        let (macro_did_error, macro_frame_offset) =
                            evaluate(output, &frame, frame_offset, &macro_.block, scope);
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

    if !loop_stack.is_empty() {
        display_error_message(
            &block.file_path,
            Error {
                span: block.span,
                message: "Too many open brackets".to_owned(),
            },
        );
    }

    (did_error, frame_offset)
}

fn evaluate_using(output: &mut BF, using: &Using, scope: &Scope) -> (bool, usize) {
    let frame_definition = scope.frame_definition(&using.frame.value);
    if frame_definition.is_none() {
        display_error_message(
            &using.block.file_path,
            Error {
                span: using.frame.span,
                message: format!("Error: No frame '{}' found", using.frame.value),
            },
        );
        return (true, 0);
    }

    let frame = Frame::from_definition(frame_definition.unwrap(), scope);
    evaluate(output, &frame, 0, &using.block, scope)
}

pub fn evaluate_program(program: &Program) -> std::io::Result<(BF, bool)> {
    let mut output = BF::new();
    let mut did_error = false;

    let scope = Scope::new(program, &std::env::current_dir()?)?;
    for definition in program {
        if let Definition::Using(using) = definition {
            let (using_did_error, _) = evaluate_using(&mut output, using, &scope);
            did_error |= using_did_error;
        }
    }

    Ok((output, did_error))
}
