use compiler::evaluate_file;
use std::env::args;
use std::process::ExitCode;

mod compiler;

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
