use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use super::ast::{Argument, Span, Variable};

pub type Result<T> = std::result::Result<T, Error>;
pub struct Error {
    pub span: Span,
    pub message: String,
}

fn read_source(file_path: &Path) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut source = String::new();
    file.read_to_string(&mut source)?;
    Ok(source)
}

pub fn display_error_message(file_path: &Option<PathBuf>, error: Error) {
    if file_path.is_none() {
        eprintln!("Error in unknown location: {}", error.message);
        return;
    }

    let file_path = file_path.as_ref().unwrap();
    let file_path_string = file_path.to_string_lossy();

    let source = read_source(&file_path);
    if source.is_err() {
        eprintln!(
            "Error in invalid file '{file_path_string}': {}",
            error.message
        );
    }

    let source = source.unwrap();

    let mut line_count = 1;
    let mut line_start = 0;
    let mut line_end = 0;
    let mut is_in_line = false;
    for (i, char) in source.chars().enumerate() {
        if char == '\n' {
            if is_in_line {
                line_end = i;
                break;
            }
            line_start = i + 1;
            line_count += 1;
        }

        if i >= error.span.0 {
            is_in_line = true;
        }
    }

    let line = &source[line_start..line_end];
    eprintln!("\n{file_path_string}:{line_count} {line}");
    eprintln!("Error: {}", error.message);
}

pub fn variable_span(variable: &Variable) -> Span {
    assert!(!variable.is_empty());

    let (start, _) = variable.first().unwrap().span;
    let (_, end) = variable.last().unwrap().span;
    (start, end)
}

pub fn argument_span(argument: &Argument) -> Span {
    match argument {
        Argument::Variable(variable) => variable_span(variable),
        Argument::Block(block) => block.span,
    }
}

pub fn arguments_span(arguments: &[Argument]) -> Option<Span> {
    if arguments.is_empty() {
        return None;
    }

    let (start, _) = argument_span(arguments.first().unwrap());
    let (_, end) = argument_span(arguments.last().unwrap());
    Some((start, end))
}
