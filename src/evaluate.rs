use crate::ast::{Definition, Instruction, Program, Using};
use crate::frame::Frame;
use crate::scope::Scope;
use std::io::Write;
use std::process::exit;

fn evaluate(
    output: &mut impl Write,
    frame: &Frame,
    frame_offset: usize,
    program: &Vec<Instruction>,
    scope: &Scope,
) -> std::io::Result<()> {
    let mut frame_offset = frame_offset;

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

            Instruction::Variable(name) => {
                let offset = frame.offset(name).unwrap_or_else(|| {
                    println!("Error: No variable '{name}' in frame '{}'", frame.name);
                    exit(1);
                });

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

            Instruction::MacroInvoke(name, arguments) => {
                let marco_ = scope.macro_(name).unwrap_or_else(|| {
                    println!("No macro '{name}' found");
                    exit(1);
                });

                let frame = frame.macro_frame(&marco_.parameters, &arguments);
                evaluate(output, &frame, frame_offset, &marco_.block, scope)?;
            }
        }
    }

    Ok(())
}

fn evaluate_using(output: &mut impl Write, using: &Using, scope: &Scope) -> std::io::Result<()> {
    let frame_definition = scope.frame_definition(&using.frame).unwrap_or_else(|| {
        println!("Error: No frame '{}' found", using.frame);
        exit(1);
    });

    let frame = Frame::from_definition(frame_definition, scope);
    evaluate(output, &frame, 0, &using.block, scope)
}

pub fn evaluate_program(output: &mut impl Write, program: &Program) -> std::io::Result<()> {
    let scope = Scope::new(program);
    for definition in program {
        if let Definition::Using(using) = definition {
            evaluate_using(output, &using, &scope)?;
        }
    }

    Ok(())
}
