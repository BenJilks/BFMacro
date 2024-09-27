use compiler::evaluate_file;
use simplify::simplify_program;
use std::env::{args, Args};
use std::fs::File;
use std::io::stdout;
use std::process::ExitCode;

mod bf;
mod compiler;
mod simplify;

fn usage(executable: &str) {
    eprintln!("Usage: {executable} <action> <file>...");
    eprintln!();
    eprintln!("Actions:");
    eprintln!("   compile    Compile bfmacro files into bf");
    eprintln!("   format     Format an simplify bf files");
    eprintln!();
}

fn compile(executable: &str, args: Args) -> std::io::Result<ExitCode> {
    if args.len() == 0 {
        usage(executable);
        eprintln!("{executable}: error: no input files given");
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

fn format(executable: &str, args: Args) -> std::io::Result<ExitCode> {
    if args.len() == 0 {
        usage(executable);
        eprintln!("{executable}: error: no input files given");
        return Ok(ExitCode::FAILURE);
    }

    for file_path in args {
        let file = File::open(file_path)?;
        let program = bf::parse(file)?;
        let program = simplify_program(&program);
        bf::write(stdout(), &program)?;
    }

    Ok(ExitCode::SUCCESS)
}

fn main() -> std::io::Result<ExitCode> {
    let mut args = args();

    let executable = args.next().unwrap();
    if args.len() == 0 {
        usage(&executable);
        eprintln!("{executable}: error: no action given");
        return Ok(ExitCode::FAILURE);
    }

    let action = args.next().unwrap();
    match action.as_str() {
        "compile" => compile(&executable, args),
        "format" => format(&executable, args),
        _ => {
            usage(&executable);
            eprintln!("{executable}: error: unknown action '{action}'");
            Ok(ExitCode::FAILURE)
        }
    }
}
