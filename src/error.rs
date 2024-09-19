use std::fs::File;
use std::io::Read;

use crate::ast::{Span, Variable};

fn read_source(file_path: &str) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut source = String::new();
    file.read_to_string(&mut source)?;
    Ok(source)
}

pub fn display_error_message(file_path: &Option<String>, span: Span, error_message: String) {
    if file_path.is_none() {
        println!("Error in unknown location: {error_message}");
        return;
    }

    let file_path = file_path.as_ref().unwrap();
    let source = read_source(&file_path);
    if source.is_err() {
        println!("Error in invalid file '{file_path}': {error_message}");
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

        if i >= span.0 {
            is_in_line = true;
        }
    }

    let line = &source[line_start..line_end];
    println!("\n{file_path}:{line_count} {line}");
    println!("Error: {error_message}");
}

pub fn variable_span(variable: &Variable) -> Span {
    assert!(!variable.is_empty());

    let (start, _) = variable.first().unwrap().span;
    let (_, end) = variable.last().unwrap().span;
    (start, end)
}

pub fn variable_string(variable: &Variable) -> String {
    variable
        .iter()
        .map(|x| x.value.clone())
        .collect::<Vec<String>>()
        .join(".")
}
