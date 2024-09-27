use crate::bf::BF;
use ast::set_program_file_path;
use evaluate::evaluate_program;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub macro_parser);

mod ast;
mod error;
mod evaluate;
mod frame;
mod scope;

pub fn evaluate_file(file_path: &str) -> std::io::Result<Option<BF>> {
    let mut file = File::open(file_path)?;
    let mut script = String::new();
    file.read_to_string(&mut script)?;

    let parser = macro_parser::ProgramParser::new();
    let mut program = parser.parse(&script).unwrap();
    set_program_file_path(&mut program, &PathBuf::from("test.bfm"));

    let (bf, did_error) = evaluate_program(&program)?;
    if did_error {
        Ok(None)
    } else {
        Ok(Some(bf))
    }
}
