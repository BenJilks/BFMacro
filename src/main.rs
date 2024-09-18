mod ast;
mod evaluate;
mod frame;
mod scope;

use evaluate::evaluate_program;
use std::error::Error;
use std::fs::File;
use std::io::{stdout, Read};

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub macro_parser);

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("test.bfm")?;
    let mut script = String::new();
    file.read_to_string(&mut script)?;

    let parser = macro_parser::ProgramParser::new();
    let program = parser.parse(&script).unwrap();
    evaluate_program(&mut stdout(), &program)?;
    println!();

    Ok(())
}
