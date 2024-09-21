mod ast;
mod error;
mod evaluate;
mod frame;
mod scope;

use ast::set_program_file_path;
use evaluate::evaluate_program;
use std::error::Error;
use std::fs::File;
use std::io::{stdout, Read};
use std::path::PathBuf;
use std::process::ExitCode;

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub macro_parser);

fn main() -> Result<ExitCode, Box<dyn Error>> {
    let mut file = File::open("test.bfm")?;
    let mut script = String::new();
    file.read_to_string(&mut script)?;

    let parser = macro_parser::ProgramParser::new();
    let mut program = parser.parse(&script).unwrap();
    set_program_file_path(&mut program, &PathBuf::from("test.bfm"));

    if evaluate_program(&mut stdout(), &program)? {
        Ok(ExitCode::FAILURE)
    } else {
        Ok(ExitCode::SUCCESS)
    }
}
