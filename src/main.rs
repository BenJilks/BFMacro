use ast::set_program_file_path;
use evaluate::evaluate_program;
use std::env::args;
use std::fs::File;
use std::io::{stdout, Read};
use std::path::PathBuf;
use std::process::ExitCode;

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub macro_parser);

mod ast;
mod error;
mod evaluate;
mod frame;
mod scope;

fn evaluate_file(file_path: &str) -> std::io::Result<bool> {
    let mut file = File::open(file_path)?;
    let mut script = String::new();
    file.read_to_string(&mut script)?;

    let parser = macro_parser::ProgramParser::new();
    let mut program = parser.parse(&script).unwrap();
    set_program_file_path(&mut program, &PathBuf::from("test.bfm"));

    evaluate_program(&mut stdout(), &program)
}

fn main() -> std::io::Result<ExitCode> {
    let mut args = args();

    let executable = args.next().unwrap();
    if args.len() == 0 {
        eprintln!("{}: error: no input files given", executable);
        return Ok(ExitCode::FAILURE);
    }

    let mut did_error = false;
    for file_path in args {
        did_error |= evaluate_file(&file_path)?;
    }

    if did_error {
        Ok(ExitCode::FAILURE)
    } else {
        Ok(ExitCode::SUCCESS)
    }
}
